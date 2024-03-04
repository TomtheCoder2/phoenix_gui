use phoenix_gui::gui::tab_types::plotter::compiler::Compiler;
use phoenix_gui::gui::tab_types::plotter::vm::VM;
use std::time::Instant;

fn main() {
    let code = "2+3+3*x+sin(pi*x^2/4)+4*tan(pi*x)+4+3";
    let code = "3*x+sin(pi*x^2/4)+4*tan(pi*x)+x^2+2*x+1+3*x+sin(pi*x^2/4)+4*tan(pi*x)+x^2+2*x+1+3*x+sin(pi*x^2/4)+4*tan(pi*x)+x^2+2*x+1+3*x+sin(pi*x^2/4)+4*tan(pi*x)+x^2+2*x+1";
    let code = "x^2+2*x+1";
    let mut compiler = Compiler::new();
    let mut instrs = compiler
        .compile(code.to_string())
        .expect("Could not compile");
    compiler.print_instructions();
    compiler.optimize().expect("Could not optimize");
    compiler.print_instructions();
    instrs.0 = compiler.instr;
    dbg!(compiler.identifier_constants);
    let n = 1000;
    let start = Instant::now();
    for i in 0..n {
        VM::run((&*instrs.0, &*instrs.1), &[i as f64]).expect("Could not run");
    }
    let duration = start.elapsed();
    println!("Total time: {:?}", duration);
    println!("Average time: {:?}", duration / n as u32);
    dbg!(VM::run((&*instrs.0, &*instrs.1), &[4.0]).expect("Could not run"));
}
