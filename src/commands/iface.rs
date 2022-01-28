use crate::commands::common;
use crate::contract::nft;
use clap::ArgMatches;
use std::io;

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
pub async fn run(args: IfaceArgs) -> Result<(), io::Error> {
    let address = args.common.contract;
    let provider = args.common.provider.unwrap();
    // TODO: need to confirm args.provider exists and is provided!
    let imp = match nft::NFT::build(address, provider).await {
        Ok(imp) => imp,
        Err(e) => {
            println!("No NFT interface found supported: {:?}", e);
            std::process::exit(1);
        }
    };
    println!("{:?}", imp.iface());
    Ok(())
}
