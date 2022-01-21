mod cli;
mod commands;
mod market;
use commands::top;

fn main() {
    let args = cli::parser::parse();
    match args.subcommand() {
        Some(("top", top_matches)) => match top::validate(top_matches) {
            Ok(top_args) => {
                top::run(top_args);
            }
            Err(e) => {
                println!("Error: {:?}", e)
            }
        },
        _ => unimplemented!("Unimp"),
    }
}
