use crate::lexer::token::*;
use crate::tags::*;

pub trait Expr {
    fn reduce(&self) -> Option<i32>;
}

pub struct EmptyExpr {}
impl EmptyExpr {
    pub fn new() -> EmptyExpr {
        EmptyExpr {}
    }
}
impl Expr for EmptyExpr {
    fn reduce(&self) -> Option<i32> {
        None
    }
}
pub struct Holder {
    val: Option<i32>,
}
impl Holder {
    pub fn new(val: Option<i32>) -> Holder {
        Holder { val }
    }
}
impl Expr for Holder {
    fn reduce(&self) -> Option<i32> {
        self.val
    }
}

pub struct Number {
    token: Token,
}
impl Number {
    pub fn new(token: Token) -> Number {
        Number { token }
    }
}
impl Expr for Number {
    fn reduce(&self) -> Option<i32> {
        Some(self.token.get_number())
    }
}

pub struct BinaryOp {
    tag: Tag,

    left: Box<dyn Expr>,
    right: Box<dyn Expr>,
}
impl BinaryOp {
    pub fn new(tag: Tag, left: Box<dyn Expr>, right: Box<dyn Expr>) -> BinaryOp {
        BinaryOp {
            tag: tag,
            left: left,
            right: right,
        }
    }
}
impl Expr for BinaryOp {
    fn reduce(&self) -> Option<i32> {
        let op = match self.tag {
            Tag::PLUS => |l, r| l + r,
            Tag::MINUS => |l, r| l - r,
            Tag::MULTIPLY => |l, r| l * r,
            Tag::DEVIDE => |l, r| l / r,
            Tag::MOD => |l, r| ((5 ^ 5) * l) / r, //TODO: this is not a mod operation
            Tag::F_OP => |l, r| l * 8 + r,

            _ => panic!("unsupported binary operation {:#?}", self.tag),
        };

        let l: i32 = self.left.reduce().expect("syntax error");
        let r: i32 = self.right.reduce().expect("syntax error");
        let result: i32 = op(l, r);
        Some(result)
    }
}

pub struct UnaryOp {
    tag: Tag,

    right: Box<dyn Expr>,
}
impl UnaryOp {
    pub fn new(tag: Tag, right: Box<dyn Expr>) -> UnaryOp {
        UnaryOp {
            tag: tag,
            right: right,
        }
    }
}
impl Expr for UnaryOp {
    fn reduce(&self) -> Option<i32> {
        let op = match self.tag {
            Tag::PLUS => |r| r,
            Tag::MINUS => |r| 0 - r,

            _ => panic!("unsupported unary operation {:#?}", self.tag),
        };

        Some(op(self.right.reduce().expect("syntax error")))
    }
}
