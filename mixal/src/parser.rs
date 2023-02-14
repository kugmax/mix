use crate::lexer::token::*;
use crate::lexer::*;
use crate::tags::*;
use crate::word::*;

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

struct AddrParser<'a> {
    symbols: &'a SymbolTable,
    line_num: u32,

    tokens: &'a Vec<Token>,
    current: usize,
}
impl<'a> AddrParser<'a> {
    pub fn new(symbols: &'a SymbolTable, line_num: u32, tokens: &'a Vec<Token>) -> AddrParser<'a> {
        AddrParser {
            symbols: symbols,
            line_num: line_num,
            tokens: tokens,
            current: 0,
        }
    }

    pub fn parse(&self) -> i32 {
        //TODO: for ALF it would be something else
        0
    }

    fn lookahead(&mut self) -> Option<Token> {
        let result = match self.tokens.get(self.current) {
            None => None,
            Some(t) => Some(t.clone()),
        };
        result
    }
    fn next(&mut self) -> Option<Token> {
        let result = self.lookahead();
        self.current += 1;
        result
    }

    fn exprs(&mut self, acc: Option<i32>) -> Option<i32> {
        let left = match acc {
            None => self.expr().reduce(),
            Some(x) => acc,
        };

        let op = match self.next() {
            None => return left,
            Some(t) => t.get_tag(),
        };

        let right: Box<dyn Expr> = match self.next() {
            None => panic!("syntax error"),
            Some(t) => Box::new(self.unary(t.clone())),
        };

        let result = BinaryOp::new(op, Box::new(Holder::new(left)), right).reduce();
        self.exprs(result)
    }

    fn expr(&mut self) -> Box<dyn Expr> {
        let left: Box<dyn Expr> = match self.next() {
            None => return Box::new(EmptyExpr::new()),
            Some(t) => Box::new(self.unary(t.clone())),
        };

        let op = match self.next() {
            None => return left,
            Some(t) => t.get_tag(),
        };

        let right: Box<dyn Expr> = match self.next() {
            None => panic!("syntax error"),
            Some(t) => Box::new(self.unary(t.clone())),
        };

        Box::new(BinaryOp::new(op, left, right))
    }

    fn unary(&mut self, token: Token) -> UnaryOp {
        return match token.get_tag() {
            Tag::MINUS => {
                let next_t = self.next().expect("syntax error");
                UnaryOp::new(Tag::MINUS, Box::new(self.atom_expr(next_t)))
            }
            Tag::PLUS => {
                let next_t = self.next().expect("syntax error");
                UnaryOp::new(Tag::PLUS, Box::new(self.atom_expr(next_t)))
            }
            _ => UnaryOp::new(Tag::PLUS, Box::new(self.atom_expr(token))),
        };
    }

    fn atom_expr(&self, token: Token) -> Number {
        return match token.get_tag() {
            Tag::NUMBER => Number::new(token.clone()),
            Tag::SYMBOLS => Number::new(Token::new_number(self.symbols.get(&token.get_symbols()))),
            Tag::MULTIPLY => Number::new(Token::new_number(self.line_num as i32)),
            _ => {
                panic!("syntax error");
            }
        };
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_op_simple() {
        let table = SymbolTable::new();

        let tokens = vec![];
        let mut parser = AddrParser::new(&table, 0, &tokens);
        let e = parser.expr();
        let result = e.reduce();
        assert_eq!(None, result);

        let tokens = vec![Token::new_number(5)];
        let mut parser = AddrParser::new(&table, 0, &tokens);
        let e = parser.expr();
        let result = e.reduce();
        assert_eq!(Some(5), result);

        let tokens = vec![Token::new(Tag::PLUS, "+".to_string()), Token::new_number(5)];
        let mut parser = AddrParser::new(&table, 0, &tokens);
        let e = parser.expr();
        let result = e.reduce();
        assert_eq!(Some(5), result);

        let tokens = vec![
            Token::new(Tag::MINUS, "-".to_string()),
            Token::new_number(5),
        ];
        let mut parser = AddrParser::new(&table, 0, &tokens);
        let e = parser.expr();
        let result = e.reduce();
        assert_eq!(Some(-5), result);

        let tokens = vec![Token::new(Tag::MULTIPLY, "*".to_string())];
        let mut parser = AddrParser::new(&table, 1, &tokens);
        let e = parser.expr();
        let result = e.reduce();
        assert_eq!(Some(1), result);
    }

    #[test]
    fn binary_op() {
        let table = SymbolTable::new();

        let tokens = vec![
            Token::new_number(5),
            Token::new(Tag::PLUS, "+".to_string()),
            Token::new_number(6),
        ];
        let mut parser = AddrParser::new(&table, 0, &tokens);
        let e = parser.expr();
        let result = e.reduce();
        assert_eq!(Some(11), result);

        let tokens = vec![
            Token::new_number(5),
            Token::new(Tag::MINUS, "-".to_string()),
            Token::new_number(6),
        ];
        let mut parser = AddrParser::new(&table, 0, &tokens);
        let e = parser.expr();
        let result = e.reduce();
        assert_eq!(Some(-1), result);

        let tokens = vec![
            Token::new_number(1),
            Token::new(Tag::F_OP, ":".to_string()),
            Token::new_number(5),
        ];
        let mut parser = AddrParser::new(&table, 0, &tokens);
        let e = parser.expr();
        let result = e.reduce();
        assert_eq!(Some(13), result);

        let tokens = vec![
            Token::new(Tag::MULTIPLY, "*".to_string()),
            Token::new(Tag::MULTIPLY, "*".to_string()),
            Token::new(Tag::MULTIPLY, "*".to_string()),
        ];
        let mut parser = AddrParser::new(&table, 2, &tokens);
        let e = parser.expr();
        let result = e.reduce();
        assert_eq!(Some(4), result);
    }

    #[test]
    fn binary_op_symbols() {
        let mut table = SymbolTable::new();
        table.put("x1", 2);
        table.put("x2", 4);

        let tokens = vec![
            Token::new_symbols("x1".to_string()),
            Token::new(Tag::PLUS, "+".to_string()),
            Token::new_symbols("x2".to_string()),
        ];
        let mut parser = AddrParser::new(&table, 0, &tokens);
        let e = parser.expr();
        let result = e.reduce();
        assert_eq!(Some(6), result);
    }

    #[test]
    fn exprs() {
        let table = SymbolTable::new();

        let tokens = vec![
            Token::new_number(5),
            Token::new(Tag::PLUS, "+".to_string()),
            Token::new_number(6),
        ];
        let mut parser = AddrParser::new(&table, 0, &tokens);
        let result = parser.exprs(None);
        assert_eq!(Some(11), result);

        let tokens = vec![
            Token::new_number(1),
            Token::new(Tag::PLUS, "+".to_string()),
            Token::new_number(2),
            Token::new(Tag::MINUS, "-".to_string()),
            Token::new_number(3),
            Token::new(Tag::PLUS, "+".to_string()),
            Token::new_number(9),
        ];
        let mut parser = AddrParser::new(&table, 0, &tokens);
        let result = parser.exprs(None);
        assert_eq!(Some(9), result);

        let tokens = vec![
            Token::new(Tag::MINUS, "-".to_string()),
            Token::new_number(1),
            Token::new(Tag::PLUS, "+".to_string()),
            Token::new_number(5),

            Token::new(Tag::MULTIPLY, "*".to_string()),
            Token::new_number(20),

            Token::new(Tag::DEVIDE, "/".to_string()),
            Token::new_number(6),
        ];
        let mut parser = AddrParser::new(&table, 0, &tokens);
        let result = parser.exprs(None);
        assert_eq!(Some(13), result);
    }
}
