use crate::commands::common;
use crate::contract::nft;
use clap::ArgMatches;
use std::io;

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
pub async fn run(args: TopArgs) -> Result<(), io::Error> {
    // initialize provider
    // need to do abigen and contract iniitalization
    //
    let address = args.common.contract;
    let provider = args.common.provider.unwrap();
    // TODO: need to confirm args.provider exists and is provided!

    let x = nft::NFT::build(address, provider);
    //let nft = nft::build_any(address, provider).await?;
    // dbg!(nft.unwrap().iface);
    unimplemented!();
}
