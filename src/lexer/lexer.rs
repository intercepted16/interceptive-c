use crate::lexer::token::{Token, VariableValue};
// funky: a function
// here: return
// var: variable
// print: print
// if: if
// else: else
// while: while
// for: for
fn keywords() -> Vec<&'static str> {
    vec!["var", "print", "if", "else", "while", "for", "funky", "here"]
}

fn syntax() -> Vec<&'static str> {
    vec!["()"]
}


pub struct Lexer<'a> {
    input: &'a str,
    current_pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input,
            current_pos: 0,
        }
    }

    fn current_char(&self) -> Option<char> {
        self.input[self.current_pos..].chars().next()
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();

        if self.current_pos >= self.input.len() {
            return Ok(Token::EOF);
        }

        for keyword in keywords() {
            if self.input[self.current_pos..].starts_with(&keyword) {
                // match the keyword
                return match keyword {
                    "var" => self.handle_var(),
                    "print" => self.handle_print(),
                    "funky" => self.handle_funky(),
                    _ => Err(format!("Unknown keyword: {}", keyword)),
                };
            }
        }
        for symbol in syntax() {
            if self.input[self.current_pos..].starts_with(&symbol) {
                return match symbol {
                    "()" => self.handle_fn_call(),
                    _ => Err(format!("Unknown symbol: {}", symbol)),
                };
            }
        }
        self.current_pos += 1;
        self.next_token()
    }

    fn skip_whitespace(&mut self) {
        while self.current_pos < self.input.len() && self.current_char().unwrap().is_whitespace() {
            self.current_pos += 1;
        }
    }

    fn extract_args(&self, args_str: &str) -> Vec<String> {
        let mut args = Vec::new();
        let mut current_pos = 0;
        let input_len = args_str.len();

        while current_pos < input_len {
            let start_pos = current_pos;
            // Check if the current character is a quote
            // Do this because we want to ignore commas inside quotes
            if args_str[current_pos..].chars().next().unwrap() == '"' {
                current_pos += 1; // Move past the opening quote
                while current_pos < input_len && args_str[current_pos..].chars().next().unwrap() != '"' {
                    current_pos += 1;
                }
                current_pos += 1; // Move past the closing quote
                // add the argument to the list
                args.push(args_str[start_pos..current_pos].to_string());
                continue;
            }

            // Find the end of the current argument
            while current_pos < input_len && args_str[current_pos..].chars().next().unwrap() != ',' {
                current_pos += 1;
            }

            // Extract the argument and trim leading/trailing whitespace
            let arg = args_str[start_pos..current_pos].trim().to_string();
            if !arg.is_empty() {
                args.push(arg);
            }

            // Move past the comma, if present
            if current_pos < input_len && args_str[current_pos..].chars().next().unwrap() == ',' {
                current_pos += 1;
            }
        }

        args
    }

    fn handle_var(&mut self) -> Result<Token, String> {
        self.current_pos += 3; // Move past "var"
        self.skip_whitespace();

        // Find the variable name
        let start_pos = self.current_pos;
        while self.current_pos < self.input.len() && self.input[self.current_pos..].chars().next().unwrap().is_alphanumeric() {
            self.current_pos += 1;
        }
        let variable_name = &self.input[start_pos..self.current_pos];
        self.skip_whitespace();

        // Ensure the next character is '='
        if self.current_char() == Option::from('=') {
            self.current_pos += 1; // Move past '='
            self.skip_whitespace();

            let mut start_pos = self.current_pos;

            // Check if the value is a string
            return if self.current_char() == Option::from('"') {
                start_pos += 1; // Move past opening quote
                self.current_pos += 1; // Move past opening quote

                while self.current_pos < self.input.len() && self.current_char() != Option::from('"') {
                    self.current_pos += 1;
                }
                let variable_value = self.input[start_pos..self.current_pos].to_string();
                self.current_pos += 1; // Move past closing quote
                Ok(Token::Variable(variable_name.to_string(), VariableValue::String(variable_value)))
            } else {
                // Assume the value is an integer
                while self.current_pos < self.input.len() && self.input[self.current_pos..].chars().next().unwrap().is_digit(10) {
                    self.current_pos += 1;
                }
                let variable_value = self.input[start_pos..self.current_pos].parse::<i32>().unwrap();
                Ok(Token::Variable(variable_name.to_string(), VariableValue::Integer(variable_value as i64)))
            };
        }

        // Handle cases where '=' is not present
        Ok(Token::Variable(variable_name.to_string(), VariableValue::Undefined))
    }

    fn handle_print(&mut self) -> Result<Token, String> {
        self.current_pos += 5; // Move past "print"
        self.skip_whitespace();

        if let Some('(') = self.current_char() {
            self.current_pos += 1; // Move past '('
            let start_pos = self.current_pos;
            let mut bracket_count = 1;

            while let Some(ch) = self.current_char() {
                if ch == '(' {
                    bracket_count += 1;
                } else if ch == ')' {
                    bracket_count -= 1;
                    if bracket_count == 0 {
                        break;
                    }
                }
                self.current_pos += 1;
            }

            if self.current_char().is_none() {
                return Err("Expected ')'".to_string());
            }

            let end_pos = self.current_pos;
            self.current_pos += 1; // Move past ')'

            let args_str = &self.input[start_pos..end_pos];
            let args = self.extract_args(args_str);

            return Ok(Token::PrintWithArgument(args[0].clone(), args));
        }
        Ok(Token::Print)
    }
    fn handle_funky(&mut self) -> Result<Token, String> {
        // Move past "funky"
        self.current_pos += 5;
        self.skip_whitespace();

        // State machine for parsing
        enum State {
            Name,
            Args,
            Body,
        }

        let mut state = State::Name;
        let mut function_name = String::new();
        let mut args = Vec::new();
        let mut body = String::new();
        let mut bracket_count = 0;

        while self.current_pos < self.input.len() {
            match state {
                State::Name => {
                    if let Some(ch) = self.current_char() {
                        if ch == '(' {
                            state = State::Args;
                            self.current_pos += 1;
                            self.skip_whitespace();
                        } else {
                            function_name.push(ch);
                            self.current_pos += 1;
                        }
                    }
                }
                State::Args => {
                    if let Some(ch) = self.current_char() {
                        if ch == ')' {
                            state = State::Body;
                            self.current_pos += 1;
                            self.skip_whitespace();
                        } else {
                            args.push(ch.to_string());
                            self.current_pos += 1;
                        }
                    }
                }
                State::Body => {
                    if let Some(ch) = self.current_char() {
                        if ch == '{' {
                            bracket_count += 1;
                        } else if ch == '}' {
                            bracket_count -= 1;
                            if bracket_count == 0 {
                                self.current_pos += 1;
                                break;
                            }
                        }
                        body.push(ch);
                        self.current_pos += 1;
                    }
                }
            }
        }

        if bracket_count != 0 {
            return Err("Unmatched curly braces in function body".to_string());
        }

        Ok(Token::FunctionDefinition(function_name.trim().to_string(), body.trim().to_string(), args))
    }

    // Handles function calls
    fn handle_fn_call(&mut self) -> Result<Token, String> {
        // get the start position
        let start_pos = self.current_pos;
        // go back till '\n'
        while self.current_pos > 0 && self.input[self.current_pos..].chars().next().unwrap() != '\n' {
            self.current_pos -= 1;
        }
        // move past '\n'
        self.current_pos += 1;
        let function_name = &self.input[self.current_pos..start_pos];
        println!("Function name: {:?}", function_name);
        Ok(Token::FunctionCall(function_name.to_string(), vec![]))
    }
}