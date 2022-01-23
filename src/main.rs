mod cli;
mod commands;
mod market;
use commands::{floor, top};

fn main() {
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
                top::run(command_args);
            }
            Err(e) => {
                println!("Error: {:?}", e)
            }
        },
        _ => unimplemented!("Unimp"),
    }
}
