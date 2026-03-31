use std::fmt::Debug;

/// Provide a command line tool for interpret the in the README defined language definition
use clap::Parser;
use simple_interpreter::{exec, lex_ascii, parse};
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// simple_interpreter is an interpreter for a minimal turing complete language.
///
/// It takes as input a path (--path) to the file to interpret.
struct Args {
    /// file to interpret
    #[clap(short, long)]
    path: String,
}

fn main() {
    // enable logging
    env_logger::init();

    let args = Args::parse();

    // Reading in file
    let input_str = match std::fs::read_to_string(&args.path) {
        Ok(path) => path,
        Err(err) => {
            println!("Could not read file: {}\nError:{}", &args.path, err);
            return;
        }
    };

    // Lexical Analysis
    let token_vector = match lex_ascii(&input_str) {
        Ok(tokens) => tokens,
        Err(err) => {
            println!("Lexical Error\n{}", err.generate_error_msg(&input_str));
            return;
        }
    };

    // Parsing and minimal semantic analysis
    let program = match parse(&token_vector, &input_str) {
        Ok(program) => program,
        Err(err) => {
            println!("Parsing Error\n{}", err.generate_error_msg(&input_str));
            return;
        }
    };

    // Interpret file
    if let Err(err) = exec(program, &input_str) {
        println!("Runtime Error\n{}", err.generate_error_msg(&input_str));
    }
}
