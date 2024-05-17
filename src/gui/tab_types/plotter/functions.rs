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

/// github copilot code for gamma and factorial functions
fn gamma(x: f64) -> f64 {
    let p: [f64; 9] = [
        0.999_999_999_999_809_93,
        676.520_368_121_885_1,
        -1259.139_216_722_402_9,
        771.323_428_777_653_1,
        -176.615_029_162_140_6,
        12.507_343_278_677_0,
        -0.138_571_095_530_41,
        9.984_369_578_019_57e-6,
        1.505_632_735_149_31e-7,
    ];

    if x < 0.5 {
        std::f64::consts::PI / ((std::f64::consts::PI * x).sin() * gamma(1.0 - x))
    } else {
        let x = x - 1.0;
        let t = x + p.len() as f64 - 0.5;
        let mut y = p[0];
        for (i, &pi) in p.iter().enumerate().skip(1) {
            y += pi / (x + i as f64);
        }
        let sqrt_2pi = std::f64::consts::SQRT_2 * std::f64::consts::PI.sqrt();
        sqrt_2pi * t.powf(x + 0.5) * (-t).exp() * y
    }
}

pub fn factorial(n: f64) -> f64 {
    if n <= 1.0 {
        1.0
    } else {
        gamma(n + 1.0)
    }
}