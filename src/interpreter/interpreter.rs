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
                    let mut number_of_formats = 0;
                    // remove the double quote
                    let arg = str.trim_matches(|c| c == '"');
                    // loop over each %s in the string
                    for c in arg.chars() {
                        if c == '%' {
                            number_of_formats += 1;
                        }
                    }
                    // if the number of %s in the string is equal to the number of arguments
                    // print the string with the arguments
                    println!("Number of formats: {}", number_of_formats);
                    if number_of_formats == args.len() - 1 {
                        let mut arg = arg.to_string();
                        for a in args {
                            // ignore the first argument
                            if a == &args[0] {
                                continue;
                            }
                            println!("Replacing %s with {}", a);
                            // get the variable value and replace the %s with the value
                            println!("Getting value for {}", a);
                            println!("Variables: {:?}", self.variables);
                            let value = self.variables.get(a).unwrap();
                            let replacement = match value {
                                VariableValue::Integer(i) => i.to_string(),
                                VariableValue::String(s) => s.clone(),
                                VariableValue::Undefined => "".to_string(),
                            };
                            arg = arg.replacen("%s", &replacement, 1);
                        }
                    self.print(&arg);
                    return;
                }
                else if self.variables.contains_key(str) {
                    let value = self.variables.get(str).unwrap();
                    self.print(&value.eq(&VariableValue::String("".to_string())).to_string());
                    return;
                }
            }
        }
            Token::Print => self.print(""),
            Token::Variable(name, val) => self.assign(name, val.clone()),
            Token::FunctionDefinition(name, body, args) => {
                println!("Function body: {}", body);
                // create a new interpreter and run the function body
                // so it has its own scope
                let mut interpreter = Interpreter::new();
                for (i, arg) in args.iter().enumerate() {
                    interpreter.assign(arg, VariableValue::String(args[i].clone()));
                }
                interpreter.run(body.to_string());

            },
            // Handle other tokens
            _ => {},
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

    fn print(&self, message: &str) {
        // I have 0 idea why this is necessary
        let message = message.replace("\\n", "\n");
        print!("{}", message);
    }
    fn assign(&mut self, name: &str, value: VariableValue) {
        self.variables.insert(name.to_string(), value);
    }
}


