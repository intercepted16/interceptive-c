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
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.current_pos >= self.input.len() {
            return Token::EOF;
        }

        // Handle the 'print' keyword
        if self.input[self.current_pos..].starts_with("print") {
            self.current_pos += 5; // Move past "print"
            self.skip_whitespace();

            if self.current_char() == '(' {
                self.current_pos += 1; // Move past '('
                let start_pos = self.current_pos;
                let mut bracket_count = 1;

                while self.current_pos < self.input.len() {
                    if self.current_char() == '(' {
                        bracket_count += 1;
                    } else if self.current_char() == ')' {
                        bracket_count -= 1;
                        if bracket_count == 0 {
                            break;
                        }
                    }
                    self.current_pos += 1;
                }

                let end_pos = self.current_pos;
                self.current_pos += 1; // Move past ')'

                let args_str = &self.input[start_pos..end_pos];
                let args = self.extract_args_from_str(args_str);

                return Token::PrintWithArgument(args[0].clone(), args);
            }
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

                let mut start_pos = self.current_pos;

                // Check if the value is a string
                return if self.current_char() == '"' {
                    start_pos += 1; // Move past opening quote
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

    fn extract_args_from_str(&self, args_str: &str) -> Vec<String> {
        let mut args = Vec::new();
        let mut current_pos = 0;
        let input_len = args_str.len();

        while current_pos < input_len {
            let start_pos = current_pos;

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
}