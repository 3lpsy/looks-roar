use crate::market::opensea;
use crate::market::types::Network;
use clap::ArgMatches;
use ethers::core::types::Address;
use std::io;

pub struct FloorArgs {
    pub contract: Address,
    pub testnet: bool,
    pub number: u8,
}

pub fn validate(args: &ArgMatches) -> Result<FloorArgs, io::Error> {
    match args.value_of("contract") {
        Some(contract_arg) => match contract_arg.parse::<Address>() {
            Ok(contract) => Ok(FloorArgs {
                contract,
                testnet: args.is_present("testnet"),
                number: args.value_of_t("number").unwrap_or_else(|e| e.exit()),
            }),
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
pub fn run(args: FloorArgs) {
    // assuming contract exists and is good
    // build client
    let api = match args.testnet {
        true => opensea::OpenSeaApi::new(Network::Rinkeby),
        false => opensea::OpenSeaApi::new(Network::Mainnet),
    };

    // floor api is syntactice sugar that builds request for you
    // send request and take action
    match api.get_floor(args.contract, args.number) {
        Ok(data) => dbg!(data),
        Err(e) => {
            dbg!(e);
            panic!("ouch");
        }
    }
}
