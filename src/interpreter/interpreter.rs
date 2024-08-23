use crate::lexer::token::Token;
use std::collections::HashMap;
use crate::lexer::token::VariableValue;

pub struct Interpreter {
    pub variables: HashMap<String, VariableValue>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
        }
    }

    pub fn evaluate(&mut self, token: &Token) {
        match token {
            Token::PrintWithArgument(str, args) => {
                println!("Internal interpreter is printing: {}", str);
                if str.starts_with('"') && str.ends_with('"') {
                    // remove the double quotes
                    let arg = str.trim_matches(|c| c == '"');
                    // handle %s
                    if arg.contains("%s") {
                        println!("The argument contains %s");
                        let mut arg = arg.to_string();
                        // Get the variable value
                        let value = self.variables.get(&args[1]).unwrap();
                        // pattern match the value
                        if let VariableValue::String(val) = value {
                            arg = arg.replace("%s", &val);
                        }
                        self.print(&arg);
                        return;
                    }
                    self.print(arg);
                    return;
                }
                else if self.variables.contains_key(str) {
                    let value = self.variables.get(str).unwrap();
                    self.print(&value.eq(&VariableValue::String("".to_string())).to_string());
                    return;
                }
            }
            Token::Print => self.print(""),
            Token::Variable(name, val) => self.assign(name, val.clone()),
            // Handle other tokens
            _ => {},
        }
    }

    pub fn run(&mut self, input: String) {
        let mut lexer = crate::lexer::lexer::Lexer::new(&input);
        loop {
            println!("On next token");
            let token = lexer.next_token();
            println!("Token: {:?}", token);
            self.evaluate(&token); // Pass a reference to the token
            if token == Token::EOF {
                break;
            }
        }
    }

    fn print(&self, message: &str) {
        // I have 0 idea why this is necessary
        let message = message.replace("\\n", "\n");
        print!("{}", message);
    }
    fn assign(&mut self, name: &str, value: VariableValue) {
        self.variables.insert(name.to_string(), value);
    }
}


