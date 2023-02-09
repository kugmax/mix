use crate::new_if_presudo_of;
use crate::Tags;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

pub mod word;

/*
 * program -> lines
 * lines -> lines line | e
 * line -> loc op address
 * 
 * loc -> id | local_symbol | e 
 * op -> mix_op | mixal_op
 * address -> A_part||I_part||F_part | W_value 
 *
 * A_part -> expr | local_symbol | literal_constant | e
 * I_part -> ,expr | e
 * F_part -> (expr) | e
 */

/*
 * literal_constant -> =W_value=
 * W_value -> expr||F_part | ,W_value 
 * //TODO: here recusion like ***...
 * expr -> expr+atom_expr | expr-atom_expr | 
 *         expr*atom_expr | expr+atom_expr |
 *         expr/atom_expr | expr//atom_expr |
 *         expr:atom_expr | 
 *         unary
 * unary -> -atom_expr | +atom_expr | atom_expr
 * atom_expr -> num | id | *
 */

/*
 * mix_op -> NOP | ... | CMPX
 * mixal_op -> EQU | ORIG | CON | ALF | END
 *
 * local_symbol -> num||H | num||B | num||F
 *
 * id ->  chars | symbol 
 * symbol -> chars||num | num||chars
 * num -> 0|1|2|3|4|5|6|7|8|9
 * chars -> A..Z
 **/


pub trait Printable {
    fn print(&self) -> String;
}

pub struct DefinedSymbolTable {
    values: HashMap<String, i32>,
}
impl DefinedSymbolTable {
    pub fn new() -> DefinedSymbolTable {
        DefinedSymbolTable {
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
        self.values.insert(name.to_string(), value);
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
impl fmt::Debug for ProgramLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "|{}|{}|{}|", self.loc, self.op, self.addr)
    }
}

pub struct Parser<'a> {
    defined_symbols: DefinedSymbolTable,
    tags: Tags<'a>,
}
impl<'a> Parser<'a> {
    pub fn new() -> Parser<'a> {
        Parser {
            defined_symbols: DefinedSymbolTable::new(),
            tags: Tags::new(),
        }
    }
    pub fn parse(&self, lines: Vec<String>) {
        let pr_lines = self.parse_program_lines(lines);
        println!("{:#?}", pr_lines);
    }

    pub fn parse_program_lines(&self, lines: Vec<String>) -> Vec<ProgramLine> {
        let mut result = Vec::new();

        for line in lines {
            if line.starts_with("*") {
                continue;
            }

            let (loc, line) = line.split_once(" ").expect("wrong line format");

            let (op, line) = split_whitespace_once(line);

            let mut address = String::new();
            let pseudo_op = new_if_presudo_of(op, line);
            if pseudo_op.is_none() {
                if !self.tags.is_instruction(op) {
                    panic!("unsupported operation {op}");
                }
                let (left, _) = split_whitespace_once(line);
                address = left.to_string();
            } else {
                let pseudo_op = pseudo_op.expect("err");
                address = pseudo_op.parse_address();

                // match pseodo_op {
                    // EQU => self.defined_symbols.put(loc, address),
                // }
            }

            let pr_line = ProgramLine::new(loc.to_string(), op.to_string(), address.to_string());
            result.push(pr_line);
            // println!("{:#?}", pr_line);
        }

        result
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
