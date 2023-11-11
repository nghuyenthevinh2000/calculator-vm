mod compiler;
use calculator_ast_parser::Compile;
use compiler::Compiler;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version)]
#[command(about = "calculator - a simple CLI to calculate based on llvm")]
struct Cli {
    operation: String
}

// cargo run --package calculator-compiler --bin main
fn main() {
    let cli = Cli::parse();

    let out = Compiler::from_source(&cli.operation).unwrap_or_else(
        |e| {
            panic!("parse error: {}", e)
        }
    );
    println!("result is {}", out)
}