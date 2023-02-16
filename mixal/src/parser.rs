use crate::lexer::token::*;
use crate::lexer::*;
use crate::tags::*;
use crate::word::*;
use crate::parser::addr_parser::*;

use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

pub mod addr_parser;
pub mod word;

/*
 * program      -> lines
 * lines        -> lines line | e
 * line         -> loc action description||\n
 *
 * loc          -> * |   | id | local_symbol
 *
 * action       -> mix_action | mixal_action
 * mix_action   -> mix_op address
 * mixal_action -> mixal_op W_value
 *
 * address      -> A_part||I_part||F_part
 *
 * A_part  -> expr | local_symbol | literal_constant | e
 * I_part  -> ,expr | e
 * F_part  -> (expr) | e
 * W_value -> expr||F_part | ,W_value
 */

/*
 * literal_constant -> =W_value=
 *
 * expr      -> expr+atom_expr | expr-atom_expr |
 *              expr*atom_expr | expr+atom_expr |
 *              expr/atom_expr | expr//atom_expr |
 *              expr:atom_expr |
 *              unary
 * unary     -> -atom_expr | +atom_expr | atom_expr
 * atom_expr -> num | id | *
 */

/*
 * mix_op   -> NOP | ... | CMPX
 * mixal_op -> EQU | ORIG | CON | ALF | END
 *
 * local_symbol -> num||H | num||B | num||F
 *
 * id          -> chars | symbol
 * symbol      -> chars||num | num||chars
 * num         -> 0|1|2|3|4|5|6|7|8|9
 * chars       -> A..Z
 * description -> chars||num | num||chars | e
 **/

pub trait Printable {
    fn print(&self) -> String;
}

pub struct SymbolTable {
    name_lines: HashMap<String, Vec<i32>>,
    name_value: HashMap<String, i32>,
}
impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            name_lines: HashMap::new(),
            name_value: HashMap::new(),
        }
    }
    pub fn get(&self, name: &str) -> i32 {
        return match self.name_value.get(name) {
            Some(x) => *x,
            None => self
                .name_lines
                .get(name)
                .expect(&format!("symbol not found {name}")[..])[0], //TODO: need implement for 2H 2B 2F
        };
    }

    pub fn put(&mut self, name: &str, line: i32) {
        let mut lines = match self.name_lines.get(name) {
            Some(lines) => lines.to_vec(),
            None => Vec::new(),
        };

        lines.push(line);
        self.name_lines.insert(name.to_string(), lines);
    }
}

pub struct Parser<'a> {
    symbols: SymbolTable,
    mix_instructions: MixInstructions<'a>,
}
impl<'a> Parser<'a> {
    pub fn new() -> Parser<'a> {
        Parser {
            symbols: SymbolTable::new(),
            mix_instructions: MixInstructions::new(),
        }
    }
    pub fn parse(&mut self, lines: Vec<ProgramLine>) {
        self.fill_symbol_table(&lines);

        let mut i = 0;
        for line in lines {
            match line.op.get_tag() {
                Tag::MIX_OP => {
                    // A I F
                }

                Tag::MIXAL_OP => {
                    // W_value
                }
                _ => {
                    panic!("unsupported operation {:#?}", line.op.get_tag());
                }
            }

            i += 1;
        }
    }

    pub fn fill_symbol_table(&mut self, lines: &Vec<ProgramLine>) {
        //TODO: this could be done in one, main cycle
        let mut i = 0;
        for line in lines {
            if line.loc.get_tag() == Tag::SYMBOLS {
                self.symbols.put(&line.loc.get_symbols()[..], i);

                //todo: cat reduce the values
            }

            i += 1;
        }
    }
}


