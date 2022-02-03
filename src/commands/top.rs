use crate::cache::Cache;
use crate::commands::common;
use crate::contract::nft;
use clap::ArgMatches;
use ethers::core::types::Address;
use ethers::providers::{Http, Provider};
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

pub async fn run_for_address(
    address: Address,
    provider: Arc<Provider<Http>>,
    db: Option<Cache>,
    fresh: bool,
) -> Result<(), Box<dyn Error>> {
    // TODO: need to confirm args.provider exists and is provided!
    let api = match nft::NFT::build(address, provider, db, fresh).await {
        Ok(imp) => imp,
        Err(e) => {
            println!("No NFT interface found supported: {:?}", e);
            std::process::exit(1);
        }
    };
    println!("{:?}:{:?}", address, api.iface());
    match api.enumerate().await {
        Ok(_tokens) => {
            //..
            unimplemented!();
        }
        Err(e) => {
            //..
            unimplemented!()
        }
    }
}
pub async fn run(args: TopArgs) -> Result<(), Box<dyn Error>> {
    // initialize provider
    // need to do abigen and contract iniitalization
    //
    match args.common.contract {
        common::ContractArg::Address(address) => {
            let provider = Arc::new(args.common.provider.unwrap());
            let fresh = args.common.fresh;
            run_for_address(address, provider, args.common.db, fresh).await
        }
        common::ContractArg::AddressList(addresses) => {
            let provider = Arc::new(args.common.provider.unwrap());
            let fresh = args.common.fresh;
            for address in addresses {
                // TODO: add cache
                let db = args.common.db.clone();
                let _is_good = run_for_address(address, provider.clone(), db, fresh)
                    .await
                    .is_ok();
            }
            Ok(())
        }
    }
}
