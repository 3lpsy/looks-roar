use clap::{App, AppSettings, Arg};

pub fn parse() -> clap::ArgMatches {
    let arg_number = Arg::new("number")
        .short('n')
        .long("number")
        .help("number to list")
        .default_value("10")
        .takes_value(true);
    let arg_contract = Arg::new("contract")
        .short('c')
        .long("contract")
        .help("contract to target")
        .required(true)
        .takes_value(true);
    let arg_testnet = Arg::new("testnet")
        .short('t')
        .long("testnet")
        .help("use testnet");
    let verbose = Arg::new("verbose")
        .short('v')
        .long("verbose")
        .help("verbosity")
        .global(true);
    App::new("roar")
        .bin_name("roar")
        .about("roarity inspector")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(verbose)
        .subcommand(
            App::new("floor")
                .about("list floor rarirties")
                .arg(arg_number)
                .arg(arg_testnet)
                .arg(arg_contract),
        )
        .get_matches()
}
