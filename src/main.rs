mod asm;
mod interpreter;
mod parser;

use reformation::Reformation;
use asm::{Asm, compile_asm, compile_opt};

fn main() {
    let source = std::fs::read_to_string("./bubble.s").unwrap();
    let result: Vec<_> = source.lines().map(|x| Asm::parse(x).unwrap()).collect();
    let opt = compile_asm(&result);
    println!("{:#?}", result);
    println!("{:#?}", opt);
    println!("{}", compile_opt(&opt));
}
