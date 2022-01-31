use crate::commands::common;
use crate::contract::nft;
use clap::ArgMatches;
use ethers::core::types::Address;
use ethers::providers::{Http, Provider};
use sled;
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
    cache: Option<sled::Db>,
) -> Result<(), io::Error> {
    // TODO: need to confirm args.provider exists and is provided!
    let mut imp = match nft::NFT::build(address, provider, cache).await {
        Ok(imp) => imp,
        Err(e) => {
            println!("No NFT interface found supported: {:?}", e);
            std::process::exit(1);
        }
    };
    imp.load_metadata().await?;
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
            run_for_address(address, provider, args.common.cache).await
        }
        common::ContractArg::AddressList(addresses) => {
            let provider = Arc::new(args.common.provider.unwrap());
            for address in addresses {
                // TODO: add cache
                let _is_good = run_for_address(address, provider.clone(), None)
                    .await
                    .is_ok();
            }
            Ok(())
        }
    }
}
