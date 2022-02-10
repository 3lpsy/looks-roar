mod cli;
mod commands;
mod contract;
mod db;
mod market;
mod utils;
use commands::{floor, iface, top};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::parser::parse();
    match args.subcommand() {
        Some(("floor", matches)) => match floor::validate(matches) {
            Ok(command_args) => {
                floor::run(command_args);
            }
            Err(e) => {
                println!("Error: {:?}", e)
            }
        },
        Some(("top", matches)) => match top::validate(matches) {
            Ok(command_args) => {
                top::run(command_args).await?;
            }
            Err(e) => {
                println!("Error: {:?}", e)
            }
        },
        Some(("iface", matches)) => match iface::validate(matches) {
            Ok(command_args) => {
                iface::run(command_args).await?;
            }
            Err(e) => {
                println!("Error: {:?}", e)
            }
        },
        _ => unimplemented!("Unimp"),
    }

    Ok(())
}
