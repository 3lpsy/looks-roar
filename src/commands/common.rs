use clap::ArgMatches;
use ethers::core::types::Address;
use ethers::providers::{Http, Provider};
use std::convert::TryFrom;
use std::env;
use std::io;

#[derive(Debug)]
pub struct CommonArgs {
    pub contract: Address,
    pub provider: Option<Provider<Http>>,
    pub testnet: bool,
    pub number: u8,
}

impl CommonArgs {
    pub fn new(
        contract: Address,
        provider: Option<Provider<Http>>,
        testnet: bool,
        number: u8,
    ) -> Self {
        Self {
            contract,
            provider,
            testnet,
            number,
        }
    }
}

pub fn validate(args: &ArgMatches) -> Result<CommonArgs, io::Error> {
    // i don't like this
    let mut provider: Option<Provider<Http>> = None;
    let mut provider_url = String::new();
    if args.is_present("provider") {
        provider_url = args.value_of("provider").unwrap().to_string();
    } else if env::var("ETHEREUM_RPC_URL").is_ok() {
        // fallback to env var if its set
        provider_url = env::var("ETHEREUM_RPC_URL").unwrap();
    }
    if !provider_url.is_empty() {
        match Provider::<Http>::try_from(provider_url.as_str()) {
            Ok(prov) => {
                // have to override. is there better way?
                provider = Some(prov);
            }
            Err(e) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Cannot initialize provider from value: {:?}", e),
                ));
            }
        }
    }
    match args.value_of("contract") {
        Some(contract_arg) => match contract_arg.parse::<Address>() {
            Ok(contract) => {
                let number = args.value_of_t("number").unwrap_or_else(|e| e.exit());
                let testnet = args.is_present("testnet");
                Ok(CommonArgs::new(contract, provider, testnet, number))
            }
            Err(e) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Bad contract address: {:?}", e),
            )),
        },
        None => Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No contract provided",
        )),
    }
}
