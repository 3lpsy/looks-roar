use crate::commands::common;
use clap::ArgMatches;
use ethers::core::types::Address;
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
pub fn run(args: TopArgs) {
    // initialize provider
    dbg!(args);
    unimplemented!();
}
