use crate::new_if_presudo_op;
use crate::pseudo_op::*;
use crate::tags::Tag;
use crate::MixInstruction;

use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

pub struct Token {
    tag: Tag,

    num_value: Option<i32>,
    symbols_value: Option<String>,
}
impl Token {
    pub fn new(tag: Tag, value: String) -> Token {
        Token {
            tag: tag,
            num_value: None,
            symbols_value: Some(value),
        }
    }
    pub fn new_number(value: i32) -> Token {
        Token {
            tag: Tag::NUMBER,
            num_value: Some(value),
            symbols_value: None,
        }
    }
    pub fn new_symbols(value: String) -> Token {
        let tag = if value.is_empty() {
            Tag::BLANK
        } else {
            Tag::SYMBOLS
        };

        Token {
            tag: tag,
            num_value: None,
            symbols_value: Some(value),
        }
    }
    pub fn get_tag(&self) -> Tag {
        self.tag
    }
    pub fn get_number(&self) -> i32 {
        return match self.num_value {
            Some(x) => x,
            _ => panic!("token doesn't have num value"),
        };
    }
    pub fn get_symbols(&self) -> String {
        return match &self.symbols_value {
            Some(x) => String::from(&x[..]),
            _ => panic!("token doesn't have num value"),
        };
    }
}
impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self.num_value {
            Some(x) => x.to_string(),
            None => match &self.symbols_value {
                Some(x) => x,
                None => "<NO VALUE>",
            }
            .to_string(),
        };

        write!(f, "|{}|", value)
    }
}
impl Clone for Token {
    fn clone(&self) -> Token {
        Token {
            tag: self.tag,
            num_value: self.num_value,
            symbols_value: self.symbols_value.clone(),
        }
    }
}

pub struct OpToken<'a> {
    tag: Tag,
    mix_op: Option<MixInstruction<'a>>,
    mixal_op: Option<MixalOp>,
}
impl<'a> OpToken<'a> {
    pub fn new_mix_op(op: MixInstruction<'a>) -> OpToken<'a> {
        OpToken {
            tag: Tag::MIX_OP,
            mix_op: Some(op),
            mixal_op: None,
        }
    }
    pub fn new_mixal_op(op: MixalOp) -> OpToken<'a> {
        OpToken {
            tag: Tag::MIXAL_OP,
            mix_op: None,
            mixal_op: Some(op),
        }
    }
    pub fn get_tag(&self) -> Tag {
        self.tag
    }
    pub fn get_mix_op(&self) -> &MixInstruction<'a> {
        return match &self.mix_op {
            Some(x) => x,
            _ => panic!("op token doesn't have mix operation"),
        };
    }
    pub fn get_mixal_op(&self) -> &MixalOp {
        return match &self.mixal_op {
            Some(x) => x,
            _ => panic!("op token doesn't have mixal operation"),
        };
    }
}
impl Clone for OpToken<'_> {
    fn clone(&self) -> Self {
        OpToken {
            tag: self.tag,
            mix_op: self.mix_op.clone(),
            mixal_op: self.mixal_op.clone(),
        }
    }
}
impl fmt::Debug for OpToken<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match &self.mix_op {
            Some(x) => x.name.to_string(),
            None => match &self.mixal_op {
                Some(x) => x.get_name(),
                None => "<NO VALUE>".to_string(),
            },
        };

        write!(f, "|{}|", value)
    }
}
