mod interpreter;
use calculator_ast_parser::Compile;
use interpreter::Interpreter;

pub fn main() {
    // rework main to expose cli interactions

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("No input file was provided");
        std::process::exit(-1);
    }
    println!(
        "{:?}",
        Interpreter::from_source(&std::fs::read_to_string(&args[1]).unwrap()).unwrap()
    );
}