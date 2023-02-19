use crate::lexer::token::*;
use crate::tags::*;

pub const ABS: u32 = 0b00_111111_111111_111111_111111_111111;

pub const SIGN: u32 = 0b10_000000_000000_000000_000000_000000;
pub const BYTE_1: u32 = 0b00_111111_000000_000000_000000_000000;
pub const BYTE_2: u32 = 0b00_000000_111111_000000_000000_000000;
pub const BYTE_3: u32 = 0b00_000000_000000_111111_000000_000000;
pub const BYTE_4: u32 = 0b00_000000_000000_000000_111111_000000;
pub const BYTE_5: u32 = 0b00_000000_000000_000000_000000_111111;

pub const BYTES: [u32; 6] = [SIGN, BYTE_1, BYTE_2, BYTE_3, BYTE_4, BYTE_5];

pub trait Expr {
    fn reduce(&self) -> Option<i32>;
    fn to_string(&self) -> String;
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
    fn to_string(&self) -> String {
        "empty_expr".to_string()
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
    fn to_string(&self) -> String {
        format!("holder {:#?}", self.val)
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
    fn to_string(&self) -> String {
        format!("number {:#?}", self.token.get_number())
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
    fn to_string(&self) -> String {
        format!("binary_op {} {:#?} {}", self.left.to_string(), self.tag, self.right.to_string())
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
    fn to_string(&self) -> String {
        format!("unary_op {:#?} {}",  self.tag, self.right.to_string())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct WordAccess {
    pub left: u8,
    pub right: u8,
    pub spec: u8,
}

impl WordAccess {
    pub fn new(left: u8, right: u8) -> WordAccess {
        if left > 5 || right > 5 {
            panic!("wrong left, right values {}:{}", left, right)
        }

        WordAccess {
            left,
            right,
            spec: (8 * left + right),
        }
    }

    pub fn new_by_spec(spec: u8) -> WordAccess {
        let left: u8 = spec / 8;
        let right: u8 = spec - left * 8;

        if left > right {
            panic!("left can't be greather then right {left}:{right}={spec}");
        }
        if left > 5 || right > 5 {
            panic!("wrong parsed values {left}:{right}={spec}");
        }

        WordAccess { left, right, spec }
    }
}

impl PartialEq for WordAccess {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.right == other.right && self.spec == other.spec
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Word {
    value: u32,
}

impl Word {
    pub fn new(value: u32) -> Word {
        Word { value }
    }

    pub fn new_from_signed(value: i32) -> Word {
        let sign = Word::get_sign_mask_from_value(value);
        let result = (value.abs() as u32) | sign;
        Word { value: result }
    }

    pub fn get(&self) -> u32 {
        self.value
    }
    pub fn put_by_access(&self, value: i32, access: WordAccess) {
      //TODO :need impl
    }

    pub fn get_by_access(&self, access: WordAccess) -> u32 {
        if access.left == 0 && access.right == 0 {
            return self.value & SIGN;
        }

        let mut result = 0;
        for b in access.left..access.right + 1 {
            if b == 0 {
                continue;
            }
            result |= self.value & BYTES[b as usize];
            // println!("{:#034b}", result);
        }

        result >>= 6 * (5 - access.right);
        if access.left == 0 {
            result |= self.value & SIGN;
        }

        result
    }

    pub fn get_negative_by_access(&self, access: WordAccess) -> u32 {
        let positive_value = self.get_by_access(access);
        let sign = positive_value & SIGN;
        let result = if sign == 0 {
            positive_value | SIGN
        } else {
            positive_value & !SIGN
        };
        result
    }

    pub fn get_signed_value(&self) -> i32 {
        let positive_val = (self.value & !SIGN) as i32;

        return if (self.value & SIGN) == 0 {
            positive_val
        } else {
            -positive_val
        };
    }

    pub fn get_sign_mask_from_value(value: i32) -> u32 {
        if value < 0 {
            SIGN
        } else {
            0
        }
    }
}
