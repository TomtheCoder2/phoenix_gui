use crate::gui::tab_types::plotter::functions::Function;
use crate::gui::tab_types::plotter::functions::Function::*;
use crate::gui::tab_types::plotter::parser::Operation::Identifier;
use crate::gui::tab_types::plotter::parser::TokenType::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use strum_macros::{Display, EnumString, FromRepr};

// these are used for parsing and for the instructions for the vm
#[derive(
    PartialEq, Debug, Clone, Copy, Serialize, Deserialize, FromRepr, Display, EnumString, Default,
)]
#[strum(serialize_all = "snake_case")]
#[repr(u8)]
pub enum Operation {
    #[default]
    None,
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Negate,
    Factorial,
    Modulo,
    Constant(f64),
    OpenParenthesis,
    CloseParenthesis,
    Comma,
    EOF,
    Error,
    // index of the function and arity
    Call(u8, usize),
    Identifier,
    GetVar(usize),
}

/// Combines Operation and Function
#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize, FromRepr, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
#[repr(u8)]
pub enum TokenType {
    OperationToken(Operation),
    FunctionToken(Function),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub operation: TokenType,
    pub start_pos: usize,
    pub lexeme: String,
}

#[derive(Debug, Clone)]
pub struct Parser {
    cur_pos: usize,
    code: String,
    start_pos: usize,
    // maybe theres something like 3x and we want to convert that to 3*x, so we need to know if we are multiplying or not
    // so we always set mult to true if we the last token was a number or a variable, and then we check if the next token is a number, a variable, a function or a (
    mult: bool,
}

impl Parser {
    pub fn new(string: String) -> Self {
        Self {
            cur_pos: 0,
            code: string,
            start_pos: 0,
            mult: false,
        }
    }

    pub fn parse_function(&mut self) -> Result<Vec<Token>, String> {
        // parse eg f(x) = (cos(x^2) / x) ^ 2
        // we only care about the stuff after the equals sign, because that's the function, but the "f(x) = " is not in the string
        let function_string = self.code.clone();
        let function_string = function_string.trim().replace(' ', "");
        // we first want to parse the string into a vector of operations
        // we will then use this vector to create a tree of operations
        // we will then use this tree to create a function
        // we will then use this function to plot the function
        let mut tokens = Vec::new();
        self.code = function_string;
        self.cur_pos = 0;
        self.start_pos = 0;
        loop {
            let token = self.scan_token();
            if token.operation == OperationToken(Operation::EOF) {
                break;
            }
            tokens.push(token);
        }
        dbg!(&tokens);
        // self.operations = tokens;
        Ok(tokens)
    }

    fn number(&mut self) -> Token {
        if self.cur_pos > self.code.len() {
            return self.error_token("Unexpected end of file.");
        }
        while Self::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == b'.' && Self::is_digit(self.peek_next()) {
            self.advance(); // consume the .

            while Self::is_digit(self.peek()) {
                self.advance();
            }
        }

        self.create_token(OperationToken(Operation::Constant(
            self.code[self.start_pos..self.cur_pos]
                .parse::<f64>()
                .expect("Failed to parse float"),
        )))
    }

    fn peek_next(&self) -> u8 {
        if self.is_at_end() || self.cur_pos + 1 >= self.code.len() {
            return b'\0';
        }
        self.code.as_bytes()[self.cur_pos + 1]
    }

    pub fn scan_token(&mut self) -> Token {
        let was_pos = self.cur_pos;
        self.start_pos = self.cur_pos;

        if self.is_at_end() {
            return self.create_token(OperationToken(Operation::EOF));
        }

        let c = self.advance();

        if Self::is_digit(c) {
            self.mult = true;
            return self.number();
        }

        if Self::is_alpha(c) {
            if self.mult {
                self.mult = false;
                self.cur_pos = was_pos;
                return self.create_token(OperationToken(Operation::Multiply));
            }
            self.mult = true;
            return self.identifier();
        }

        let t = match c {
            // todo clean this up
            b'+' => self.create_token(OperationToken(Operation::Add)),
            b'-' => self.create_token(OperationToken(Operation::Subtract)),
            b'*' => self.create_token(OperationToken(Operation::Multiply)),
            b'/' => self.create_token(OperationToken(Operation::Divide)),
            b'^' => self.create_token(OperationToken(Operation::Power)),
            b'(' => {
                if self.mult {
                    self.mult = false;
                    self.cur_pos = was_pos;
                    self.create_token(OperationToken(Operation::Multiply))
                } else {
                    self.create_token(OperationToken(Operation::OpenParenthesis))
                }
            }
            b')' => self.create_token(OperationToken(Operation::CloseParenthesis)),
            b',' => self.create_token(OperationToken(Operation::Comma)),
            b'!' => self.create_token(OperationToken(Operation::Factorial)),
            b'%' => self.create_token(OperationToken(Operation::Modulo)),
            _ => self.error_token("Unexpected character"),
        };
        self.mult = false;
        t
    }

    fn identifier(&mut self) -> Token {
        if self.cur_pos > self.code.len() {
            return self.error_token("Unexpected end of file.");
        }
        while Self::is_alpha(self.peek()) || Self::is_digit(self.peek()) {
            self.advance();
        }

        let token_type = self.identifier_type();
        if token_type != OperationToken(Identifier) {
            self.mult = false;
        }
        self.create_token(token_type)
    }

    fn identifier_type(&self) -> TokenType {
        let c = self.code.as_bytes()[self.start_pos];
        // todo: create a macro for this
        return match c {
            // abs acos asin atan
            b'a' => {
                if self.cur_pos - self.start_pos > 1 {
                    // more than 1 char in this maybe keyword
                    // check if the length of the code is long enough
                    if self.code.len() < self.start_pos + 2 {
                        return OperationToken(Identifier);
                    }
                    match self.code.as_bytes()[self.start_pos + 1] {
                        b'b' => self.check_for_keyword(2, 1, "s", FunctionToken(Abs)),
                        b'c' => self.check_for_keyword(2, 2, "os", FunctionToken(Acos)),
                        b's' => self.check_for_keyword(2, 2, "in", FunctionToken(Asin)),
                        b't' => self.check_for_keyword(2, 2, "an", FunctionToken(Atan)),
                        _ => OperationToken(Identifier),
                    }
                } else {
                    OperationToken(Identifier)
                }
            }
            // floor fast_sqrt
            b'f' => {
                if self.cur_pos - self.start_pos > 1 {
                    // more than 1 char in this maybe keyword
                    // check if the length of the code is long enough
                    if self.code.len() < self.start_pos + 2 {
                        return OperationToken(Identifier);
                    }
                    match self.code.as_bytes()[self.start_pos + 1] {
                        b'l' => self.check_for_keyword(2, 3, "oor", FunctionToken(Floor)),
                        // b'a' => self.check_for_keyword(2, 7, "st_sqrt", FunctionToken(FastSqrt)),
                        _ => OperationToken(Identifier),
                    }
                } else {
                    OperationToken(Identifier)
                }
            }
            // round
            b'r' => self.check_for_keyword(1, 4, "ound", FunctionToken(Round)),
            // sin sqrt sinh
            b's' => {
                if self.cur_pos - self.start_pos > 1 {
                    // more than 1 char in this maybe keyword
                    // check if the length of the code is long enough
                    if self.code.len() < self.start_pos + 2 {
                        return OperationToken(Identifier);
                    }
                    match self.code.as_bytes()[self.start_pos + 1] {
                        b'i' => {
                            // could be sin or sinh
                            // check if there is a third char
                            if self.code.len() < self.start_pos + 3 {
                                return OperationToken(Identifier);
                            }
                            if self.code.as_bytes()[self.start_pos + 2] == b'n' {
                                if self.code.len() < self.start_pos + 4 {
                                    return OperationToken(Identifier);
                                }
                                if self.code.as_bytes()[self.start_pos + 3] == b'h' {
                                    FunctionToken(Sinh)
                                } else {
                                    FunctionToken(Sin)
                                }
                            } else {
                                OperationToken(Identifier)
                            }
                        }
                        b'q' => self.check_for_keyword(2, 2, "rt", FunctionToken(Sqrt)),
                        _ => OperationToken(Identifier),
                    }
                } else {
                    OperationToken(Identifier)
                }
            }
            // cos ceil cosh
            b'c' => {
                if self.cur_pos - self.start_pos > 1 {
                    // more than 1 char in this maybe keyword
                    // check if the length of the code is long enough
                    if self.code.len() < self.start_pos + 2 {
                        return OperationToken(Identifier);
                    }
                    match self.code.as_bytes()[self.start_pos + 1] {
                        b'o' => {
                            // could be cos or cosh
                            // check if there is a third char
                            if self.code.len() < self.start_pos + 3 {
                                return OperationToken(Identifier);
                            }
                            if self.code.as_bytes()[self.start_pos + 2] == b's' {
                                if self.code.len() < self.start_pos + 4 {
                                    return OperationToken(Identifier);
                                }
                                if self.code.as_bytes()[self.start_pos + 3] == b'h' {
                                    FunctionToken(Cosh)
                                } else {
                                    FunctionToken(Cos)
                                }
                            } else {
                                OperationToken(Identifier)
                            }
                        }
                        b'e' => self.check_for_keyword(2, 2, "il", FunctionToken(Ceil)),
                        _ => OperationToken(Identifier),
                    }
                } else {
                    OperationToken(Identifier)
                }
            }
            // tan trunc tanh
            b't' => {
                if self.cur_pos - self.start_pos > 1 {
                    // more than 1 char in this maybe keyword
                    // check if the length of the code is long enough
                    if self.code.len() < self.start_pos + 2 {
                        return OperationToken(Identifier);
                    }
                    match self.code.as_bytes()[self.start_pos + 1] {
                        b'r' => self.check_for_keyword(2, 2, "unc", FunctionToken(Trunc)),
                        b'a' => {
                            // could be tan or tanh
                            // check if there is a third char
                            if self.code.len() < self.start_pos + 3 {
                                return OperationToken(Identifier);
                            }
                            if self.code.as_bytes()[self.start_pos + 2] == b'n' {
                                if self.code.len() < self.start_pos + 4 {
                                    return OperationToken(Identifier);
                                }
                                if self.code.as_bytes()[self.start_pos + 3] == b'h' {
                                    FunctionToken(Tanh)
                                } else {
                                    FunctionToken(Tan)
                                }
                            } else {
                                OperationToken(Identifier)
                            }
                        }
                        _ => OperationToken(Identifier),
                    }
                } else {
                    OperationToken(Identifier)
                }
            }
            // ln log
            b'l' => {
                if self.cur_pos - self.start_pos > 1 {
                    // more than 1 char in this maybe keyword
                    match self.code.as_bytes()[self.start_pos + 1] {
                        b'n' => self.check_for_keyword(2, 0, "", FunctionToken(Ln)),
                        b'o' => self.check_for_keyword(2, 1, "g", FunctionToken(Log)),
                        _ => OperationToken(Identifier),
                    }
                } else {
                    OperationToken(Identifier)
                }
            }
            // pi
            b'p' => self.check_for_keyword(
                1,
                1,
                "i",
                OperationToken(Operation::Constant(std::f64::consts::PI)),
            ),
            // e exp
            b'e' => {
                if self.cur_pos - self.start_pos > 1 {
                    // more than 1 char in this maybe keyword
                    match self.code.as_bytes()[self.start_pos + 1] {
                        b'x' => self.check_for_keyword(2, 1, "p", FunctionToken(Exp)),
                        _ => OperationToken(Identifier),
                    }
                } else {
                    self.check_for_keyword(
                        1,
                        0,
                        "",
                        OperationToken(Operation::Constant(std::f64::consts::E)),
                    )
                }
            }
            _ => OperationToken(Identifier),
        };
    }

    fn check_for_keyword(
        &self,
        start: usize,
        length: usize,
        rest: &str,
        keyword_type: TokenType,
    ) -> TokenType {
        if self.cur_pos - self.start_pos == start + length {
            // this will check that begin + length is within the array, since we already moved cur_pos exactly that far
            let begin = self.start_pos + start;
            // check if the code is long enough
            if self.code.len() < begin + length {
                return OperationToken(Identifier);
            }
            if &self.code[begin..begin + length] == rest {
                return keyword_type;
            }
        }
        OperationToken(Identifier)
    }

    fn error_token(&self, message: &str) -> Token {
        Token {
            operation: OperationToken(Operation::Error),
            lexeme: message.to_string(),
            start_pos: self.start_pos,
        }
    }

    fn is_alpha(c: u8) -> bool {
        c.is_ascii_lowercase() || c.is_ascii_uppercase() || c == b'_'
    }

    fn is_digit(c: u8) -> bool {
        c.is_ascii_digit()
    }

    fn create_token(&mut self, token_type: TokenType) -> Token {
        Token {
            operation: token_type,
            start_pos: self.start_pos,
            lexeme: self.code[self.start_pos..self.cur_pos].to_string(),
        }
    }

    fn advance(&mut self) -> u8 {
        let ret = self.peek();
        self.cur_pos += 1;
        ret
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            return b'\0';
        }
        self.code.as_bytes()[self.cur_pos]
    }

    fn is_at_end(&self) -> bool {
        self.cur_pos >= self.code.len()
    }
}
