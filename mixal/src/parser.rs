use crate::lexer::token::*;
use crate::lexer::*;
use crate::parser::addr_parser::*;
use crate::parser::symbol_table::*;
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
pub mod symbol_table;
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

pub struct Parser {}
impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }
    //1. Cycle 1
    //  - reduce w_value for mixal
    //  - process not printable mixal operations
    //  - for mix operations reduce line address (ORIG)
    //  - implement literal constant = = -> con1 CON ...
    //  - remove not printable  mixal operation, in cycle 2 there are only printable operations
    pub fn parse_not_printable<'a>(
        symbols: &mut SymbolTable,
        lines: Vec<ProgramLine<'a>>
        ) -> (u32, Vec<ProgramLine<'a>>, Vec<u32>) {
        let mut lines = lines.to_vec();

        let mut program_start_addr = 0;
        let mut mix_lines: Vec<ProgramLine<'a>> = Vec::new();
        let mut addresses: Vec<u32> = Vec::new();

        let mut l_con_inx = 1;
        let mut addr = 0;
        let mut line_num = 0;

        let mut i = 0;
        loop {
            if i >= lines.len() {
                break;
            }

            let line = lines.get(i).expect("error");

            if !line.loc.get_symbols().is_empty() {
                symbols.put_reference(line.loc.get_symbols(), addr);
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
                                AddrParser::new(symbols, line_num, addr, &line.addr);
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

                            let indx_before_end = lines.len() - 1;
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
                    let mut add_parser = AddrParser::new(symbols, line_num, addr, &line.addr);
                    match &line.op.get_mixal_op().get_name()[..] {
                        "EQU" => {
                            let w_values = add_parser.w_value(Vec::new());
                            symbols.put_equ(line.loc.get_symbols(), w_value_to_word(w_values));
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
        (program_start_addr, mix_lines, addresses)
    }

    pub fn parse_printable(
        symbols: &mut SymbolTable,
        program_start_addr: u32,
        lines: Vec<ProgramLine>,
        addresses: Vec<u32>,
    ) -> Vec<String> {
        if lines.len() != addresses.len() {
            panic!("lines.len() != addresses.len()");
        }

        let mut program: Vec<String> = Vec::new();

        let mut line_num: u32 = 0;
        for line in lines {
            let addr = addresses.get(line_num as usize).expect("error");
            let mut printable_line = String::new();

            let mut add_parser = AddrParser::new(symbols, line_num, *addr, &line.addr);

            match line.op.get_tag() {
                Tag::MIX_OP => {
                    let (a_part, i_part, f_part) = add_parser.aif();
                    let mut instruction = *line.op.get_mix_op();

                    if a_part != None {
                        instruction.set_aa(a_part.expect("error set_aa"));
                    }
                    if i_part != None {
                        instruction.set_i(i_part.expect("error set_i") as u8);
                    }
                    if f_part != None {
                        instruction.set_f(f_part.expect("error set_f") as u8);
                    }
                    printable_line = format!("{addr}, {}", instruction.print());
                }
                Tag::MIXAL_OP => match &line.op.get_mixal_op().get_name()[..] {
                    "CON" => {
                        let w_values = add_parser.w_value(Vec::new());
                        let value_to_print = w_value_to_word(w_values).get_signed_value();

                        printable_line = format!("{addr}, {value_to_print}");
                    }
                    "ALF" => {
                        let bytes = line.op.get_mixal_op().alf_to_num();
                        printable_line = format!(
                            "{addr}, 0, {},{},{},{},{}",
                            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4]
                        );
                    }
                    mixal_op => {
                        panic!("unexpected mixal op operation {:#?}", mixal_op);
                    }
                },
                _ => {
                    panic!("unsupported operation {:#?}", line.op.get_tag());
                }
            }
            program.push(printable_line);

            line_num += 1;
        }
        program.push(program_start_addr.to_string());
        program
    }

    pub fn parse(&self, lines: Vec<ProgramLine>) -> Vec<String> {
        let mut symbols = SymbolTable::new();
        let (start, lines, addrs) = Parser::parse_not_printable(&mut symbols, lines);
        let lines = Parser::parse_printable(&mut symbols, start, lines, addrs);
        lines
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
