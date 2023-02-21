use crate::lexer::token::*;
use crate::parser::*;

pub struct AddrParser<'a> {
    symbols: &'a SymbolTable,
    line_num: u32,
    line_addr: u32,

    tokens: &'a Vec<Token>,
    current: usize,
}
impl<'a> AddrParser<'a> {
    pub fn new(symbols: &'a SymbolTable, line_num: u32, line_addr: u32, tokens: &'a Vec<Token>) -> AddrParser<'a> {
        AddrParser {
            symbols: symbols,
            line_num: line_num,
            line_addr: line_addr,
            tokens: tokens,
            current: 0,
        }
    }

    fn current(&mut self) -> Option<Token> {
        let result = match self.tokens.get(self.current) {
            None => None,
            Some(t) => Some(t.clone()),
        };
        result
    }
    fn step(&mut self) {
        self.current += 1;
    }

    pub fn literal_constant(&mut self) -> Vec<(Option<i32>, Option<i32>)> {
        match self.current() {
            None => return Vec::new(),
            Some(t) => {
                if t.get_tag() == Tag::EQUAL {
                    self.step();
                }
            }
        }
        return self.w_value(Vec::new());
    }

    pub fn w_value(
        &mut self,
        mut acc: Vec<(Option<i32>, Option<i32>)>,
    ) -> Vec<(Option<i32>, Option<i32>)> {
        loop {
            match self.current() {
                None => break,
                Some(t) => match t.get_tag() {
                    Tag::COMMA | Tag::OPEN_BR | Tag::EQUAL => self.step(),
                    tag => break,
                },
            };
        }

        let e = self.exprs(None);
        let mut f_part = None;
        if e != None {
            f_part = self.f_part();
        }

        if (e == None && f_part == None) {
            return acc;
        }
        acc.push((e, f_part));

        return match self.current() {
            None => acc,
            Some(t) => match t.get_tag() {
                Tag::EQUAL => return acc,
                Tag::COMMA | Tag::CLOSE_BR => {
                    self.step();
                    self.w_value(acc)
                }
                tag => panic!("w_value syntax error"),
            },
        };
    }

    pub fn aif(&mut self) -> (Option<i32>, Option<i32>, Option<i32>) {
        let a_part = self.exprs(None);
        // println!("a_part {:#?}", a_part);

        let i_part = match self.current() {
            None => return (a_part, None, None),
            Some(t) => match t.get_tag() {
                Tag::COMMA => {
                    self.step();
                    self.exprs(None)
                }
                _ => None,
            },
        };

        let f_part = self.f_part();
        // println!("i_part {:#?}", i_part);

        // println!("f_part {:#?}", f_part);
        (a_part, i_part, f_part)
    }

    fn f_part(&mut self) -> Option<i32> {
        return match self.current() {
            None => None,
            Some(t) => match t.get_tag() {
                Tag::OPEN_BR => {
                    self.step();
                    self.exprs(None)
                }
                _ => None,
            },
        };
    }

    fn exprs(&mut self, acc: Option<i32>) -> Option<i32> {
        let left = match acc {
            None => self.expr().reduce(),
            Some(x) => acc,
        };
        // println!("exprs left {:#?}", left);
        // println!("exprs current {:#?}", self.current());

        let op = match self.current() {
            None => return left,
            Some(t) => match t.get_tag() {
                Tag::COMMA => return left,
                Tag::EQUAL => return left, //TODO: this should be parsed as symbols
                Tag::OPEN_BR => return left,
                Tag::CLOSE_BR => return left,
                Tag::NUMBER => return left,
                Tag::SYMBOLS => return left,
                _ => t.get_tag(),
            },
        };
        self.step();
        // println!("exprs op {:#?}", op);

        let right: Box<dyn Expr> = match self.current() {
            None => panic!("syntax error"),
            Some(t) => Box::new(self.unary(t.clone())),
        };
        self.step();

        let result = BinaryOp::new(op, Box::new(Holder::new(left)), right).reduce();
        self.exprs(result)
    }

    fn expr(&mut self) -> Box<dyn Expr> {
        let left: Box<dyn Expr> = match self.current() {
            None => return Box::new(EmptyExpr::new()),
            Some(t) => Box::new(self.unary(t.clone())),
        };
        self.step();
        // println!("expr left {:#?}", left.to_string());
        // println!("expr current {:#?}", self.current());

        let op = match self.current() {
            None => return left,
            Some(t) => match t.get_tag() {
                Tag::COMMA => return left,
                Tag::EQUAL => return left, //TODO: this should be parsed as symbols
                Tag::OPEN_BR => return left,
                Tag::CLOSE_BR => return left,
                _ => t.get_tag(),
            },
        };
        self.step();
        // println!("expr op {:#?}", op);

        let right: Box<dyn Expr> = match self.current() {
            None => panic!("syntax error"),
            Some(t) => Box::new(self.unary(t.clone())),
        };
        self.step();

        Box::new(BinaryOp::new(op, left, right))
    }

    fn unary(&mut self, token: Token) -> UnaryOp {
        return match token.get_tag() {
            Tag::MINUS => {
                self.step();
                let next_t = self.current().expect("syntax error");
                UnaryOp::new(Tag::MINUS, Box::new(self.atom_expr(next_t)))
            }
            Tag::PLUS => {
                self.step();
                let next_t = self.current().expect("syntax error");
                UnaryOp::new(Tag::PLUS, Box::new(self.atom_expr(next_t)))
            }
            _ => UnaryOp::new(Tag::PLUS, Box::new(self.atom_expr(token))),
        };
    }

    fn atom_expr(&self, token: Token) -> Number {
        return match token.get_tag() {
            Tag::NUMBER => Number::new(token.clone()),
            Tag::SYMBOLS => Number::new(Token::new_number(self.symbols.get(token.get_symbols(), self.line_addr))),
            Tag::MULTIPLY => Number::new(Token::new_number(self.line_num as i32)),
            tag => {
                panic!("atom_expr syntax error {:#?}", tag);
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_op_simple() {
        let table = SymbolTable::new();

        let tokens = vec![];
        let mut parser = AddrParser::new(&table, 0, 0, &tokens);
        let e = parser.expr();
        let result = e.reduce();
        assert_eq!(None, result);

        let tokens = vec![Token::new_number(5)];
        let mut parser = AddrParser::new(&table, 0, 0, &tokens);
        let e = parser.expr();
        let result = e.reduce();
        assert_eq!(Some(5), result);

        let tokens = vec![Token::new(Tag::PLUS, "+".to_string()), Token::new_number(5)];
        let mut parser = AddrParser::new(&table, 0, 0, &tokens);
        let e = parser.expr();
        let result = e.reduce();
        assert_eq!(Some(5), result);

        let tokens = vec![
            Token::new(Tag::MINUS, "-".to_string()),
            Token::new_number(5),
        ];
        let mut parser = AddrParser::new(&table, 0, 0, &tokens);
        let e = parser.expr();
        let result = e.reduce();
        assert_eq!(Some(-5), result);

        let tokens = vec![Token::new(Tag::MULTIPLY, "*".to_string())];
        let mut parser = AddrParser::new(&table, 1, 0, &tokens);
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
        let mut parser = AddrParser::new(&table, 0, 0, &tokens);
        let e = parser.expr();
        let result = e.reduce();
        assert_eq!(Some(11), result);

        let tokens = vec![
            Token::new_number(5),
            Token::new(Tag::MINUS, "-".to_string()),
            Token::new_number(6),
        ];
        let mut parser = AddrParser::new(&table, 0, 0, &tokens);
        let e = parser.expr();
        let result = e.reduce();
        assert_eq!(Some(-1), result);

        let tokens = vec![
            Token::new_number(1),
            Token::new(Tag::F_OP, ":".to_string()),
            Token::new_number(5),
        ];
        let mut parser = AddrParser::new(&table, 0, 0, &tokens);
        let e = parser.expr();
        let result = e.reduce();
        assert_eq!(Some(13), result);

        let tokens = vec![
            Token::new(Tag::MULTIPLY, "*".to_string()),
            Token::new(Tag::MULTIPLY, "*".to_string()),
            Token::new(Tag::MULTIPLY, "*".to_string()),
        ];
        let mut parser = AddrParser::new(&table, 2, 0, &tokens);
        let e = parser.expr();
        let result = e.reduce();
        assert_eq!(Some(4), result);
    }

    #[test]
    fn binary_op_symbols() {
        let mut table = SymbolTable::new();
        table.put_equ("x1".to_string(), Word::new(2));
        table.put_equ("x2".to_string(), Word::new(4));

        let tokens = vec![
            Token::new_symbols("x1".to_string()),
            Token::new(Tag::PLUS, "+".to_string()),
            Token::new_symbols("x2".to_string()),
        ];
        let mut parser = AddrParser::new(&table, 0, 0, &tokens);
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
        let mut parser = AddrParser::new(&table, 0, 0, &tokens);
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
        let mut parser = AddrParser::new(&table, 0, 0, &tokens);
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
        let mut parser = AddrParser::new(&table, 0, 0, &tokens);
        let result = parser.exprs(None);
        assert_eq!(Some(13), result);

        let tokens = vec![
            Token::new(Tag::MULTIPLY, "*".to_string()),
            Token::new(Tag::MULTIPLY, "*".to_string()),
            Token::new(Tag::MULTIPLY, "*".to_string()),
        ];
        let mut parser = AddrParser::new(&table, 2, 0, &tokens);
        let result = parser.exprs(None);
        assert_eq!(Some(4), result);

        let tokens = vec![
            Token::new(Tag::MULTIPLY, "*".to_string()),
            Token::new(Tag::MINUS, "-".to_string()),
            Token::new_number(3),
        ];
        let mut parser = AddrParser::new(&table, 2, 0, &tokens);
        let result = parser.exprs(None);
        assert_eq!(Some(-1), result);

        let tokens = vec![
            Token::new(Tag::MULTIPLY, "*".to_string()),
            Token::new(Tag::MINUS, "-".to_string()),
            Token::new(Tag::MINUS, "-".to_string()),
            Token::new_number(3),
        ];
        let mut parser = AddrParser::new(&table, 2, 0, &tokens);
        let result = parser.exprs(None);
        assert_eq!(Some(5), result);
    }

    #[test]
    fn aif() {
        let table = SymbolTable::new();

        let tokens = vec![
            Token::new_number(5),
            Token::new(Tag::COMMA, ",".to_string()),
            Token::new_number(2),
            Token::new(Tag::OPEN_BR, "(".to_string()),
            Token::new_number(0),
            Token::new(Tag::CLOSE_BR, ")".to_string()),
        ];
        let mut parser = AddrParser::new(&table, 0, 0, &tokens);
        let (a, i, f) = parser.aif();
        assert_eq!(Some(5), a);
        assert_eq!(Some(2), i);
        assert_eq!(Some(0), f);

        let tokens = vec![
            Token::new_number(5),
            Token::new(Tag::COMMA, ",".to_string()),
            Token::new_number(2),
            Token::new(Tag::OPEN_BR, "(".to_string()),
            Token::new_number(1),
            Token::new(Tag::F_OP, ":".to_string()),
            Token::new_number(5),
            Token::new(Tag::CLOSE_BR, ")".to_string()),
        ];
        let mut parser = AddrParser::new(&table, 0, 0, &tokens);
        let (a, i, f) = parser.aif();
        assert_eq!(Some(5), a);
        assert_eq!(Some(2), i);
        assert_eq!(Some(13), f);

        let tokens = vec![Token::new_number(5)];
        let mut parser = AddrParser::new(&table, 0, 0, &tokens);
        let (a, i, f) = parser.aif();
        assert_eq!(Some(5), a);
        assert_eq!(None, i);
        assert_eq!(None, f);

        let tokens = vec![
            Token::new_number(5),
            Token::new(Tag::COMMA, ",".to_string()),
            Token::new_number(2),
        ];
        let mut parser = AddrParser::new(&table, 0, 0, &tokens);
        let (a, i, f) = parser.aif();
        assert_eq!(Some(5), a);
        assert_eq!(Some(2), i);
        assert_eq!(None, f);

        let tokens = vec![
            Token::new_number(5),
            Token::new(Tag::PLUS, "+".to_string()),
            Token::new_number(5),
            Token::new(Tag::COMMA, ",".to_string()),
            Token::new_number(2),
            Token::new(Tag::OPEN_BR, "(".to_string()),
            Token::new_number(0),
            Token::new(Tag::CLOSE_BR, ")".to_string()),
        ];
        let mut parser = AddrParser::new(&table, 0, 0, &tokens);
        let (a, i, f) = parser.aif();
        assert_eq!(Some(10), a);
        assert_eq!(Some(2), i);
        assert_eq!(Some(0), f);
    }

    #[test]
    fn w_value() {
        let table = SymbolTable::new();

        let tokens = vec![Token::new_number(1)];
        let mut parser = AddrParser::new(&table, 0, 0, &tokens);
        let result = parser.w_value(Vec::new());

        assert_eq!(1, result.len());
        let (e1, f1) = result.get(0).expect("error");

        assert_eq!(Some(1), *e1);
        assert_eq!(None, *f1);

        let tokens = vec![
            Token::new_number(1),
            Token::new(Tag::COMMA, ",".to_string()),
            Token::new(Tag::MINUS, "-".to_string()),
            Token::new_number(1000),
            Token::new(Tag::OPEN_BR, "(".to_string()),
            Token::new_number(0),
            Token::new(Tag::F_OP, ":".to_string()),
            Token::new_number(2),
            Token::new(Tag::CLOSE_BR, ")".to_string()),
        ];
        let mut parser = AddrParser::new(&table, 0, 0, &tokens);
        let result = parser.w_value(Vec::new());

        assert_eq!(2, result.len());

        let (e1, f1) = result.get(0).expect("error");
        assert_eq!(Some(1), *e1);
        assert_eq!(None, *f1);

        let (e2, f2) = result.get(1).expect("error");
        assert_eq!(Some(-1000), *e2);
        assert_eq!(Some(2), *f2);

        let tokens = vec![
            Token::new(Tag::MINUS, "-".to_string()),
            Token::new_number(1000),
            Token::new(Tag::OPEN_BR, "(".to_string()),
            Token::new_number(0),
            Token::new(Tag::F_OP, ":".to_string()),
            Token::new_number(2),
            Token::new(Tag::CLOSE_BR, ")".to_string()),
            Token::new(Tag::COMMA, ",".to_string()),
            Token::new_number(1),
        ];
        let mut parser = AddrParser::new(&table, 0, 0, &tokens);
        let result = parser.w_value(Vec::new());

        assert_eq!(2, result.len());

        let (e1, f1) = result.get(0).expect("error");
        assert_eq!(Some(-1000), *e1);
        assert_eq!(Some(2), *f1);

        let (e2, f2) = result.get(1).expect("error");
        assert_eq!(Some(1), *e2);
        assert_eq!(None, *f2);
    }

    #[test]
    fn literal_constant() {
        let mut table = SymbolTable::new();

        let tokens = vec![
            Token::new(Tag::EQUAL, "=".to_string()),
            Token::new_number(1),
            Token::new(Tag::EQUAL, "=".to_string()),
        ];
        let mut parser = AddrParser::new(&table, 0, 0, &tokens);
        let result = parser.literal_constant();

        assert_eq!(1, result.len());
        let (e1, f1) = result.get(0).expect("error");

        assert_eq!(Some(1), *e1);
        assert_eq!(None, *f1);

        table.put_equ("L".to_string(), Word::new(2));
        let tokens = vec![
            Token::new(Tag::EQUAL, "=".to_string()),
            Token::new_number(1),
            Token::new(Tag::MINUS, "-".to_string()),
            Token::new_symbols("L".to_string()),
            Token::new(Tag::EQUAL, "=".to_string()),
        ];
        let mut parser = AddrParser::new(&table, 0, 0, &tokens);
        let result = parser.literal_constant();

        assert_eq!(1, result.len());
        let (e1, f1) = result.get(0).expect("error");

        assert_eq!(Some(-1), *e1);
        assert_eq!(None, *f1);
    }
}
