use crate::commands::common;
use crate::contract::nft;
use clap::ArgMatches;
use ethers::core::types::Address;
use ethers::providers::{Http, Provider};
use std::io;
use std::sync::Arc;

pub struct IfaceArgs {
    common: common::CommonArgs,
}

pub fn validate(args: &ArgMatches) -> Result<IfaceArgs, io::Error> {
    match common::validate(args) {
        Ok(common_args) => Ok(IfaceArgs {
            common: common_args,
        }),
        Err(e) => Err(e),
    }
}

pub async fn run_for_address(
    address: Address,
    provider: Arc<Provider<Http>>,
) -> Result<(), io::Error> {
    let mut imp = match nft::NFT::build(address.clone(), provider).await {
        Ok(imp) => imp,
        Err(e) => {
            println!("No NFT interface found supported: {:?}", e);
            std::process::exit(1);
        }
    };
    // TODO: need to confirm args.provider exists and is provided!

    let mut opt_str: String = String::new();
    if !imp.opt_ifaces().is_empty() {
        let opt_joined = imp
            .opt_ifaces()
            .iter()
            .fold(String::new(), |carry, item| carry + "," + &item.to_string());
        opt_str = format!(" ({})", opt_joined.trim_matches(','));
    }

    println!("{:?}:{:?}{}", address, imp.iface(), opt_str);
    // TODO: check for other implementations
    Ok(())
}

pub async fn run(args: IfaceArgs) -> Result<(), io::Error> {
    match args.common.contract {
        common::ContractArg::Address(address) => {
            // TODO: handle
            let provider = args.common.provider.unwrap();
            let provider = Arc::new(provider);
            run_for_address(address, provider).await
        }
        common::ContractArg::AddressList(addresses) => {
            // TODO: handle
            let provider = args.common.provider.unwrap();
            let provider = Arc::new(provider);
            for address in addresses {
                let _res = run_for_address(address, provider.clone()).await;
            }
            Ok(())
        }
    }
}
