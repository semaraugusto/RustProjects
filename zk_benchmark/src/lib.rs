use ethers::{prelude::*, providers};
use std::sync::Arc;
use std::time::Duration;
use std::{error::Error, fmt};

mod assembly_sol;

type HttpProvider = providers::Provider<providers::Http>;

#[derive(Debug, Clone)]
struct AbiError(String);

impl fmt::Display for AbiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid abi input")
    }
}
impl Error for AbiError {}

type SigningKey = ethers::core::k256::ecdsa::SigningKey;

// type ClientMiddleware = ethers::middleware::NonceManagerMiddleware<
//     ethers::middleware::SignerMiddleware<
//         ethers::providers::Provider<ethers::providers::Http>,
//         ethers::signers::Wallet<SigningKey>,
//     >,
// >;
type ClientMiddleware = ethers::middleware::SignerMiddleware<
    ethers::providers::Provider<ethers::providers::Http>,
    ethers::signers::Wallet<SigningKey>,
>;

#[derive(Debug, Clone)]
struct MyWallet {
    pvtkey: Wallet<SigningKey>,
    pubkey: H160,
}

impl MyWallet {
    fn new(pvtkey: &str) -> MyWallet {
        let pvtkey = pvtkey.parse::<LocalWallet>().unwrap();

        let pubkey = pvtkey.address();

        MyWallet { pvtkey, pubkey }
    }
}

fn setup_client(provider_addr: &str, wallet: MyWallet) -> Arc<ClientMiddleware> {
    let provider = HttpProvider::try_from(provider_addr)
        .unwrap()
        .interval(Duration::from_millis(6u64));
    println!("provider: {:?}", provider);

    println!();
    let client = SignerMiddleware::new(provider, wallet.pvtkey);
    println!("client: {:?}", client);
    Arc::new(client)
}

fn setup_contract(
    address: &str,
    client: Arc<ClientMiddleware>,
) -> assembly_sol::AssemblySol<ClientMiddleware> {
    let address = "0x1d412664e5B1c9518995Cf411e8C2F4CC929D5C2"
        .parse::<H160>()
        .unwrap();
    assembly_sol::AssemblySol::new(address, client.clone())
}

#[derive(Debug, Clone)]
pub enum BenchmarkerState {
    Running,
    Stopped,
}

pub struct Benchmarker {
    pub state: BenchmarkerState,
    pub num_transactions: u64,
    // client: Arc<ClientMiddleware>,
    contract: assembly_sol::AssemblySol<ClientMiddleware>,
}

impl Benchmarker {
    pub fn new(pvtkey: &str, provider_addr: &str, contract_addr: &str) -> Benchmarker {
        let my_wallet = MyWallet::new(pvtkey);

        println!("wallet: {:?}", my_wallet);

        let client = setup_client(provider_addr, my_wallet);

        println!("client: {:?}", client);
        // let address = "0x1d412664e5B1c9518995Cf411e8C2F4CC929D5C2";
        let contract = setup_contract(contract_addr, client);
        Benchmarker {
            state: BenchmarkerState::Running,
            num_transactions: 0,
            contract,
        }
    }

    pub async fn make_request(&mut self) {
        // let contract = ethers::contract::Contract::new(address, abi, client);
        if self.num_transactions > 10 {
            self.state = BenchmarkerState::Stopped;
        }
        println!();
        let contract_call = self.contract.hello_world();
        println!("contract: {:?}", contract_call.tx);
        let pending_tx = contract_call.send().await.unwrap();
        std::thread::sleep(Duration::from_secs(2));
        println!("pending_tx: {:?}", pending_tx);
        let receipt = pending_tx.confirmations(1).await;
        println!("contract: {:?}", receipt);
        self.num_transactions += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::Benchmarker;

    #[tokio::test]
    async fn test_send_eth() {
        let pvtkey = "21674d1610e72ce115f58bf85a93543a520fe1bdd403091973a39e05e1a84f1c";
        let provider_addr = "http://127.0.0.1:9545";
        let contract_addr = "0x1d412664e5B1c9518995Cf411e8C2F4CC929D5C2";
        let mut benchmarker = Benchmarker::new(pvtkey, provider_addr, contract_addr);

        // benchmarker.send_eth().await;
    }
}
