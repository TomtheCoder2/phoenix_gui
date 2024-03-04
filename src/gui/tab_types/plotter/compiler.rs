use crate::gui::tab_types::plotter::functions::Function;
use crate::gui::tab_types::plotter::parser::TokenType::OperationToken;
use crate::gui::tab_types::plotter::parser::{Operation, Parser, Token};
use crate::gui::tab_types::plotter::precedence::{get_rule, ParseFn, Precedence};
use special_fun::FloatSpecial;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Compiler {
    pub instr: Vec<Operation>,
    parser: Parser,
    tokens: Vec<Token>,
    pub identifier_constants: Vec<String>,
    error_message: String,
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            instr: Vec::new(),
            parser: Parser::new("".to_string()),
            tokens: Vec::new(),
            identifier_constants: Vec::new(),
            error_message: String::new(),
        }
    }

    pub fn push(&mut self, operation: Operation) {
        self.instr.push(operation);
    }

    pub fn compile(&mut self, code: String) -> Result<(Vec<Operation>, Vec<String>), String> {
        let code = code.replace(' ', "");
        self.parser = Parser::new(code);
        // load first token
        self.tokens.push(self.parser.scan_token());
        self.expression();
        if self.error_message.is_empty() {
            Ok((self.instr.clone(), self.identifier_constants.clone()))
        } else {
            Err(self.error_message.clone())
        }
    }

    pub fn optimized_compile(
        &mut self,
        code: String,
    ) -> Result<(Vec<Operation>, Vec<String>), String> {
        self.compile(code)?;
        self.optimize()
    }

    fn advance(&mut self) {
        self.tokens.push(self.parser.scan_token());
        if self.current().operation == OperationToken(Operation::Error) {
            self.error(self.current().lexeme.clone().as_str());
            self.advance();
        }
    }

    pub fn match_cur(&mut self, operation: Operation) -> bool {
        if !self.check(operation) {
            false
        } else {
            self.advance();
            true
        }
    }

    fn check(&self, operation: Operation) -> bool {
        self.current().operation == OperationToken(operation)
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.tokens.len() - 2]
    }

    fn error(&mut self, error: &str) {
        // eprintln!("Error: {}", error);
        self.error_message
            .push_str(format!("Error: {} at {}\n", error, self.current().lexeme).as_str());
    }

    fn current(&self) -> &Token {
        &self.tokens[self.tokens.len() - 1]
    }

    fn parse_precedence(&mut self, prec: Precedence) {
        // println!("Parsing precedence: {:?}", prec);
        self.advance();

        // Parse the start of the prefix expression
        // We know this must be a prefix because we can't start with something that is an infix (eg + 3 2)
        let prefix_rule = (get_rule(self.previous().operation)).prefix;
        self.call_parse_fn(prefix_rule);

        // Parse any number of infix expressions, as long as they have higher precedence
        while prec <= get_rule(self.current().operation).precedence {
            self.advance();
            let infix_rule = (get_rule(self.previous().operation)).infix;
            self.call_parse_fn(infix_rule);
        }
    }

    fn call_parse_fn(&mut self, parse_fn: ParseFn) {
        match parse_fn {
            ParseFn::None => self.error("Expected expression"),
            ParseFn::Binary => self.binary(),
            ParseFn::Grouping => self.grouping(),
            ParseFn::Unary => self.unary(),
            ParseFn::Number => self.number(),
            ParseFn::Call => self.call(),
            ParseFn::FunctionCall(op) => self.function_call(op),
            ParseFn::Variable => self.variable(),
            ParseFn::Factorial => self.factorial(),
        }
    }

    fn factorial(&mut self) {
        self.push(Operation::Factorial);
    }

    fn variable(&mut self) {
        let name = &self.previous().lexeme.clone();
        let index = self.identifier_constant(name);
        self.push(Operation::GetVar(index));
    }

    /// Add a string to the chunk as a constant and return the index
    fn identifier_constant(&mut self, str_val: &String) -> usize {
        // self.add_constant(Value::PhoenixString(str_val.to_string()))
        match self.identifier_constants.iter().position(|x| x == str_val) {
            Some(i) => i,
            None => {
                self.identifier_constants.push(str_val.to_string());
                self.identifier_constants.len() - 1
            }
        }
    }

    fn function_call(&mut self, fun: Function) {
        self.consume(
            Operation::OpenParenthesis,
            "Expected '(' after function name",
        );
        // self.expression();
        // self.consume(Operation::CloseParenthesis, "Expected ')' after argument");
        let arg_count = self.argument_list();
        self.push(Operation::Call(fun as u8, arg_count));
    }

    /// Parses expressions while looking for commas between and for the closing paren. Leaves the values on the stack
    fn argument_list(&mut self) -> usize {
        let mut arg_count = 0;
        if !self.check(Operation::CloseParenthesis) {
            loop {
                self.expression();
                if arg_count == 255 {
                    self.error("Cannot have more than 255 arguments");
                }
                arg_count += 1;

                if !self.match_cur(Operation::Comma) {
                    break;
                }
            }
        }
        self.consume(
            Operation::CloseParenthesis,
            "Expected ')' after function argument list",
        );
        arg_count
    }

    fn number(&mut self) {
        // We trust that the scanner has given us something that looks like a number (124214.52)
        // BUT the scanner does NOT check the size, so this parse to f64 can still fail due to overflow

        if let Ok(value) = self.previous().lexeme.parse::<f64>() {
            self.push(Operation::Constant(value));
        } else {
            // check if its already a number
            if let OperationToken(Operation::Constant(value)) = self.previous().operation {
                self.push(Operation::Constant(value));
            } else {
                self.error("Invalid number");
            }
        }
    }

    fn call(&mut self) {
        // we only have functions with one argument for now
        // self.consume(Operation::CloseParenthesis, "Expected ')' after argument");
        // self.operation_tree.create_child_node(self.previous().operation);
        // self.push(Operation::Call);
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Term)
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(Operation::CloseParenthesis, "Expected ')' after expression");
    }

    fn unary(&mut self) {
        let operator_type = self.previous().operation;
        self.parse_precedence(Precedence::Unary); // evaluate the expression in the unary
        if operator_type == OperationToken(Operation::Subtract) {
            self.push(Operation::Negate)
        }
    }

    fn consume(&mut self, operation: Operation, msg: &str) {
        self.advance();
        if !(self.previous().operation == OperationToken(operation)) {
            self.error(msg);
        }
    }

    fn binary(&mut self) {
        let operator = self.previous().operation;
        let rule = get_rule(operator);
        self.parse_precedence(rule.next_precedence());
        match operator {
            OperationToken(operator_type) => match operator_type {
                Operation::Add => self.push(Operation::Add),
                Operation::Subtract => self.push(Operation::Subtract),
                Operation::Multiply => self.push(Operation::Multiply),
                Operation::Divide => self.push(Operation::Divide),
                Operation::Power => self.push(Operation::Power),
                _ => {
                    self.error("Invalid binary operator");
                }
            },
            _ => {
                self.error("Invalid binary operator");
            }
        }
    }

    /// Pre-computes all operations that dont involve a variable
    // todo: optimize this (other stack)
    pub fn optimize(&mut self) -> Result<(Vec<Operation>, Vec<String>), String> {
        // let start_len = self.instr.len();
        let instr = self.instr.clone();
        if instr.is_empty() {
            return Err("No instructions provided".to_string());
        }
        let mut current = 0;
        #[derive(Debug, Clone)]
        enum Value {
            Constant(f64),
            Operations(Vec<Operation>),
        }
        let mut stack: Vec<Value> = Vec::new();
        while current < instr.len() {
            let instr = instr[current];
            // println!("processing {:?}", instr);
            macro_rules! binary {
                ($op:tt, $opCode:tt) => {{
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    if let (Value::Constant(a), Value::Constant(b)) = (a.clone(), b.clone()) {
                        stack.push(Value::Constant(b $op a));
                    } else {
                        stack.push(b);
                        stack.push(a);
                        stack.push(Value::Operations(vec![Operation::$opCode]));
                    }
                }};
            }
            match instr {
                Operation::Add => binary!(+, Add),
                Operation::Subtract => binary!(-, Subtract),
                Operation::Multiply => binary!(*, Multiply),
                Operation::Divide => binary!(/, Divide),
                Operation::Power => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    if let (Value::Constant(a), Value::Constant(b)) = (a.clone(), b.clone()) {
                        stack.push(Value::Constant(a.powf(b)));
                    } else {
                        stack.push(b);
                        stack.push(a);
                        stack.push(Value::Operations(vec![Operation::Power]));
                    }
                }
                Operation::Negate => {
                    let a = stack.pop().unwrap();
                    if let Value::Constant(a) = a {
                        stack.push(Value::Constant(-a));
                    } else {
                        stack.push(a);
                        stack.push(Value::Operations(vec![Operation::Negate]));
                    }
                }
                Operation::Factorial => {
                    let a = stack.pop().unwrap();
                    if let Value::Constant(a) = a {
                        stack.push(Value::Constant(a.factorial()));
                    } else {
                        stack.push(a);
                        stack.push(Value::Operations(vec![Operation::Factorial]));
                    }
                }
                Operation::Modulo => binary!(%, Modulo),
                Operation::Constant(c) => {
                    stack.push(Value::Constant(c));
                }
                Operation::Call(index, arity) => {
                    let args = stack.split_off(stack.len() - arity);
                    if args.iter().any(|x| matches!(x, Value::Operations(_))) {
                        stack.extend(args);
                        stack.push(Value::Operations(vec![Operation::Call(index, arity)]));
                    } else {
                        let mut args = args
                            .into_iter()
                            .map(|x| match x {
                                Value::Constant(c) => c,
                                _ => panic!("Invalid argument"),
                            })
                            .collect::<Vec<_>>();
                        args.reverse();
                        let result = match Function::execute(index, &args) {
                            Ok(v) => v,
                            Err(e) => {
                                // eprintln!("Error executing function: {}", e);
                                return Err(e);
                            }
                        };
                        stack.push(Value::Constant(result));
                    }
                }
                Operation::GetVar(v) => {
                    stack.push(Value::Operations(vec![Operation::GetVar(v)]));
                    // new_instr.push(instr);
                }
                _ => return Err("Invalid instruction".to_string()),
            }
            current += 1;
        }
        // dbg!(stack.clone());
        self.instr = stack.iter().fold(vec![], |vector, x| match x {
            Value::Operations(v) => vector.into_iter().chain(v.clone()).collect(),
            Value::Constant(v) => vector
                .into_iter()
                .chain(vec![Operation::Constant(*v)])
                .collect(),
        });
        // println!("Optimized {} instructions to {}", start_len, self.instr.len());
        Ok((self.instr.clone(), self.identifier_constants.clone()))
    }

    // debug functions
    pub fn print_instructions(&self) {
        println!("======= Instructions: {} =======", self.instr.len());
        for (i, instr) in self.instr.iter().enumerate() {
            print!("{:2}| ", i);
            Self::print_op(instr);
        }
    }

    pub fn print_tokens(&self) {
        println!("======= Tokens: {} =======", self.tokens.len());
        for (i, token) in self.instr.iter().enumerate() {
            print!("{:2}| ", i);
            Self::print_op(token);
        }
    }

    fn print_op(token: &Operation) {
        match token {
            Operation::Constant(c) => println!("Constant: {}", c),
            Operation::Call(n, arity) => println!("Call: {} {}", n, arity),
            Operation::GetVar(v) => println!("GetVar: {}", v),
            _ => println!("{:?}", token),
        }
    }
}
