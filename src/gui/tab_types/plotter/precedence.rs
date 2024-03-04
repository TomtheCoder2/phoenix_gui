use crate::gui::tab_types::plotter::functions::Function;
use crate::gui::tab_types::plotter::parser::{Operation, TokenType};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Precedence {
    None,
    Term,
    Unary,
    Factor,
    Power,
    Factorial,
    Call,
    Primary,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ParseFn {
    None,
    Unary,
    Factorial,
    Grouping,
    Number,
    Binary,
    Variable,
    FunctionCall(Function),
    Call,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParseRule {
    pub prefix: ParseFn,
    pub infix: ParseFn,
    pub precedence: Precedence,
}

impl ParseRule {
    pub fn next_precedence(&self) -> Precedence {
        match self.precedence {
            Precedence::None => Precedence::Term,
            Precedence::Term => Precedence::Unary,
            Precedence::Unary => Precedence::Factor,
            Precedence::Factor => Precedence::Power,
            Precedence::Power => Precedence::Factorial,
            Precedence::Factorial => Precedence::Call,
            Precedence::Call => Precedence::Primary,
            Precedence::Primary => Precedence::Primary,
        }
    }
}

const PARSE_RULE_NONE: ParseRule = ParseRule {
    prefix: ParseFn::None,
    infix: ParseFn::None,
    precedence: Precedence::None,
};

const PARSE_RULE_LP: ParseRule = ParseRule {
    prefix: ParseFn::Grouping,
    infix: ParseFn::Call,
    precedence: Precedence::Call,
};

const PARSE_RULE_MINUS: ParseRule = ParseRule {
    prefix: ParseFn::Unary,
    infix: ParseFn::Binary,
    precedence: Precedence::Term,
};

const PARSE_RULE_PLUS: ParseRule = ParseRule {
    prefix: ParseFn::None,
    infix: ParseFn::Binary,
    precedence: Precedence::Term,
};

const PARSE_RULE_SLASH: ParseRule = ParseRule {
    prefix: ParseFn::None,
    infix: ParseFn::Binary,
    precedence: Precedence::Factor,
};

const PARSE_RULE_STAR: ParseRule = ParseRule {
    prefix: ParseFn::None,
    infix: ParseFn::Binary,
    precedence: Precedence::Factor,
};

const PARSE_RULE_POWER: ParseRule = ParseRule {
    prefix: ParseFn::None,
    infix: ParseFn::Binary,
    precedence: Precedence::Power,
};

const PARSE_RULE_NUM: ParseRule = ParseRule {
    prefix: ParseFn::Number,
    infix: ParseFn::None,
    precedence: Precedence::None,
};

const PARSE_RULE_FAC: ParseRule = ParseRule {
    prefix: ParseFn::Grouping,
    infix: ParseFn::Factorial,
    precedence: Precedence::Factorial,
};

const PARSE_RULE_ID: ParseRule = ParseRule {
    prefix: ParseFn::Variable,
    infix: ParseFn::None,
    precedence: Precedence::None,
};

pub fn get_rule(token_type: TokenType) -> ParseRule {
    match token_type {
        TokenType::OperationToken(operator) => match operator {
            Operation::OpenParenthesis => PARSE_RULE_LP,
            Operation::Subtract => PARSE_RULE_MINUS,
            Operation::Add => PARSE_RULE_PLUS,
            Operation::Divide => PARSE_RULE_SLASH,
            Operation::Multiply => PARSE_RULE_STAR,
            Operation::Constant(_) => PARSE_RULE_NUM,
            Operation::Factorial => PARSE_RULE_FAC,
            Operation::Power => PARSE_RULE_POWER,
            Operation::Identifier => PARSE_RULE_ID,
            _ => PARSE_RULE_NONE,
        },
        TokenType::FunctionToken(function) => ParseRule {
            prefix: ParseFn::FunctionCall(function),
            infix: ParseFn::None,
            precedence: Precedence::None,
        },
    }
}
