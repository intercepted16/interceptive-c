use crate::lexer::token::Token;
use std::collections::HashMap;
use crate::lexer::token::VariableValue;

impl Interpreter {

}

pub struct Interpreter {
    pub variables: HashMap<String, VariableValue>,
    pub functions: HashMap<String, String>,
    pub call_stack: Vec<String>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            functions: HashMap::new(),
            call_stack: Vec::new(),
        }
    }
        pub fn evaluate(&mut self, token: &Token) {
            match token {
                Token::Variable(name, val) => self.assign(name, val.clone()),
                Token::FunctionDefinition(name, body, args) => {
                    println!("Function body: {}", body);
                    println!("Inserting function: {} with body: {}", name, body);
                    self.functions.insert(name.to_string(), body.to_string());
                }
                Token::FunctionCall(name, args) => {
                    if self.call_stack.contains(name) {
                        println!("Error: Recursive call detected for function {}", name);
                        return;
                    }
                    self.call_stack.push(name.to_string());
                    println!("Functions are: {:?}", self.functions);
                    let function = self.functions.get(name).unwrap();
                    if let (body) = function {
                        let mut interpreter = Interpreter::new();
                        for (i, arg) in args.iter().enumerate() {
                            interpreter.assign(arg, VariableValue::String(args[i].clone()));
                        }
                        interpreter.run(body.to_string());
                    }
                    self.call_stack.pop();
                }
                _ => {}
            }
        }


    pub fn run(&mut self, input: String) {
        let mut lexer = crate::lexer::lexer::Lexer::new(&input);
        loop {
            println!("On next token");
            let token = lexer.next_token().unwrap_or(Token::EOF);
            println!("Token: {:?}", token);
            self.evaluate(&token); // Pass a reference to the token
            if token == Token::EOF {
                break;
            }
        }
    }

    fn assign(&mut self, name: &str, value: VariableValue) {
        self.variables.insert(name.to_string(), value);
    }
}


