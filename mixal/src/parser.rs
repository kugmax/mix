use crate::new_if_presudo_of;
use crate::Tags;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

pub trait Printable {
    fn print(&self) -> String;
}

pub struct SymbolTable {
    values: HashMap<String, i32>,
}
impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            values: HashMap::new(),
        }
    }
    pub fn get(&self, name: &str) -> i32 {
        *self
            .values
            .get(name)
            .expect(&format!("symbol not found {name}")[..])
    }
    pub fn put(&mut self, name: &str, value: i32) {
        self.put(name, value);
    }
}

pub struct ProgramLine {
    loc: String,
    op: String,
    addr: String,
}
impl ProgramLine {
    pub fn new(loc: String, op: String, addr: String) -> ProgramLine {
        ProgramLine { loc, op, addr }
    }
}

pub struct Parser {}
impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }

    pub fn parse(&self, lines: Vec<String>) {
        let symbols = SymbolTable::new();
        let tags = Tags::new();

        for line in lines {
            if line.starts_with("*") {
                continue;
            }

            let (loc, line) = line.split_once(" ").expect("wrong line format");

            let (op, line) = split_whitespace_once(line);

            let mut address = String::new();
            let pseudo_op = new_if_presudo_of(op, line);
            if pseudo_op.is_none() {
                if !tags.is_instruction(op) {
                    panic!("unsupported operation {op}");
                }
                let (left, _) = split_whitespace_once(line);
                address = left.to_string();
            } else {
                address = pseudo_op.expect("err").parse_address();
            }

            println!("{loc}|{op}|{address}|");
        }
    }
}

pub fn split_whitespace_once(line: &str) -> (&str, &str) {
    return match line.split_once(" ") {
        None => (line, ""),
        Some((op, line)) => (op, line),
    };
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let parser = Parser::new();

        let sourse = read_programm("./programs/print_500_primes.mixal");

        parser.parse(sourse);
    }
}
