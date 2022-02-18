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
    let arg_db = Arg::new("db")
        .short('d')
        .long("db")
        .help("path to db (created automatically if absent)")
        .takes_value(true);
    let arg_fresh = Arg::new("fresh")
        .short('f')
        .long("fresh")
        .help("sync fresh data to db");
    let arg_no_db = Arg::new("no_db")
        .short('N')
        .long("no-db")
        .help("do not use db");
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
        // .subcommand(
        //     App::new("floor")
        //         .about("list floor prices")
        //         .arg(arg_provider.clone())
        //         .arg(arg_number.clone())
        //         .arg(arg_testnet.clone())
        //         .arg(arg_contract.clone()),
        // )
        .subcommand(
            App::new("top")
                .about("list top rarities")
                .arg(arg_provider.clone())
                .arg(arg_testnet.clone())
                .arg(arg_contract.clone())
                .arg(arg_db.clone())
                .arg(arg_fresh.clone())
                .arg(arg_no_db.clone()),
        )
        .subcommand(
            App::new("iface")
                .about("list supported interfaces")
                .arg(arg_provider.clone())
                .arg(arg_testnet.clone())
                .arg(arg_contract.clone())
                .arg(arg_db.clone())
                .arg(arg_fresh.clone())
                .arg(arg_no_db.clone()),
        )
        .get_matches()
}
