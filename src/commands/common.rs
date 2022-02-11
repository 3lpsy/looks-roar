use crate::db::Db;
use crate::utils::AppError;
use clap::ArgMatches;
use ethers::core::types::Address;
use ethers::providers::{Http, Provider};
use std::convert::TryFrom;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
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
    pub db: Option<Db>,
    pub fresh: bool,
}

impl CommonArgs {
    pub fn new(
        contract: ContractArg,
        provider: Option<Provider<Http>>,
        testnet: bool,
        db: Option<Db>,
        fresh: bool,
    ) -> Self {
        Self {
            contract,
            provider,
            testnet,
            db,
            fresh,
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

pub fn validate(args: &ArgMatches) -> Result<CommonArgs, Box<dyn Error>> {
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

    // define caching setup variables
    let mut db_path: Option<String> = None;
    let fresh: bool = args.is_present("fresh");
    if args.is_present("cache") {
        db_path = Some(args.value_of("cache").unwrap().to_string());
    } else if env::var("LOOKS_ROAR_CACHE").is_ok() {
        db_path = Some(env::var("LOOKS_ROAR_CACHE").unwrap());
    }

    if args.is_present("no_cache") {
        db_path = None;
    }

    let mut db: Option<Db> = None;
    if db_path.is_some() {
        db = match Db::open(&db_path.unwrap()) {
            Ok(db) => Some(db),
            Err(e) => {
                // TODO: better handling
                println!("Error loading db: {:?}", e);
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
                return Err(AppError::boxed(
                    format!("Cannot initialize provider from value: {:?}", e),
                    0,
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
                    db,
                    fresh,
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
                        db,
                        fresh,
                    ))
                }
                Err(e) => Err(AppError::boxed(format!("Bad contract address: {:?}", e), 0)),
            },
        },
        None => Err(AppError::boxed("No contract provided".to_string(), 0)),
    }
}
