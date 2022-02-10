use ethers::{prelude::*, providers};
use std::sync::Arc;
use std::time::Duration;
use std::{error::Error, fmt, fs};
// use zk_benchmark::{run, Config};
//
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

pub fn create_typesafe_abi(config: &Config) -> Result<(), Box<dyn Error>> {
    if fs::read_to_string(&config.filename).is_ok() {
        return Ok(());
    }
    // ethers::contract::Abigen::new()
    ethers::contract::Abigen::new("AssemblySol", &config.contract_abi)?
        .generate()?
        .write_to_file(&config.filename)?;

    let abi = fs::read_to_string(&config.filename)?;
    //     Ok(abi) => Ok(abi),
    //     Err(e) => Err(e),
    // }
    // let abi = fs::read_to_string(&config.filename);
    Ok(())
}

type SigningKey = ethers::core::k256::ecdsa::SigningKey;

type ClientMiddleware = ethers::middleware::NonceManagerMiddleware<
    ethers::middleware::SignerMiddleware<
        ethers::providers::Provider<ethers::providers::Http>,
        ethers::signers::Wallet<SigningKey>,
    >,
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

// type MyClient = Arc<ClientMiddleware>;
fn setup_client(provider_addr: &str, wallet: MyWallet) -> Arc<ClientMiddleware> {
    let provider = HttpProvider::try_from(provider_addr)
        .unwrap()
        .interval(Duration::from_millis(6u64));
    println!("provider: {:?}", provider);

    let client = SignerMiddleware::new(provider, wallet.pvtkey);
    let client = NonceManagerMiddleware::new(client, wallet.pubkey);
    let client = Arc::new(client);

    client
}

pub async fn run() -> bool {
    // let pvtkey = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7"
    //     .parse::<LocalWallet>()
    //     .unwrap();
    //
    // let pubkey = pvtkey.address();
    // println!("pubkey: {}", pubkey.to_string());
    let pvtkey = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7";
    let wallet = MyWallet::new(pvtkey);

    println!("wallet: {:?}", wallet);
    // let dest_chain_id = dest_client.get_chainid().await?;
    // let dest_contract = FixedDepositAnchorContract::new(
    //     linked_anchor.address,
    //     dest_client.clone(),
    // );
    // let provider = Provider::<Http>::try_from("http://localhost:8545").unwrap();
    // let provider = provider.interval(Duration::from_millis(1000));
    // let client = SignerMiddleware::new(provider, wallet);
    // let client = NonceManagerMiddleware::new(client, address);
    // let client = Arc::new(client);
    let client = setup_client("http://127.0.0.1:9545", wallet);

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
    true
}

pub struct Config {
    pub contract_abi: String,
    pub filename: String,
}

impl Config {
    pub fn new(contract_abi: String, filename: String) -> Result<Self, &'static str> {
        Ok(Config {
            contract_abi,
            filename,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const POEM: &str = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.
How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!";

    #[test]
    fn search_in_string() {
        assert_eq!(search("Who", POEM), vec!["I'm nobody! Who are you?"]);
        assert_eq!(
            search("nobody", POEM),
            vec!["I'm nobody! Who are you?", "Are you nobody, too?"]
        );

        assert_eq!(
            search("NOboDy", POEM),
            vec!["I'm nobody! Who are you?", "Are you nobody, too?"],
            "Search is not case insensitive",
        );

        assert_eq!(search("NOT IN THE POEM", POEM), Vec::<&str>::new());
    }
}
