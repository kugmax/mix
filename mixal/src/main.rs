use crate::parser::*;
use crate::pseudo_op::*;
use crate::tags::*;

mod lexer;
mod parser;
mod pseudo_op;
mod tags;

// tag
// token
// syntax tree
// symbol table
// statement
// exprassion
//
//
// 1. replace EQU
// 2. extend programm with =expression= -> CON
//
// need to have instruction parser +-AA,I(F)  +-AA,I(lF:rF)
// need to have table of local symbols (0-9)
//  - addresses 2H, 2B, 2F, * and
//  - arithmetic with them
//  - 2H can be either EQU or runtime address
//
//  HLT = END
//  * - to ignore
//
//  arithmetic is goin left to right -1+5*20/6 -> 4*20/6 -> 80/6
fn main() {
    let tags = MixInstructions::new();
    let mut inst = tags.get("LDA");

    println!("{:#?}", inst);
    inst.set_aa(123);
    println!("{:#?}", inst);
    println!("{:#?}", tags.get("LDA"));
}
