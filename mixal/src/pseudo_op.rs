use std::collections::HashMap;

pub fn new_if_presudo_of(op: &str) -> Option<Box<dyn PseudoOp>> {
    return match op {
        "EQU" => Some(Box::new(EQU::new(" ".to_string()))),
        "ORIG" => Some(Box::new(ORIG::new(" ".to_string()))),
        "CON" => Some(Box::new(CON::new(" ".to_string()))),
        "ALF" => Some(Box::new(ALF::new(" ".to_string()))),
        "END" => Some(Box::new(END::new(" ".to_string()))),
        _ => None,
    };
}

pub trait PseudoOp {}

pub struct EQU {
    value: String,
}
impl EQU {
    pub fn new(value: String) -> EQU {
        EQU { value }
    }
}
impl PseudoOp for EQU {}

pub struct ORIG {
    value: String,
}
impl ORIG {
    pub fn new(value: String) -> ORIG {
        ORIG { value }
    }
}
impl PseudoOp for ORIG {}

pub struct CON {
    value: String,
}
impl CON {
    pub fn new(value: String) -> CON {
        CON { value }
    }
}
impl PseudoOp for CON {}

pub struct ALF {
    value: String,
}
impl ALF {
    pub fn new(value: String) -> ALF {
        ALF { value }
    }
}
impl PseudoOp for ALF {}

pub struct END {
    value: String,
}
impl END {
    pub fn new(value: String) -> END {
        END { value }
    }
}
impl PseudoOp for END {}
