use crate::parser::Printable;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tag {
    BLANK,
    MINUS,
    PLUS,
    MULTIPLY,
    DEVIDE,
    MOD,
    F_OP,
    COMMA,
    EQUAL,
    OPEN_BR,
    CLOSE_BR,

    NUMBER,
    SYMBOLS,

    MIX_OP,
    MIXAL_OP,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MixInstruction<'a> {
    pub name: &'a str,
    aa: i32,
    i: u8,
    f: u8,
    c: u8,
}
impl<'a> MixInstruction<'a> {
    pub fn new(name: &str, aa: i32, i: u8, f: u8, c: u8) -> MixInstruction {
        MixInstruction {
            name: name,
            aa: aa,
            i: i,
            f: f,
            c: c,
        }
    }

    pub fn set_aa(&mut self, aa: i32) {
        self.aa = aa;
    }
    pub fn set_i(&mut self, i: u8) {
        self.i = i;
    }
    pub fn set_f(&mut self, f: u8) {
        self.f = f;
    }
}

impl<'a> Printable for MixInstruction<'a> {
    fn print(&self) -> String {
        self.aa.to_string()
            + ","
            + &self.i.to_string()
            + ","
            + &self.f.to_string()
            + ","
            + &self.c.to_string()
    }
}

pub struct MixInstructions<'a> {
    instructions: HashMap<&'a str, MixInstruction<'a>>,
}

impl<'a> MixInstructions<'a> {
    pub fn new() -> MixInstructions<'a> {
        let instructions = HashMap::from([
            ("NOP", MixInstruction::new("NOP", 0, 0, 0, 0)),
            ("ADD", MixInstruction::new("ADD", 0, 0, 5, 1)),
            ("SUB", MixInstruction::new("SUB", 0, 0, 5, 2)),
            ("MUL", MixInstruction::new("MUL", 0, 0, 5, 3)),
            ("DIV", MixInstruction::new("DIV", 0, 0, 5, 4)),
            ("NUM", MixInstruction::new("NUM", 0, 0, 0, 5)),
            ("CHAR", MixInstruction::new("CHAR", 0, 0, 1, 5)),
            ("HLT", MixInstruction::new("HLT", 0, 0, 2, 5)),
            ("SLA", MixInstruction::new("SLA", 0, 0, 0, 6)),
            ("SRA", MixInstruction::new("SRA", 0, 0, 1, 6)),
            ("SLAX", MixInstruction::new("SLAX", 0, 0, 2, 6)),
            ("SRAX", MixInstruction::new("SRAX", 0, 0, 3, 6)),
            ("SLC", MixInstruction::new("SLC", 0, 0, 4, 6)),
            ("SRC", MixInstruction::new("SRC", 0, 0, 5, 6)),
            ("MOVE", MixInstruction::new("MOVE", 0, 0, 0, 7)),
            ("LDA", MixInstruction::new("LDA", 0, 0, 5, 8)),
            ("LD1", MixInstruction::new("LD1", 0, 0, 5, 9)),
            ("LD2", MixInstruction::new("LD2", 0, 0, 5, 10)),
            ("LD3", MixInstruction::new("LD3", 0, 0, 5, 11)),
            ("LD4", MixInstruction::new("LD4", 0, 0, 5, 12)),
            ("LD5", MixInstruction::new("LD5", 0, 0, 5, 13)),
            ("LD6", MixInstruction::new("LD6", 0, 0, 5, 14)),
            ("LDX", MixInstruction::new("LDX", 0, 0, 5, 15)),
            ("LDAN", MixInstruction::new("LDAN", 0, 0, 5, 16)),
            ("LD1N", MixInstruction::new("LD1N", 0, 0, 5, 17)),
            ("LD2N", MixInstruction::new("LD2N", 0, 0, 5, 18)),
            ("LD3N", MixInstruction::new("LD3N", 0, 0, 5, 19)),
            ("LD4N", MixInstruction::new("LD4N", 0, 0, 5, 20)),
            ("LD5N", MixInstruction::new("LD5N", 0, 0, 5, 21)),
            ("LD6N", MixInstruction::new("LD6N", 0, 0, 5, 22)),
            ("LDXN", MixInstruction::new("LDXN", 0, 0, 5, 23)),
            ("STA", MixInstruction::new("STA", 0, 0, 5, 24)),
            ("ST1", MixInstruction::new("ST1", 0, 0, 5, 25)),
            ("ST2", MixInstruction::new("ST2", 0, 0, 5, 26)),
            ("ST3", MixInstruction::new("ST3", 0, 0, 5, 27)),
            ("ST4", MixInstruction::new("ST4", 0, 0, 5, 28)),
            ("ST5", MixInstruction::new("ST5", 0, 0, 5, 29)),
            ("ST6", MixInstruction::new("ST6", 0, 0, 5, 30)),
            ("STX", MixInstruction::new("STX", 0, 0, 5, 31)),
            ("STJ", MixInstruction::new("STJ", 0, 0, 5, 32)),
            ("STZ", MixInstruction::new("STZ", 0, 0, 5, 33)),
            ("IOC", MixInstruction::new("IOC", 0, 0, 0, 35)),
            ("IN", MixInstruction::new("IN", 0, 0, 0, 36)),
            ("OUT", MixInstruction::new("OUT", 0, 0, 0, 37)),
            ("JMP", MixInstruction::new("JMP", 0, 0, 0, 39)),
            ("JSJ", MixInstruction::new("JSJ", 0, 0, 1, 39)),
            ("JOV", MixInstruction::new("JOV", 0, 0, 2, 39)),
            ("JNOV", MixInstruction::new("JNOV", 0, 0, 3, 39)),
            ("JL", MixInstruction::new("JL", 0, 0, 4, 39)),
            ("JE", MixInstruction::new("JE", 0, 0, 5, 39)),
            ("JG", MixInstruction::new("JG", 0, 0, 6, 39)),
            ("JGE", MixInstruction::new("JGE", 0, 0, 7, 39)),
            ("JNE", MixInstruction::new("JNE", 0, 0, 8, 39)),
            ("JLE", MixInstruction::new("JLE", 0, 0, 9, 39)),
            ("JAN", MixInstruction::new("JAN", 0, 0, 0, 40)),
            ("JAZ", MixInstruction::new("JAZ", 0, 0, 1, 40)),
            ("JAP", MixInstruction::new("JAP", 0, 0, 2, 40)),
            ("JANN", MixInstruction::new("JANN", 0, 0, 3, 40)),
            ("JANZ", MixInstruction::new("JANZ", 0, 0, 4, 40)),
            ("JANP", MixInstruction::new("JANP", 0, 0, 5, 40)),
            ("J1N", MixInstruction::new("J1N", 0, 0, 0, 41)),
            ("J2N", MixInstruction::new("J2N", 0, 0, 0, 42)),
            ("J3N", MixInstruction::new("J3N", 0, 0, 0, 43)),
            ("J4N", MixInstruction::new("J4N", 0, 0, 0, 44)),
            ("J5N", MixInstruction::new("J5N", 0, 0, 0, 45)),
            ("J6N", MixInstruction::new("J6N", 0, 0, 0, 46)),
            ("J1Z", MixInstruction::new("J1Z", 0, 0, 1, 41)),
            ("J2Z", MixInstruction::new("J2Z", 0, 0, 1, 42)),
            ("J3Z", MixInstruction::new("J3Z", 0, 0, 1, 43)),
            ("J4Z", MixInstruction::new("J4Z", 0, 0, 1, 44)),
            ("J5Z", MixInstruction::new("J5Z", 0, 0, 1, 45)),
            ("J6Z", MixInstruction::new("J6Z", 0, 0, 1, 46)),
            ("J1P", MixInstruction::new("J1P", 0, 0, 2, 41)),
            ("J2P", MixInstruction::new("J2P", 0, 0, 2, 42)),
            ("J3P", MixInstruction::new("J3P", 0, 0, 2, 43)),
            ("J4P", MixInstruction::new("J4P", 0, 0, 2, 44)),
            ("J5P", MixInstruction::new("J5P", 0, 0, 2, 45)),
            ("J6P", MixInstruction::new("J6P", 0, 0, 2, 46)),
            ("J1NN", MixInstruction::new("J1NN", 0, 0, 3, 41)),
            ("J2NN", MixInstruction::new("J2NN", 0, 0, 3, 42)),
            ("J3NN", MixInstruction::new("J3NN", 0, 0, 3, 43)),
            ("J4NN", MixInstruction::new("J4NN", 0, 0, 3, 44)),
            ("J5NN", MixInstruction::new("J5NN", 0, 0, 3, 45)),
            ("J6NN", MixInstruction::new("J6NN", 0, 0, 3, 46)),
            ("J1NZ", MixInstruction::new("J1NZ", 0, 0, 4, 41)),
            ("J2NZ", MixInstruction::new("J2NZ", 0, 0, 4, 42)),
            ("J3NZ", MixInstruction::new("J3NZ", 0, 0, 4, 43)),
            ("J4NZ", MixInstruction::new("J4NZ", 0, 0, 4, 44)),
            ("J5NZ", MixInstruction::new("J5NZ", 0, 0, 4, 45)),
            ("J6NZ", MixInstruction::new("J6NZ", 0, 0, 4, 46)),
            ("J1NP", MixInstruction::new("J1NP", 0, 0, 5, 41)),
            ("J2NP", MixInstruction::new("J2NP", 0, 0, 5, 42)),
            ("J3NP", MixInstruction::new("J3NP", 0, 0, 5, 43)),
            ("J4NP", MixInstruction::new("J4NP", 0, 0, 5, 44)),
            ("J5NP", MixInstruction::new("J5NP", 0, 0, 5, 45)),
            ("J6NP", MixInstruction::new("J6NP", 0, 0, 5, 46)),
            ("JXN", MixInstruction::new("JXN", 0, 0, 0, 47)),
            ("JXZ", MixInstruction::new("JXZ", 0, 0, 1, 47)),
            ("JXP", MixInstruction::new("JXP", 0, 0, 2, 47)),
            ("JXNN", MixInstruction::new("JXNN", 0, 0, 3, 47)),
            ("JXNZ", MixInstruction::new("JXNZ", 0, 0, 4, 47)),
            ("JXNP", MixInstruction::new("JXNP", 0, 0, 5, 47)),
            ("INCA", MixInstruction::new("INCA", 0, 0, 0, 48)),
            ("DECA", MixInstruction::new("DECA", 0, 0, 1, 48)),
            ("ENTA", MixInstruction::new("ENTA", 0, 0, 2, 48)),
            ("ENNA", MixInstruction::new("ENNA", 0, 0, 3, 48)),
            ("INC1", MixInstruction::new("INC1", 0, 0, 0, 49)),
            ("INC2", MixInstruction::new("INC2", 0, 0, 0, 50)),
            ("INC3", MixInstruction::new("INC3", 0, 0, 0, 51)),
            ("INC4", MixInstruction::new("INC4", 0, 0, 0, 52)),
            ("INC5", MixInstruction::new("INC5", 0, 0, 0, 53)),
            ("INC6", MixInstruction::new("INC6", 0, 0, 0, 54)),
            ("DEC1", MixInstruction::new("DEC1", 0, 0, 1, 49)),
            ("DEC2", MixInstruction::new("DEC2", 0, 0, 1, 50)),
            ("DEC3", MixInstruction::new("DEC3", 0, 0, 1, 51)),
            ("DEC4", MixInstruction::new("DEC4", 0, 0, 1, 52)),
            ("DEC5", MixInstruction::new("DEC5", 0, 0, 1, 53)),
            ("DEC6", MixInstruction::new("DEC6", 0, 0, 1, 54)),
            ("ENT1", MixInstruction::new("ENT1", 0, 0, 2, 49)),
            ("ENT2", MixInstruction::new("ENT2", 0, 0, 2, 50)),
            ("ENT3", MixInstruction::new("ENT3", 0, 0, 2, 51)),
            ("ENT4", MixInstruction::new("ENT4", 0, 0, 2, 52)),
            ("ENT5", MixInstruction::new("ENT5", 0, 0, 2, 53)),
            ("ENT6", MixInstruction::new("ENT6", 0, 0, 2, 54)),
            ("ENN1", MixInstruction::new("ENN1", 0, 0, 3, 49)),
            ("ENN2", MixInstruction::new("ENN2", 0, 0, 3, 50)),
            ("ENN3", MixInstruction::new("ENN3", 0, 0, 3, 51)),
            ("ENN4", MixInstruction::new("ENN4", 0, 0, 3, 52)),
            ("ENN5", MixInstruction::new("ENN5", 0, 0, 3, 53)),
            ("ENN6", MixInstruction::new("ENN6", 0, 0, 3, 54)),
            ("INCX", MixInstruction::new("INCX", 0, 0, 0, 55)),
            ("DECX", MixInstruction::new("DECX", 0, 0, 1, 55)),
            ("ENTX", MixInstruction::new("ENTX", 0, 0, 2, 55)),
            ("ENNX", MixInstruction::new("ENNX", 0, 0, 3, 55)),
            ("CMPA", MixInstruction::new("CMPA", 0, 0, 5, 56)),
            ("CMP1", MixInstruction::new("CMP1", 0, 0, 5, 57)),
            ("CMP2", MixInstruction::new("CMP2", 0, 0, 5, 58)),
            ("CMP3", MixInstruction::new("CMP3", 0, 0, 5, 59)),
            ("CMP4", MixInstruction::new("CMP4", 0, 0, 5, 60)),
            ("CMP5", MixInstruction::new("CMP5", 0, 0, 5, 61)),
            ("CMP6", MixInstruction::new("CMP6", 0, 0, 5, 62)),
            ("CMPX", MixInstruction::new("CMPX", 0, 0, 5, 63)),
        ]);

        MixInstructions { instructions }
    }

    pub fn get(&self, name: &str) -> MixInstruction {
        let inst = self
            .instructions
            .get(name)
            .expect("mix instuction not found");
        *inst
    }

    pub fn is_instruction(&self, name: &str) -> bool {
        self.instructions.contains_key(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print() {
        let t = MixInstructions::new();
        let mut op = t.get("CMPA");
        op.set_aa(-3000);
        op.set_i(3);
        assert_eq!("-3000,3,5,56", op.print());
    }
}
