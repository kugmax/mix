use crate::lexer::*;
use crate::parser::*;
use crate::parser::*;
use crate::pseudo_op::*;
use crate::tags::*;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

use std::env;

mod lexer;
mod parser;
mod pseudo_op;
mod tags;

fn main() {
    let args: Vec<String> = env::args().collect();

    let program_path = &args[1];
    if !program_path.ends_with(".mixal") {
        panic!("The MIXAL source program should end with '.mixal'")
    }

    compile(program_path);
}

fn compile(path: &str) {
    let mix_inst = MixInstructions::new();
    let sourse = read_programm(path);

    let lexer = Lexer::new();
    let tokens = lexer.parse_program_lines(&mix_inst, sourse);

    let parser = Parser::new();

    let lines = parser.parse(tokens);

    write_programm(path.to_string().replace(".mixal", ".mix"), lines);
}

fn write_programm(path: String, lines: Vec<String>) -> io::Result<()> {
    let mut file = File::options().create(true).append(true).open(path)?;

    for line in lines {
        let mut line = line;
        line += &"\n";

        // println!("{line}");
        file.write_all(&line.as_bytes())?;
    }

    Ok(())
}

fn read_programm(path: &str) -> Vec<String> {
    let file = File::open(path.to_string()).expect(&("file not found ".to_owned() + path));
    let reader = BufReader::new(file);
    let mut result = Vec::new();

    let lines = reader.lines();
    for line in lines {
        let line = line.expect("some err in lines");

        if line.len() <= 0 {
            continue;
        }
        result.push(line.to_string());

        // println!("{line}");
    }

    result
}
