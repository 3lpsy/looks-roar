use clap::ArgMatches;
use ethers::core::types::Address;
use ethers::providers::{Http, Provider};
use sled;
use std::convert::TryFrom;
use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

#[derive(Debug)]
pub enum ContractArg {
    Address(Address),
    AddressList(Vec<Address>),
}

#[derive(Debug)]
pub struct CommonArgs {
    pub contract: ContractArg,
    pub provider: Option<Provider<Http>>,
    pub testnet: bool,
    pub cache: Option<sled::Db>,
}

impl CommonArgs {
    pub fn new(
        contract: ContractArg,
        provider: Option<Provider<Http>>,
        testnet: bool,
        cache: Option<sled::Db>,
    ) -> Self {
        Self {
            contract,
            provider,
            testnet,
            cache,
        }
    }
}

fn load_dotenv_line(line: String) {
    match line.split_once("=") {
        Some(kv) => {
            let val = kv.1.trim_matches('"');
            println!("Setting env variable: {:?}", kv.0);
            std::env::set_var(kv.0, val)
        }
        None => {
            println!("Failed to parse env line");
        }
    }
}

fn load_dotenv(path: &str) {
    if Path::new(path).exists() {
        match File::open(path) {
            Ok(f) => {
                let b = BufReader::new(f);
                for (_index, line) in b.lines().enumerate() {
                    let line = line.unwrap(); // Ignore errors.
                    if !line.starts_with('#') {
                        load_dotenv_line(line);
                    }
                }
            }
            Err(_) => {
                println!("Failed to read env file: {:?}", path);
            }
        }
    }
}

pub fn validate(args: &ArgMatches) -> Result<CommonArgs, io::Error> {
    load_dotenv("./.env");
    // i don't like this
    let mut provider: Option<Provider<Http>> = None;
    let mut provider_url = String::new();

    if args.is_present("provider") {
        provider_url = args.value_of("provider").unwrap().to_string();
    } else if env::var("ETHEREUM_RPC_URL").is_ok() {
        // fallback to env var if its set
        provider_url = env::var("ETHEREUM_RPC_URL").unwrap();
    }

    let mut cache_path: Option<String> = None;
    if args.is_present("cache") {
        cache_path = Some(args.value_of("cache").unwrap().to_string());
    } else if env::var("LOOKS_ROAR_CACHE").is_ok() {
        cache_path = Some(env::var("LOOKS_ROAR_CACHE").unwrap());
    }

    if args.is_present("no_cache") {
        cache_path = None;
    }

    let mut cache: Option<sled::Db> = None;
    if cache_path.is_some() {
        cache = match sled::open(cache_path.unwrap()) {
            Ok(db) => Some(db),
            Err(e) => {
                // TODO: better handling
                println!("Error loading cache: {:?}", e);
                None
            }
        };
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

    let mut contract: Option<String> = None;
    if args.is_present("contract") {
        contract = Some(args.value_of("contract").unwrap().to_string());
    } else if env::var("LOOKS_ROAR_CONTRACT").is_ok() {
        contract = Some(env::var("LOOKS_ROAR_CONTRACT").unwrap());
    }

    match contract {
        Some(arg) => match arg.parse::<Address>() {
            Ok(val) => {
                let testnet = args.is_present("testnet");
                Ok(CommonArgs::new(
                    ContractArg::Address(val),
                    provider,
                    testnet,
                    cache,
                ))
            }
            Err(_e) => match File::open(arg) {
                Ok(f) => {
                    let testnet = args.is_present("testnet");
                    let buffer = BufReader::new(f);
                    // TODO: find a cooler/cleaner way to do this
                    // TODO: parse failures don't bubble/report
                    let addrs: Vec<Address> = buffer
                        .lines()
                        .filter(|line| line.is_ok())
                        .map(|line| line.unwrap())
                        .map(|line| line.parse::<Address>())
                        .filter(|line| line.is_ok())
                        .map(|line| line.unwrap())
                        .collect();

                    Ok(CommonArgs::new(
                        ContractArg::AddressList(addrs),
                        provider,
                        testnet,
                        cache,
                    ))
                }
                Err(e) => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Bad contract address: {:?}", e),
                )),
            },
        },
        None => Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No contract provided",
        )),
    }
}
