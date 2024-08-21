use crate::lexer::token::Token;

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

        let c = self.current_char();

        // Handle the 'print' keyword
        if self.input[self.current_pos..].starts_with("print") {
            // Skip the 'print' keyword (5 characters) and the ( character, so 6 characters in total
            self.current_pos += 6;
            if self.current_char() == '"' {
                self.current_pos += 1; // Skip the opening quote
                let start_pos = self.current_pos;
                // Find the closing quote
                while self.current_char() != '"' && self.current_pos < self.input.len() {
                    self.current_pos += 1;
                }
                // Get the string between the quotes
                let argument = &self.input[start_pos..self.current_pos];
                self.current_pos += 1; // Skip the closing quote
                return Token::PrintWithArgument(argument.to_string());
            }
            return Token::Print;
        }
        // Handle the 'var' keyword
        if self.input[self.current_pos..].starts_with("var") {
            self.current_pos += 3;
            self.skip_whitespace();
            // Parse the variable name
            // get the start position of the variable name
            let start_pos = self.current_pos;
            // get the end position of the variable name
            while self.current_pos < self.input.len() && self.input[self.current_pos..].chars().next().unwrap().is_alphanumeric() {
                self.current_pos += 1;
            }
            // get the variable name
            let variable_name = &self.input[start_pos..self.current_pos];
            // get the variable value
            self.skip_whitespace();
            if self.current_char() == '=' {
                self.current_pos += 1;
                self.skip_whitespace();
                let start_pos = self.current_pos;
                while self.current_pos < self.input.len() && self.input[self.current_pos..].chars().next().unwrap().is_digit(10) {
                    self.current_pos += 1;
                }
                let variable_value = self.input[start_pos..self.current_pos].parse().unwrap();
                return Token::Variable(variable_name.to_string(), variable_value);
            }
            return Token::Variable(variable_name.to_string(), 0);
        }

        // Skip unknown characters
        self.current_pos += 1;
        self.next_token()
    }

    fn current_char(&self) -> char {
        self.input[self.current_pos..].chars().next().unwrap()
    }

    fn skip_whitespace(&mut self) {
        while self.current_pos < self.input.len() && self.current_char().is_whitespace() {
            self.current_pos += 1;
        }
    }
}
