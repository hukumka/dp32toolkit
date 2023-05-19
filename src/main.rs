#![feature(bigint_helper_methods)]

mod asm;
mod interpreter;

use reformation::Reformation;
use asm::{Asm, compile_asm, compile_opt, Instruction};
use interpreter::Interpreter;

fn main() {
    let source = std::fs::read_to_string("./bubble.s").unwrap();
    let result: Vec<_> = source.lines().map(|x| Asm::parse(x).unwrap()).collect();
    let opt = compile_asm(&result);
    println!("{}", compile_opt(&opt));
    run_simulation(opt);
}

fn run_simulation(code: Vec<Instruction>) {
    let mut interpreter = Interpreter::new(code, 1024);
    interpreter.memory[0..10].clone_from_slice(&[5, 3, -8, 1023, 6, 3, 9, 0, 99, 1]);
    interpreter.rx[1] = 0;
    interpreter.rx[2] = 10;

    interpreter.run();

    println!("{:?}", &interpreter.memory[0..10]);
}
