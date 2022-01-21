use clap::ArgMatches;
use ethers::core::types::Address;
use std::io;

pub struct TopArgs {
    pub contract: Address,
}

pub fn validate(args: &ArgMatches) -> Result<TopArgs, io::Error> {
    match args.value_of("contract") {
        Some(contract_arg) => match contract_arg.parse::<Address>() {
            Ok(contract) => Ok(TopArgs { contract }),
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
pub fn run(args: TopArgs) {
    // assuming contract exists and is good
    // create request
    // make call
    // get results
}
