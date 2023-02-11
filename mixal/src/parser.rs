use crate::new_if_presudo_of;
use crate::MixInstructions;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

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


pub struct Parser<'a> {
    defined_symbols: DefinedSymbolTable,
    mix_instructions: MixInstructions<'a>,
}
impl<'a> Parser<'a> {
    pub fn new() -> Parser<'a> {
        Parser {
            defined_symbols: DefinedSymbolTable::new(),
            mix_instructions: MixInstructions::new(),
        }
    }
    pub fn parse(&self, lines: Vec<String>) {
        // let pr_lines = self.parse_program_lines(lines);
        // println!("{:#?}", pr_lines);
    }
}

