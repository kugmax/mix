use crate::new_if_presudo_of;
use crate::tags::Tag;
use crate::MixInstructions;

use crate::lexer::token::*;

use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

pub mod token;

pub struct ProgramLine<'a> {
    loc: Token,
    op: OpToken<'a>,
    addr: Vec<Token>,
}
impl<'a> ProgramLine<'a> {
    pub fn new(loc: Token, op: OpToken<'a>, addr: Vec<Token>) -> ProgramLine<'a> {
        ProgramLine { loc, op, addr }
    }
}
impl fmt::Debug for ProgramLine<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut addr_str: String = "".to_string();
        for a in &self.addr {
            addr_str += &format!("{:#?}", a).to_string();
        }

        write!(f, "{:#?} {:#?} {}", self.loc, self.op, addr_str)
    }
}

pub struct Lexer {}
impl Lexer {
    pub fn new() -> Lexer {
        Lexer {}
    }

    pub fn parse_program_lines<'a>(
        &'a self,
        mix_inst: &'a MixInstructions,
        lines: Vec<String>,
    ) -> Vec<ProgramLine> {
        let mut result = Vec::new();

        for line in lines {
            if line.starts_with("*") {
                continue;
            }

            let (loc, line) = line.split_once(" ").expect("wrong line format");

            let (op, line) = split_whitespace_once(line);

            let mut op_token;

            let mut address = String::new();
            let pseudo_op = new_if_presudo_of(op, line);
            if pseudo_op.is_none() {
                let (left, _) = split_whitespace_once(line);
                address = left.to_string();
                let mix_op = mix_inst.get(op);
                op_token = OpToken::new_mix_op(Box::new(mix_op));
            } else {
                let pseudo_op = pseudo_op.expect("err");
                address = pseudo_op.parse_address();
                op_token = OpToken::new_mixal_op(pseudo_op);
            }

            let pr_line = ProgramLine::new(
                Token::new_symbols(loc.to_string()),
                op_token,
                self.parse_address(address),
            );
            println!("{:#?}", &pr_line);
            result.push(pr_line);
        }

        result
    }

    fn parse_address(&self, address: String) -> Vec<Token> {
        let mut result = Vec::new();
        let mut chars = address.chars();

        let mut ch = chars.next();
        loop {
            match ch {
                None => break,
                Some(c) => match c {
                    '-' => {
                        result.push(Token::new(Tag::MINUS, c.to_string()));
                    }
                    '+' => {
                        result.push(Token::new(Tag::PLUS, c.to_string()));
                    }
                    '*' => {
                        result.push(Token::new(Tag::MULTIPLY, c.to_string()));
                    }
                    ',' => {
                        result.push(Token::new(Tag::COMMA, c.to_string()));
                    }
                    '=' => {
                        result.push(Token::new(Tag::EQUAL, c.to_string()));
                    }
                    '(' => {
                        result.push(Token::new(Tag::OPEN_BR, c.to_string()));
                    }
                    ')' => {
                        result.push(Token::new(Tag::CLOSE_BR, c.to_string()));
                    }
                    '/' => {
                        ch = chars.next();
                        if ch == Some('/') {
                            result.push(Token::new(Tag::MOD, "//".to_string()));
                            continue;
                        } else {
                            result.push(Token::new(Tag::DEVIDE, c.to_string()));
                        }
                    }
                    _ => {
                        let mut c = c;
                        let mut is_number = true;
                        let mut symbols = String::new();

                        loop {
                            if (!c.is_alphanumeric() && c != ' ') {
                                break;
                            }
                            if (!c.is_numeric()) {
                                is_number = false;
                            }

                            symbols += &c.to_string()[..];

                            ch = chars.next();
                            if ch == None {
                                break;
                            } else {
                                c = ch.expect("error");
                            }
                        }
                        if !symbols.is_empty() {
                            if is_number {
                                let num = i32::from_str_radix(&symbols[..], 10)
                                    .expect("error parse number");
                                result.push(Token::new_number(num));
                            } else {
                                result.push(Token::new_symbols(symbols));
                            }
                            continue;
                        }
                    }
                },
            };
            ch = chars.next();
            // println!("{}", c);
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
        let mix_inst = MixInstructions::new();
        let lexer = Lexer::new();
        let sourse = read_programm("./programs/print_500_primes.mixal");
        lexer.parse_program_lines(&mix_inst, sourse);
    }
}
