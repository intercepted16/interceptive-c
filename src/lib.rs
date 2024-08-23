use std::env;
use crate::interpreter::interpreter::Interpreter;

mod interpreter;
mod lexer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_print_hello_world() {
        let mut interpreter = Interpreter::new();
        interpreter.run("print(\"Hello, World!\")".to_string());
    }
    #[test]
    fn can_assign_variable() {
        let mut interpreter = Interpreter::new();
        interpreter.run("var x = 10".to_string());
        assert_eq!(interpreter.variables.get("x").unwrap(), &lexer::token::VariableValue::Integer(10));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // Check if the user has provided a script to run
    if args.len() < 2 {
        println!("Usage: {} <script>", args[0]);
        std::process::exit(1);
    }
    // Check that the file exists and ends in `.ic`
    if !args[1].ends_with(".ic") {
        // format the error message with a red color
        eprintln!("\x1b[31mError: Invalid file extension\x1b[0m");
        std::process::exit(1);
    }

    let script = std::fs::read_to_string(&args[1]);
    if let Err(e) = script {
        eprintln!("\x1b[31mError: {}\x1b[0m", e);
        std::process::exit(1);
    }
    let script = script.unwrap();

    let mut interpreter = Interpreter::new();
    interpreter.run(script);
}

