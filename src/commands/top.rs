use crate::commands::common;
use crate::contract::nft;
// use crate::db::Db;
use clap::ArgMatches;
// use ethers::core::types::Address;
// use ethers::providers::{Http, Provider};
use std::error::Error;
use std::sync::Arc;

#[derive(Debug)]
pub struct TopArgs {
    common: common::CommonArgs,
}

impl TopArgs {
    pub fn new(common: common::CommonArgs) -> Self {
        Self { common }
    }
}

pub fn validate(args: &ArgMatches) -> Result<TopArgs, Box<dyn Error>> {
    match common::validate(args) {
        Ok(common_args) => Ok(TopArgs::new(common_args)),
        Err(e) => Err(e),
    }
}

pub async fn run(args: TopArgs) -> Result<(), Box<dyn Error>> {
    // initialize provider
    // need to do abigen and contract iniitalization
    //
    let provider = Arc::new(args.common.provider.unwrap());
    let fresh = args.common.fresh;
    let db = args.common.db.clone();
    let targets = match args.common.contract {
        common::ContractArg::Address(address) => {
            vec![address]
        }
        common::ContractArg::AddressList(addresses) => addresses,
    };

    let mut api = match nft::Nft::build(targets, provider, db, fresh).await {
        Ok(imp) => imp,
        Err(e) => {
            println!("No NFT interface found supported: {:?}", e);
            std::process::exit(1);
        }
    };
    // load known tokenIds
    api.enumerate().await?;

    unimplemented!()
}
