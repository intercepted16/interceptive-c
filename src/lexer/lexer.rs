use crate::lexer::token::{Token, VariableValue};

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
    } pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.current_pos >= self.input.len() {
            return Token::EOF;
        }

        // Handle the 'print' keyword
        if self.input[self.current_pos..].starts_with("print") {
            self.current_pos += 6;
            // get the input from the first bracket to the last bracket
            let start_pos = self.current_pos;
            let mut bracket_count = 1;
            while self.current_pos < self.input.len() {
                if self.current_char() == '(' {
                    bracket_count += 1;
                }
                else if self.current_char() == ')' {
                    bracket_count -= 1;
                    if bracket_count == 0 {
                        break;
                    }
                }
                self.current_pos += 1;
            }
            // Reassign the input to the string inside the brackets
            self.input = &self.input[start_pos..self.current_pos];
            self.current_pos = 0;
            // Now, check if there are any additional arguments
            let mut args = self.extract_args();
            while self.current_pos < self.input.len() {
                let start_pos = self.current_pos;
                while self.current_pos < self.input.len() && self.current_char() != ',' {
                    self.current_pos += 1;
                }
                args.push(self.input[start_pos..self.current_pos].to_string());
                self.current_pos += 1;
            }
            // now remove the additional arguments from the input because we don't need them anymore
            // to do this just remove after the first comma
            self.input = self.input.split(',').collect::<Vec<&str>>()[0];
            return Token::PrintWithArgument(self.input.to_string(), args);
            }
        // Handle the 'var' keyword
        if self.input[self.current_pos..].starts_with("var") {
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
            if self.current_char() == '=' {
                self.current_pos += 1; // Move past '='
                self.skip_whitespace();

                let start_pos = self.current_pos;

                // Check if the value is a string
                return if self.current_char() == '"' {
                    self.current_pos += 1; // Move past opening quote

                    while self.current_pos < self.input.len() && self.current_char() != '"' {
                        self.current_pos += 1;
                    }
                    let variable_value = self.input[start_pos..self.current_pos].to_string();
                    self.current_pos += 1; // Move past closing quote

                    Token::Variable(variable_name.to_string(), VariableValue::String(variable_value))
                } else {
                    // Assume the value is an integer
                    while self.current_pos < self.input.len() && self.input[self.current_pos..].chars().next().unwrap().is_digit(10) {
                        self.current_pos += 1;
                    }
                    let variable_value = self.input[start_pos..self.current_pos].parse::<i32>().unwrap();

                    Token::Variable(variable_name.to_string(), VariableValue::Integer(variable_value as i64))
                }
            }

            // Handle cases where '=' is not present
            return Token::Variable(variable_name.to_string(), VariableValue::Undefined); // Assuming Undefined is a variant in VariableValue
        }

        // Skip unknown characters
        self.current_pos += 1;
        self.next_token()
    }

    fn current_char(&self) -> char {
        self.input[self.current_pos..].chars().next().unwrap_or('\0')
    }

    fn skip_whitespace(&mut self) {
        while self.current_pos < self.input.len() && self.current_char().is_whitespace() {
            self.current_pos += 1;
        }
    }
    pub fn extract_args(&mut self) -> Vec<String> {
        let mut args = Vec::new();
        self.skip_whitespace(); // Skip leading whitespace before the arguments

        while self.current_pos < self.input.len() {
            let start_pos = self.current_pos;

            // Find the end of the current argument
            while self.current_pos < self.input.len() && self.current_char() != ',' {
                self.current_pos += 1;
            }

            // Extract the argument and trim leading/trailing whitespace
            let arg = self.input[start_pos..self.current_pos].trim().to_string();
            if !arg.is_empty() {
                args.push(arg);
            }

            // Move past the comma, if present
            if self.current_char() == ',' {
                self.current_pos += 1;
            }

            // Skip any additional whitespace after the comma
            self.skip_whitespace();
        }

        args
    }
}
