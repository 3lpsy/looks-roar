use crate::commands::common;
use crate::contract::nft;
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

    let api = match nft::NFT::build(targets, provider, db, fresh).await {
        Ok(imp) => imp,
        Err(e) => {
            println!("No NFT interface found supported: {:?}", e);
            std::process::exit(1);
        }
    };
    for (address, entry) in api.nfts {
        let ifaces_str = entry
            .ifaces
            .iter()
            .fold(String::new(), |carry, iface| {
                carry + "," + &iface.to_string()
            })
            .trim_matches(',')
            .to_owned();
        println!("{:?}:{}", address, ifaces_str);
    }
    Ok(())
}
