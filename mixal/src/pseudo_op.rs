use crate::lexer::split_whitespace_once;
use crate::parser::Printable;
use std::collections::HashMap;

pub fn new_if_presudo_of(op: &str, line: &str) -> Option<Box<dyn MixalOp>> {
    return match op {
        "EQU" => Some(Box::new(EQU::new(line.to_string()))),
        "ORIG" => Some(Box::new(ORIG::new(line.to_string()))),
        "CON" => Some(Box::new(CON::new(line.to_string()))),
        "ALF" => Some(Box::new(ALF::new(line.to_string()))),
        "END" => Some(Box::new(END::new(line.to_string()))),
        _ => None,
    };
}

pub trait MixalOp {
    fn parse_address(&self) -> String;
    fn get_name(&self) -> String;
}

pub struct EQU {
    value: String,
}
impl EQU {
    pub fn new(value: String) -> EQU {
        EQU { value }
    }
}
impl MixalOp for EQU {
    fn parse_address(&self) -> String {
        let (op, _) = split_whitespace_once(&self.value[..]);
        op.to_string()
    }
    fn get_name(&self) -> String {
        "EQU".to_string()
    }
}

pub struct ORIG {
    value: String,
}
impl ORIG {
    pub fn new(value: String) -> ORIG {
        ORIG { value }
    }
}
impl MixalOp for ORIG {
    fn parse_address(&self) -> String {
        let (op, _) = split_whitespace_once(&self.value[..]);
        op.to_string()
    }
    fn get_name(&self) -> String {
        "ORIG".to_string()
    }
}

pub struct CON {
    value: String,
}
impl CON {
    pub fn new(value: String) -> CON {
        CON { value }
    }
}
impl MixalOp for CON {
    fn parse_address(&self) -> String {
        let (op, _) = split_whitespace_once(&self.value[..]);
        op.to_string()
    }
    fn get_name(&self) -> String {
        "CON".to_string()
    }
}

pub struct ALF {
    value: String,
}
impl ALF {
    pub fn new(value: String) -> ALF {
        ALF { value }
    }
}
impl MixalOp for ALF {
    fn parse_address(&self) -> String {
        self.value[0..5].to_string()
    }
    fn get_name(&self) -> String {
        "ALF".to_string()
    }
}

pub struct END {
    value: String,
}
impl END {
    pub fn new(value: String) -> END {
        END { value }
    }
}
impl MixalOp for END {
    fn parse_address(&self) -> String {
        let (op, _) = split_whitespace_once(&self.value[..]);
        op.to_string()
    }
    fn get_name(&self) -> String {
        "END".to_string()
    }
}
