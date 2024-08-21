use crate::lexer::token::Token;
use std::collections::HashMap;

pub struct Interpreter {
    pub variables: HashMap<String, i64>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
        }
    }

    pub fn evaluate(&mut self, token: &Token) {
        match token {
            Token::PrintWithArgument(arg) => self.print(arg),
            Token::Print => self.print(""),
            Token::Variable(name, val) => self.assign(name, *val),
            // Handle other tokens
            _ => {},
        }
    }

    pub fn run(&mut self, input: String) {
        let mut lexer = crate::lexer::lexer::Lexer::new(&input);
        loop {
            let token = lexer.next_token();
            self.evaluate(&token); // Pass a reference to the token
            if token == Token::EOF {
                break;
            }
        }
    }

    fn print(&self, message: &str) {
        println!("{}", message);
    }
    fn assign(&mut self, name: &str, value: i64) {
        self.variables.insert(name.to_string(), value);
    }
}
