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
    let arg_provider = Arg::new("provider")
        .short('p')
        .long("provider")
        .help("ethereum provider")
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
        .about("> An NFT roarity inspector")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(verbose)
        .subcommand(
            App::new("floor")
                .about("list floor prices")
                .arg(arg_provider.clone())
                .arg(arg_number.clone())
                .arg(arg_testnet.clone())
                .arg(arg_contract.clone()),
        )
        .subcommand(
            App::new("top")
                .about("list top rarities")
                .arg(arg_provider.clone())
                .arg(arg_number.clone())
                .arg(arg_testnet.clone())
                .arg(arg_contract.clone()),
        )
        .get_matches()
}
