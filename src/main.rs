use std::fmt::Debug;

/// Provide a command line tool for interpret the in the README defined language definition
use clap::Parser;
use simple_interpreter::{exec, lex_ascii, lexer::GlobalError, parse};
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

/// wrapper function to tokenize, parse and interpret an input string
/// Used for universal Error handling
fn interpret_input_str(input_str: &str) -> Result<(), GlobalError> {
    // Lexical Analysis
    let token_vector = lex_ascii(input_str)?;

    // Parsing and minimal semantic analysis
    let program = parse(&token_vector, input_str)?;

    // Interpret file
    exec(program, input_str)
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

    if let Err(e) = interpret_input_str(&input_str) {
        println!("{}", e.generate_error_msg(&input_str));
    }
}
