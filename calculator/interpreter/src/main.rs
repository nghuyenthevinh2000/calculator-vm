mod interpreter;
use calculator_ast_parser::Compile;
use interpreter::Interpreter;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version)]
#[command(about = "calculator - a simple CLI to calculate")]
struct Cli {
    operation: String
}

fn main() {
    let cli = Cli::parse();

    let out = Interpreter::from_source(&cli.operation).unwrap_or_else(
        |e| {
            panic!("parse error: {}", e)
        }
    );
    println!("result is {}", out)
}