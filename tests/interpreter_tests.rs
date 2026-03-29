//! Tests for Lexer

#[cfg(test)]
mod tests {

    use simple_interpreter::{
        interpreter::exec, lexer::{lex}, sem_parser::{
            parse,
        }
    };

    #[test]
    fn test_exec_simple_test_file() {
        let filepath = "tests/example_files/simple_test.ms";
        let input_str = std::fs::read_to_string(filepath).unwrap();
        let lex_vec = lex(&input_str).unwrap();

        let program = match parse(&lex_vec, &input_str) {
            Ok(program) => program,
            Err(error) => {
                println!("{}", error.generate_error_msg(&input_str));
                panic!("program not read in correctly");
            }
        };

        exec(program, &input_str).unwrap();
    }

    #[test]
    fn test_exec_loop_test_file() {
        let filepath = "tests/example_files/loop_test.ms";
        let input_str = std::fs::read_to_string(filepath).unwrap();
        let lex_vec = lex(&input_str).unwrap();

        let program = match parse(&lex_vec, &input_str) {
            Ok(program) => program,
            Err(error) => {
                println!("{}", error.generate_error_msg(&input_str));
                panic!("program not read in correctly");
            }
        };

        exec(program, &input_str).unwrap();
    }
}
