use ethers::{prelude::*, providers};
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use std::{env, process};
// use zk_benchmark::{run, Config};
//
mod assembly_sol;

type HttpProvider = providers::Provider<providers::Http>;
#[tokio::main]
async fn main() {
    let pvtkey = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7"
        .parse::<LocalWallet>()
        .unwrap();

    let pubkey = pvtkey.address();
    println!("pubkey: {}", pubkey.to_string());

    let config = zk_benchmark::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let abi = zk_benchmark::get_abi(&config).map_err(|e| -> Result<String, Box<dyn Error>> {
        eprintln!("Application error: {}", e);
        process::exit(1);
    });

    // let dest_chain_id = dest_client.get_chainid().await?;
    // let dest_contract = FixedDepositAnchorContract::new(
    //     linked_anchor.address,
    //     dest_client.clone(),
    // );

    let provider = HttpProvider::try_from("http://127.0.0.1:9545")
        .unwrap()
        .interval(Duration::from_millis(6u64));
    println!("provider: {:?}", provider);

    let client = SignerMiddleware::new(provider, pvtkey);
    let client = NonceManagerMiddleware::new(client, pubkey);
    let client = Arc::new(client);
    // let provider = Provider::<Http>::try_from("http://localhost:8545").unwrap();
    // let provider = provider.interval(Duration::from_millis(1000));
    // let client = SignerMiddleware::new(provider, wallet);
    // let client = NonceManagerMiddleware::new(client, address);
    // let client = Arc::new(client);

    println!("client: {:?}", client);
    let address = "0x1d412664e5B1c9518995Cf411e8C2F4CC929D5C2"
        .parse::<H160>()
        .unwrap();
    let contract = assembly_sol::AssemblySol::new(address, client.clone());

    // let contract = ethers::contract::Contract::new(address, abi, client);
    println!();
    let val = contract.hello_world().call().await.unwrap();
    println!("contract: {:?}", val);

    // let init_value: String = contract
    // .method::<_, String>("getValue", ())?
    // .call()
    // .await?;
}
