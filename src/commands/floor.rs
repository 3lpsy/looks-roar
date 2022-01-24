use crate::commands::common;
use crate::market::opensea;
use crate::market::types::Endpoint;
use clap::ArgMatches;
use std::io;

pub struct FloorArgs {
    common: common::CommonArgs,
}

pub fn validate(args: &ArgMatches) -> Result<FloorArgs, io::Error> {
    match common::validate(args) {
        Ok(common_args) => Ok(FloorArgs {
            common: common_args,
        }),
        Err(e) => Err(e),
    }
}
pub fn run(args: FloorArgs) {
    // assuming contract exists and is good
    // build client
    let api = match args.common.testnet {
        true => opensea::OpenSeaApi::new(Endpoint::Rinkeby),
        false => opensea::OpenSeaApi::new(Endpoint::Mainnet),
    };

    // floor api is syntactice sugar that builds request for you
    // send request and take action
    match api.get_floor(args.common.contract, args.common.number) {
        Ok(data) => dbg!(data),
        Err(e) => {
            dbg!(e);
            panic!("ouch");
        }
    }
}
