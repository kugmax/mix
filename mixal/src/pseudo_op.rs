use crate::parser::split_whitespace_once;
use crate::parser::Printable;
use std::collections::HashMap;

pub fn new_if_presudo_of(op: &str, line: &str) -> Option<Box<dyn PseudoOp>> {
    return match op {
        "EQU" => Some(Box::new(EQU::new(line.to_string()))),
        "ORIG" => Some(Box::new(ORIG::new(line.to_string()))),
        "CON" => Some(Box::new(CON::new(line.to_string()))),
        "ALF" => Some(Box::new(ALF::new(line.to_string()))),
        "END" => Some(Box::new(END::new(line.to_string()))),
        _ => None,
    };
}

pub trait PseudoOp {
    fn parse_address(&self) -> String;
}

pub struct EQU {
    value: String,
}
impl EQU {
    pub fn new(value: String) -> EQU {
        EQU { value }
    }
}
impl PseudoOp for EQU {
    fn parse_address(&self) -> String {
        let (op, _) = split_whitespace_once(&self.value[..]);
        op.to_string()
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
impl PseudoOp for ORIG {
    fn parse_address(&self) -> String {
        let (op, _) = split_whitespace_once(&self.value[..]);
        op.to_string()
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
impl PseudoOp for CON {
    fn parse_address(&self) -> String {
        let (op, _) = split_whitespace_once(&self.value[..]);
        op.to_string()
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
impl PseudoOp for ALF {
    fn parse_address(&self) -> String {
        self.value[0..5].to_string()
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
impl PseudoOp for END {
    fn parse_address(&self) -> String {
        let (op, _) = split_whitespace_once(&self.value[..]);
        op.to_string()
    }
}
