mod cli;
mod commands;
mod contract;
mod market;
use commands::{floor, top};
use std::io;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
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
        _ => unimplemented!("Unimp"),
    }

    Ok(())
}
