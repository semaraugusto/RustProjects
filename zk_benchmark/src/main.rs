use clap::{App, Arg};
use ethers::{prelude::*, providers};
use std::process;
use std::time::Duration;
// use zk_benchmark::{run, Config};
//
mod assembly_sol;

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
    let config = zk_benchmark::Config::new(
        matches.value_of("abi").unwrap().to_string(),
        matches.value_of("output_path").unwrap().to_string(),
    )
    .unwrap();
    if command == "build" {
        if let Err(e) = zk_benchmark::create_typesafe_abi(&config) {
            eprintln!("Building error error: {}", e);
            process::exit(1);
        };
    } else {
        while zk_benchmark::run().await {
            std::thread::sleep_ms(5000);
        }
    }
}
