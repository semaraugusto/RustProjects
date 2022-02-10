use clap::{App, Arg};
use std::process;
use std::time::Duration;
// use zk_benchmark::{run, Config};
//
mod assembly_sol;
mod build;

use zk_benchmark::BenchmarkerState;

#[tokio::main]
async fn main() {
    let matches = App::new("ZK Benchmark")
        .version("0.0.1")
        .author("Semar Augusto <semaraugusto@gmail.com>")
        .about("Allows for gas and proving time benchmarking for zk contracts")
        .arg(
            Arg::new("command")
                .short('c')
                .long("command")
                .takes_value(true)
                .help("Build/Run"),
        )
        .arg(
            Arg::new("abi")
                .long("abi")
                .takes_value(true)
                .help("Path to contract json abi"),
        )
        .arg(
            Arg::new("output_path")
                .short('o')
                .long("output-path")
                .takes_value(true)
                .help("Path to rust typesafe contract abi"),
        )
        .get_matches();

    let command = matches.value_of("command").unwrap_or("run");
    let config = build::Config::new(
        matches.value_of("abi").unwrap().to_string(),
        matches.value_of("output_path").unwrap().to_string(),
    )
    .unwrap();
    if command == "build" {
        if let Err(e) = build::create_typesafe_abi(&config) {
            eprintln!("Building error error: {}", e);
            process::exit(1);
        };
    } else {
        let pvtkey = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7";
        let provider_addr = "http://127.0.0.1:9545";
        let contract_addr = "0x1d412664e5B1c9518995Cf411e8C2F4CC929D5C2";
        let mut benchmarker = zk_benchmark::Benchmarker::new(pvtkey, provider_addr, contract_addr);

        while let BenchmarkerState::Running = &benchmarker.state {
            benchmarker.make_request().await;
            std::thread::sleep(Duration::from_secs(5));
        }
    }
}
