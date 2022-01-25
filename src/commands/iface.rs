use crate::commands::common;
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
    Ok(())
}
