use crate::lexer::*;
use crate::parser::*;
use crate::parser::*;
use crate::pseudo_op::*;
use crate::tags::*;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

mod lexer;
mod parser;
mod pseudo_op;
mod tags;

fn main() {
    let mix_inst = MixInstructions::new();
    let lexer = Lexer::new();
    let sourse = read_programm("./programs/print_500_primes.mixal");

    let lexer = Lexer::new();
    let tokens = lexer.parse_program_lines(&mix_inst, sourse);

    let parser = Parser::new();

    let lines = parser.parse(tokens);
    println!("program len {}", lines.len());

    let path = "/home/max/Documents/Projects.git/mix/mixal/target/tmp.mix";
//              /home/max/Documents/Projects.git/mix/mixal
    // write_programm("./tmp.mix".to_string(), lines);
    let result = write_programm(path.to_string(), lines);
    println!("result {:#?}", result);
}

fn write_programm(path: String, lines: Vec<String>) -> io::Result<()> {
    // let mut file = File::create(path.to_string() + &IO_FILE_PREFIX.to_string() + &io_unit.to_string())?;

    let mut file = File::options().create(true).append(true).open(path)?;

    for line in lines {
        let mut line = line;
        line += &"\n";

        println!("{line}");
        file.write_all(&line.as_bytes())?;
    }

    Ok(())
}

fn read_programm(path: &str) -> Vec<String> {
    let mut file = File::open(path.to_string()).expect(&("file not found ".to_owned() + path));
    let mut reader = BufReader::new(file);
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
