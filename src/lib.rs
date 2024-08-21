mod interpreter;
mod lexer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_print_hello_world() {
        let mut interpreter = interpreter::interpreter::Interpreter::new();
        interpreter.run("print(\"Hello, World!\")".to_string());
    }
    #[test]
    fn can_assign_variable() {
        let mut interpreter = interpreter::interpreter::Interpreter::new();
        interpreter.run("var x = 10".to_string());
        assert_eq!(interpreter.variables.get("x"), Some(&10));
    }
}
