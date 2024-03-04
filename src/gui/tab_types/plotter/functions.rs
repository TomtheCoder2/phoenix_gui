// use math::*;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, FromRepr};

#[derive(
    PartialEq,
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    FromRepr,
    Display,
    EnumString,
    AsRefStr,
    Default,
)]
#[strum(serialize_all = "snake_case")]
#[repr(u8)]
pub enum Function {
    #[default]
    None,
    Sin,
    Asin,
    Sinh,
    Cos,
    Acos,
    Cosh,
    Tan,
    Atan,
    Tanh,
    Ln,
    Log,
    Sqrt,
    // FastSqrt,
    Abs,
    Floor,
    Ceil,
    Round,
    Trunc,
    Exp,
}

impl Function {
    pub fn execute(index: u8, x: &[f64]) -> Result<f64, String> {
        let f = match Function::from_repr(index) {
            Some(f) => f,
            None => return Err(format!("Function with index {} does not exist", index)),
        };
        // macro for all the functions with only one input parameter
        macro_rules! execute {
            ($f:ident) => {{
                if x.len() != 1 {
                    Err(format!(
                        "Function {} expects {} arguments, but {} were given",
                        stringify!($f),
                        1,
                        x.len()
                    ))
                } else {
                    Ok(x[0].$f())
                }
            }};
        }
        match f {
            Function::Sin => execute!(sin),
            Function::Cos => execute!(cos),
            Function::Tan => execute!(tan),
            Function::Ln => execute!(ln),
            Function::Log => {
                if x.len() != 2 {
                    Err(format!(
                        "Function {} expects {} arguments, but {} were given",
                        stringify!(log),
                        2,
                        x.len()
                    ))
                } else {
                    // log(base, x)
                    Ok(x[1].log(x[0]))
                }
            }
            Function::Sqrt => execute!(sqrt),
            // Function::FastSqrt => execute!(fast_sqrt),
            Function::Abs => execute!(abs),
            Function::Floor => execute!(floor),
            Function::Ceil => execute!(ceil),
            Function::Round => execute!(round),
            Function::Trunc => execute!(trunc),
            Function::Asin => execute!(asin),
            Function::Sinh => execute!(sinh),
            Function::Acos => execute!(acos),
            Function::Cosh => execute!(cosh),
            Function::Atan => execute!(atan),
            Function::Tanh => execute!(tanh),
            Function::Exp => execute!(exp),
            Function::None => Err("No function provided".to_string()),
        }
    }
}
