use crate::lexer::split_whitespace_once;
use crate::parser::Printable;
use std::collections::HashMap;

pub fn new_if_presudo_op(op: &str, line: &str) -> Option<MixalOp> {
    return match op {
        "EQU" => Some(MixalOp::new(op.to_string(), line.to_string())),
        "ORIG" => Some(MixalOp::new(op.to_string(), line.to_string())),
        "CON" => Some(MixalOp::new(op.to_string(), line.to_string())),
        "ALF" => Some(MixalOp::new(op.to_string(), line.to_string())),
        "END" => Some(MixalOp::new(op.to_string(), line.to_string())),
        _ => None,
    };
}
pub struct MixalOp {
    name: String,
    value: String,
}
impl MixalOp {
    pub fn new(name: String, value: String) -> MixalOp {
        MixalOp { name, value }
    }
    pub fn parse_address(&self) -> String {
        return match &self.name[..] {
            "ALF" => self.value[0..5].to_string(),
            _ => {
                let (op, _) = split_whitespace_once(&self.value[..]);
                op.to_string()
            }
        };
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
impl Clone for MixalOp {
    fn clone(&self) -> MixalOp {
        MixalOp {
            name: self.name.clone(),
            value: self.value.clone(),
        }
    }
}
