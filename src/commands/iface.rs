use crate::commands::common;
use clap::ArgMatches;
use std::error::Error;
use std::sync::Arc;

pub struct IfaceArgs {
    common: common::CommonArgs,
}

pub fn validate(args: &ArgMatches) -> Result<IfaceArgs, Box<dyn Error>> {
    match common::validate(args) {
        Ok(common_args) => Ok(IfaceArgs {
            common: common_args,
        }),
        Err(e) => Err(e),
    }
}

pub async fn run(args: IfaceArgs) -> Result<(), Box<dyn Error>> {
    let provider = Arc::new(args.common.provider.unwrap());
    let db = args.common.db;

    let fresh = args.common.fresh;
    let targets = match args.common.contract {
        common::ContractArg::Address(address) => vec![address],
        common::ContractArg::AddressList(addresses) => addresses,
    };
    Ok(())
}
