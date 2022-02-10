use std::{env, error::Error, fmt, fs};

#[derive(Debug, Clone)]
struct AbiError(String);

impl fmt::Display for AbiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid abi input")
    }
}
impl Error for AbiError {}

pub fn get_abi(config: &Config) -> Result<String, Box<dyn Error>> {
    if let Ok(abi) = fs::read_to_string(&config.filename) {
        return Ok(abi);
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
    Ok(abi)
}

pub struct Config {
    pub contract_abi: String,
    pub filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Self, &'static str> {
        args.next();

        let contract_abi = args.next().ok_or("Didn't get abi file")?;
        let filename = args.next().ok_or("Didn't get rs file")?;

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
