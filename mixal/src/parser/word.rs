use crate::tags::*;
use crate::lexer::token::*;

pub trait Expr {
    fn reduce(&self) -> i32;
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
    fn reduce(&self) -> i32 {
        self.token.get_number()
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
    fn reduce(&self) -> i32 {
        let op = match self.tag {
            Tag::PLUS => |l, r| l + r,
            Tag::MINUS => |l, r| l - r,
            Tag::MULTIPLY => |l, r| l * r,
            Tag::DEVIDE => |l, r| l / r,
            Tag::MOD => |l, r| ((5 ^ 5) * l) / r, //TODO: this is not a mod operation
            Tag::F_OP => |l, r| l * 8 + r,

            _ => panic!("unsupported binary operation {:#?}", self.tag),
        };

        op(self.left.reduce(), self.right.reduce())
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
    fn reduce(&self) -> i32 {
        let op = match self.tag {
            Tag::PLUS => |r| r,
            Tag::MINUS => |r| 0 - r,

            _ => panic!("unsupported unary operation {:#?}", self.tag),
        };

        op(self.right.reduce())
    }
}
