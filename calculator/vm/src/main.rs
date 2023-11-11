mod opcode;
mod bytecode;
mod vm;

use calculator_ast_parser::Compile;
use clap::Parser;

use crate::{vm::VM, bytecode::Interpreter};

#[derive(Debug, Parser)]
#[command(author, version)]
#[command(about = "calculator - a simple CLI to calculate")]
struct Cli {
    operation: String
}

// cargo run --package calculator-vm --bin main
fn main() {
    let cli = Cli::parse();

    // create new byte code
    let bytecode = Interpreter::from_source(&cli.operation);

    // create new vm
    let mut vm = VM::new(bytecode);
    vm.run();
    let out = vm.get_result();

    println!("result is {}", out)
}