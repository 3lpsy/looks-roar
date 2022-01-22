mod cli;
mod commands;
mod market;
use commands::floor;

fn main() {
    let args = cli::parser::parse();
    match args.subcommand() {
        Some(("floor", floor_matches)) => match floor::validate(floor_matches) {
            Ok(floor_args) => {
                floor::run(floor_args);
            }
            Err(e) => {
                println!("Error: {:?}", e)
            }
        },
        _ => unimplemented!("Unimp"),
    }
}
