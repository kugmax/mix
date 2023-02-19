use crate::lexer::token::*;
use crate::lexer::*;
use crate::parser::addr_parser::*;
use crate::pseudo_op::*;
use crate::tags::*;
use crate::word::*;

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

// #[derive(Debug, Copy, Clone)]
// pub struct CodeLine {
// addr: u32,
// word: Word,
// }
// impl CodeLine {
// fn new(addr: u32, word: Word) -> CodeLine {
// CodeLine {
// addr: addr,
// word: word,
// }
// }
// }
// pub struct CodeLines {
// program_start: u32,
// lines: HashMap<u32, Vec<CodeLine>>,
// blocks: Vec<u32>,
// }
// impl CodeLines {
// fn new() -> CodeLines {
// CodeLines {
// program_start: 0,
// lines: HashMap::new(),
// blocks: Vec::new(),
// }
// }
// fn set_program_start(&mut self, program_start: u32) {
// self.program_start = program_start;
// }
// fn start_new_block(&mut self, new_block_addr: u32) {
// self.blocks.push(new_block_addr);
// }
// fn add_code_line(&mut self, word: Word) {
// let last_block_addr: u32 = if self.blocks.len() == 0 {0} else {
// *self.blocks.get(self.blocks.len()-1).expect("error")
// }
//
// let code_lines = match self.lines.get(&last_block_addr) {
// None => Vec::new(),
// Some(v) => v.to_vec()
// };
//
// let next_addr = last_block_addr + code_lines.len() as u32;
// code_lines.push(CodeLine::new(next_addr, word));
// self.lines.insert(last_block_addr, code_lines);
// }
// }
// impl Printable for CodeLines {
// fn print(&self) -> String {
// "NOT IMPLEMENTED".to_string()
// }
// }

//TODO: if loc exists and I know the address then fill the symbol table
// - table for equ values name-value
// - table for references name-address
// - table for local symbols 2H... name-address-program line numbers (*)
// - for addr parser has to be simple api for all 3 tables (put,get)

pub struct SymbolTable {
    equ_values: HashMap<String, Word>,
    references: HashMap<String, i32>,
    local_symbols: HashMap<String, Vec<i32>>,

}
impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            equ_values: HashMap::new(),
            references: HashMap::new(), 
            local_symbols: HashMap::new(), 
        }
    }
    pub fn get(&self, name: String) -> i32 {
        if self.is_local_symbol(&name) {
            //TODO: literal constant
        }

        return match self.equ_values.get(&name) {
            Some(v) => v.get_signed_value(),
            None => match self.references.get(&name) {
                Some(v) => *v,
                None => panic!("symbol {} not found", name),
            },
        };
    }

    pub fn put_equ(&mut self, name: String, value: Word) {
        self.equ_values.insert(name, value);
    }

    pub fn put_reference(&mut self, name: String, address: u32) {
        if self.is_local_symbol(&name) {
            self.put_local_symbol(name, address);
            return;
        }
        self.references.insert(name, address as i32);
    }
    fn put_local_symbol(&mut, name: String, address: u32) {
          let local_symbols_addrs = match self.local_symbols.get(name) {
            None => Vec::new(),
            Some(v) => v.to_vec()
          }
    }

    fn is_local_symbol(&self, name: &String) -> bool {
        if name.len() == 2 {
            let mut chars = name.chars();
            let digit = chars.next().expect("error");
            let direction = chars.next().expect("error");

            return digit.is_numeric()
                && (direction == 'H' || direction == 'B' || direction == 'F');
        }

        return false;
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
    //1. Cycle 1
    //  - reduce w_value for mixal
    //  - process not printable mixal operations
    //  - for mix operations reduce line address (ORIG)
    //  - implement literal constant = = -> con1 CON ...
    //  - remove not printable  mixal operation, in cycle 2 there are only printable operations
    pub fn parse_not_printable(&mut self, mut lines: Vec<ProgramLine>) {
        let mut program_start_addr = 0;
        let mut mix_lines: Vec<ProgramLine> = Vec::new();
        let mut addresses: Vec<u32> = Vec::new();

        let mut l_con_inx = 1;
        let mut addr = 0;
        let mut line_num = 0;

        let mut i = 0;
        loop {
            if i >= lines.len() {
                break;
            }

            let mut line = lines.get(i).expect("error");

            if !line.loc.get_symbols().is_empty() {
                self.symbols.put_reference(line.loc.get_symbols(), addr);
            }

            match line.op.get_tag() {
                Tag::MIX_OP => match line.addr.get(0) {
                    None => {
                        mix_lines.push(line.clone());
                        addresses.push(addr);
                        addr += 1;
                        line_num += 1;
                    }
                    Some(t) => match t.get_tag() {
                        Tag::EQUAL => {
                            let mut add_parser =
                                AddrParser::new(&self.symbols, line_num, &line.addr);
                            let con_word = w_value_to_word(add_parser.literal_constant());
                            let con_loc = format!("con{}", l_con_inx);
                            l_con_inx += 1;

                            let line_con = ProgramLine::new(
                                line.loc.clone(),
                                line.op.clone(),
                                Vec::from([Token::new_symbols(con_loc.clone())]),
                            );
                            mix_lines.push(line_con);
                            addresses.push(addr);
                            addr += 1;
                            line_num += 1;

                            let con_value = con_word.get_signed_value();
                            let con = ProgramLine::new(
                                Token::new_symbols(con_loc),
                                OpToken::new_mixal_op(MixalOp::new(
                                    "CON".to_string(),
                                    con_value.to_string(),
                                )),
                                Vec::from([Token::new_number(con_value)]),
                            );

                            let indx_before_end = lines.len() - 2;
                            lines.insert(indx_before_end as usize, con);
                        }
                        _ => {
                            mix_lines.push(line.clone());
                            addresses.push(addr);
                            addr += 1;
                            line_num += 1;
                        }
                    },
                },

                Tag::MIXAL_OP => {
                    let mut add_parser = AddrParser::new(&self.symbols, line_num, &line.addr);
                    match &line.op.get_mixal_op().get_name()[..] {
                        "EQU" => {
                            let w_values = add_parser.w_value(Vec::new());
                            self.symbols
                                .put_equ(line.loc.get_symbols(), w_value_to_word(w_values));
                        }
                        "ORIG" => {
                            let w_values = add_parser.w_value(Vec::new());
                            addr = w_value_to_word(w_values).get_signed_value() as u32;
                        }
                        "END" => {
                            let w_values = add_parser.w_value(Vec::new());
                            program_start_addr =
                                w_value_to_word(w_values).get_signed_value() as u32;
                        }
                        _ => {
                            // CON, ALF are printable
                            mix_lines.push(line.clone());
                            addresses.push(addr);
                            addr += 1;
                            line_num += 1;
                        }
                    }
                }
                _ => {
                    panic!("unsupported operation {:#?}", line.op.get_tag());
                }
            }

            i += 1;
        }
    }

    pub fn parse(&mut self, lines: Vec<ProgramLine>) {
        //TODO:
        //1. Cycle 1
        //  - reduce w_value for mixal
        //  - process all mixal operations
        //  - for mix operations reduce line address (ORIG)
        //  - implement literal constant = = -> con1 CON ...
        //  - remove all mixal operation, in cycle 2 there are only mix operations
        //2. Cycle 2 (here only mix, with line num, line addr)
        //  - reduce mix operations
        //  - implement local symbols 2H, 2B, 2F
    }
}
fn w_value_to_word(w_value: Vec<(Option<i32>, Option<i32>)>) -> Word {
    let mut result = Word::new(0);
    for (e, f) in w_value {
        match f {
            None => {
                result = Word::new_from_signed(e.expect("error"));
            }
            Some(spec) => {
                result.put_by_access(e.expect("error"), WordAccess::new_by_spec(spec as u8));
            }
        }
    }

    result
}
