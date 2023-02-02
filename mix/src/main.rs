use std::env;
use crate::mix::MIX;

pub mod memory;
pub mod mix;
pub mod operations;
pub mod processor;
pub mod registers;

fn main() {
    let args: Vec<String> = env::args().collect();

    let program_path = &args[1];

    let mut mix = MIX::new();

    mix.load(program_path);
    mix.execute();
}
