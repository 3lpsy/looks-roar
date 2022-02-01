use crate::cache::Cache;
use crate::commands::common;
use crate::contract::nft;
use clap::ArgMatches;
use ethers::core::types::Address;
use ethers::providers::{Http, Provider};
use std::io;
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

pub fn validate(args: &ArgMatches) -> Result<TopArgs, io::Error> {
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
) -> Result<(), io::Error> {
    // TODO: need to confirm args.provider exists and is provided!
    let mut imp = match nft::NFT::build(address, provider, db, fresh).await {
        Ok(imp) => imp,
        Err(e) => {
            println!("No NFT interface found supported: {:?}", e);
            std::process::exit(1);
        }
    };
    println!("{:?}", imp.iface());
    unimplemented!();
}
pub async fn run(args: TopArgs) -> Result<(), io::Error> {
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
                let _is_good = run_for_address(address, provider.clone(), None, fresh)
                    .await
                    .is_ok();
            }
            Ok(())
        }
    }
}
