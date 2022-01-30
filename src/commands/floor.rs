use crate::commands::common;
use crate::market::opensea;
use crate::market::types::Endpoint;
use clap::ArgMatches;
use ethers::core::types::Address;
use std::io;

pub struct FloorArgs {
    common: common::CommonArgs,
    number: u8,
}

// TODO: moved number from common
pub fn validate(args: &ArgMatches) -> Result<FloorArgs, io::Error> {
    match common::validate(args) {
        Ok(common_args) => Ok(FloorArgs {
            common: common_args,
            number: 0,
        }),
        Err(e) => Err(e),
    }
}

pub async fn run_for_address(
    address: Address,
    api: opensea::OpenSeaApi,
    number: u8,
) -> Result<(), io::Error> {
    // floor api is syntactice sugar that builds request for you
    // send request and take action
    match api.get_floor(address, number) {
        Ok(data) => dbg!(data),
        Err(e) => {
            dbg!(e);
            panic!("ouch");
        }
    };

    Ok(())
}
pub fn run(args: FloorArgs) {
    // assuming contract exists and is good
    // build client
    let api = match args.common.testnet {
        true => opensea::OpenSeaApi::new(Endpoint::Rinkeby),
        false => opensea::OpenSeaApi::new(Endpoint::Mainnet),
    };

    match args.common.contract {
        common::ContractArg::Address(address) => {
            let num = args.number;
            let _res = run_for_address(address, api, num);
        }
        common::ContractArg::AddressList(addresses) => {
            let num = args.number;
            for address in addresses {
                // TODO: make api clonable? or handle borrowed api
                let api = match args.common.testnet {
                    true => opensea::OpenSeaApi::new(Endpoint::Rinkeby),
                    false => opensea::OpenSeaApi::new(Endpoint::Mainnet),
                };
                let _res = run_for_address(address, api, num);
            }
        }
    }
}
