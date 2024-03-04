use crate::gui::tab_types::plotter::functions::Function;
use crate::gui::tab_types::plotter::parser::Operation;
use crate::gui::tab_types::plotter::stack::Stack;
use crate::gui::tab_types::plotter::stack::STACK_SIZE;
use special_fun::FloatSpecial;

pub struct VM {}

impl VM {
    pub fn run(result: (&[Operation], &[String]), values: &[f64]) -> Result<f64, String> {
        let instructions = result.0;
        let identifiers = result.1;
        if instructions.is_empty() {
            return Err("No instructions provided".to_string());
        }
        let mut stack = Stack::new();
        for instr in instructions {
            macro_rules! push {
                ( $ val: expr) => {{
                    if !stack.push($val) {
                        return Err(format!("Stack overflow (max: {})", STACK_SIZE));
                    }
                }};
            }
            macro_rules! pop {
                () => {{
                    let val = stack.pop();
                    if val.is_none() {
                        return Err("Stack underflow".to_string());
                    }
                    val.unwrap()
                }};
            }
            macro_rules! binary_op {
                ( $ op: tt) => {{
                    let a = pop!();
                    let b = pop!();
                    push!(b $ op a);
                }};
            }
            match *instr {
                Operation::None => {}
                Operation::Add => binary_op!( + ),
                Operation::Subtract => binary_op!( - ),
                Operation::Multiply => binary_op!( * ),
                Operation::Divide => binary_op!( / ),
                Operation::Power => {
                    let a = pop!();
                    let b = pop!();
                    push!(b.powf(a));
                }
                Operation::Negate => {
                    let a = pop!();
                    push!(-a);
                }
                Operation::Factorial => {
                    let a = pop!();
                    push!(a.factorial());
                }
                Operation::Modulo => binary_op!( % ),
                Operation::Constant(c) => push!(c),
                Operation::GetVar(index) => {
                    push!({
                        if index < values.len() {
                            values[index]
                        } else {
                            // eprintln!("Not enough values provided: {}", identifiers[index]);
                            return Err(format!("Undefined Variable: {}", identifiers[index]));
                        }
                    });
                }
                Operation::Call(index, arity) => {
                    let args_option = stack.pop_n(arity);
                    let mut args = Vec::with_capacity(arity);
                    for arg in args_option {
                        match arg {
                            Some(v) => args.push(v),
                            None => {
                                return Err(format!(
                                    "Not enough arguments provided for function: {}",
                                    Function::from_repr(index).unwrap().as_ref()
                                ));
                            }
                        }
                    }
                    args.reverse();
                    push!(match Function::execute(index, &args) {
                        Ok(v) => v,
                        Err(e) => {
                            // eprintln!("Error executing function: {}", e);
                            return Err(e);
                        }
                    });
                }
                // should never happen
                _ => return Err("Invalid instruction".to_string()),
            }
        }
        Ok(stack.pop().unwrap())
    }
}
