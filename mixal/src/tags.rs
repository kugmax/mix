use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Instruction {
    aa: i32,
    i: u8,
    f: u8,
    c: u8,
}
impl Instruction {
    pub fn new(aa: i32, i: u8, f: u8, c: u8) -> Instruction {
        Instruction { aa, i, f, c }
    }
    pub fn set_aa(&mut self, aa: i32) {
        self.aa = aa;
    }
    pub fn set_i(&mut self, i: u8) {
        self.i = i;
    }
    // pub fn set_f(&mut self, f: u8) {
        // self.f = f;
    // }
    pub fn print(&self) -> String {
        self.aa.to_string()
            + ","
            + &self.i.to_string()
            + ","
            + &self.f.to_string()
            + ","
            + &self.c.to_string()
    }
}

pub struct Tags<'a> {
    instructions: HashMap<&'a str, Instruction>,
}

impl<'a> Tags<'a> {
    pub fn new() -> Tags<'a> {
        let instructions = HashMap::from([
            ("NOP", Instruction::new(0, 0, 0, 0)),
            ("ADD", Instruction::new(0, 0, 5, 1)),
            ("SUB", Instruction::new(0, 0, 5, 2)),
            ("MUL", Instruction::new(0, 0, 5, 3)),
            ("DIV", Instruction::new(0, 0, 5, 4)),
            ("NUM", Instruction::new(0, 0, 0, 5)),
            ("CHAR", Instruction::new(0, 0, 1, 5)),
            ("HLT", Instruction::new(0, 0, 2, 5)),
            ("SLA", Instruction::new(0, 0, 0, 6)),
            ("SRA", Instruction::new(0, 0, 1, 6)),
            ("SLAX", Instruction::new(0, 0, 2, 6)),
            ("SRAX", Instruction::new(0, 0, 3, 6)),
            ("SLC", Instruction::new(0, 0, 4, 6)),
            ("SRC", Instruction::new(0, 0, 5, 6)),
            ("MOVE", Instruction::new(0, 0, 0, 7)),
            ("LDA", Instruction::new(0, 0, 5, 8)),
            ("LD1", Instruction::new(0, 0, 5, 9)),
            ("LD2", Instruction::new(0, 0, 5, 10)),
            ("LD3", Instruction::new(0, 0, 5, 11)),
            ("LD4", Instruction::new(0, 0, 5, 12)),
            ("LD5", Instruction::new(0, 0, 5, 13)),
            ("LD6", Instruction::new(0, 0, 5, 14)),
            ("LDX", Instruction::new(0, 0, 5, 15)),
            ("LDAN", Instruction::new(0, 0, 5, 16)),
            ("LD1N", Instruction::new(0, 0, 5, 17)),
            ("LD2N", Instruction::new(0, 0, 5, 18)),
            ("LD3N", Instruction::new(0, 0, 5, 19)),
            ("LD4N", Instruction::new(0, 0, 5, 20)),
            ("LD5N", Instruction::new(0, 0, 5, 21)),
            ("LD6N", Instruction::new(0, 0, 5, 22)),
            ("LDXN", Instruction::new(0, 0, 5, 23)),
            ("STA", Instruction::new(0, 0, 5, 24)),
            ("ST1", Instruction::new(0, 0, 5, 25)),
            ("ST2", Instruction::new(0, 0, 5, 26)),
            ("ST3", Instruction::new(0, 0, 5, 27)),
            ("ST4", Instruction::new(0, 0, 5, 28)),
            ("ST5", Instruction::new(0, 0, 5, 29)),
            ("ST6", Instruction::new(0, 0, 5, 30)),
            ("STX", Instruction::new(0, 0, 5, 31)),
            ("STJ", Instruction::new(0, 0, 5, 32)),
            ("STZ", Instruction::new(0, 0, 5, 33)),
            ("IOC", Instruction::new(0, 0, 0, 35)),
            ("IN", Instruction::new(0, 0, 0, 36)),
            ("OUT", Instruction::new(0, 0, 0, 37)),
            ("JMP", Instruction::new(0, 0, 0, 39)),
            ("JSJ", Instruction::new(0, 0, 1, 39)),
            ("JOV", Instruction::new(0, 0, 2, 39)),
            ("JNOV", Instruction::new(0, 0, 3, 39)),
            ("JL", Instruction::new(0, 0, 4, 39)),
            ("JE", Instruction::new(0, 0, 5, 39)),
            ("JG", Instruction::new(0, 0, 6, 39)),
            ("JGE", Instruction::new(0, 0, 7, 39)),
            ("JNE", Instruction::new(0, 0, 8, 39)),
            ("JLE", Instruction::new(0, 0, 9, 39)),
            ("JAN", Instruction::new(0, 0, 0, 40)),
            ("JAZ", Instruction::new(0, 0, 1, 40)),
            ("JAP", Instruction::new(0, 0, 2, 40)),
            ("JANN", Instruction::new(0, 0, 3, 40)),
            ("JANZ", Instruction::new(0, 0, 4, 40)),
            ("JANP", Instruction::new(0, 0, 5, 40)),
            ("J1N", Instruction::new(0, 0, 0, 41)),
            ("J2N", Instruction::new(0, 0, 0, 42)),
            ("J3N", Instruction::new(0, 0, 0, 43)),
            ("J4N", Instruction::new(0, 0, 0, 44)),
            ("J5N", Instruction::new(0, 0, 0, 45)),
            ("J6N", Instruction::new(0, 0, 0, 46)),
            ("J1Z", Instruction::new(0, 0, 1, 41)),
            ("J2Z", Instruction::new(0, 0, 1, 42)),
            ("J3Z", Instruction::new(0, 0, 1, 43)),
            ("J4Z", Instruction::new(0, 0, 1, 44)),
            ("J5Z", Instruction::new(0, 0, 1, 45)),
            ("J6Z", Instruction::new(0, 0, 1, 46)),
            ("J1P", Instruction::new(0, 0, 2, 41)),
            ("J2P", Instruction::new(0, 0, 2, 42)),
            ("J3P", Instruction::new(0, 0, 2, 43)),
            ("J4P", Instruction::new(0, 0, 2, 44)),
            ("J5P", Instruction::new(0, 0, 2, 45)),
            ("J6P", Instruction::new(0, 0, 2, 46)),
            ("J1NN", Instruction::new(0, 0, 3, 41)),
            ("J2NN", Instruction::new(0, 0, 3, 42)),
            ("J3NN", Instruction::new(0, 0, 3, 43)),
            ("J4NN", Instruction::new(0, 0, 3, 44)),
            ("J5NN", Instruction::new(0, 0, 3, 45)),
            ("J6NN", Instruction::new(0, 0, 3, 46)),
            ("J1NZ", Instruction::new(0, 0, 4, 41)),
            ("J2NZ", Instruction::new(0, 0, 4, 42)),
            ("J3NZ", Instruction::new(0, 0, 4, 43)),
            ("J4NZ", Instruction::new(0, 0, 4, 44)),
            ("J5NZ", Instruction::new(0, 0, 4, 45)),
            ("J6NZ", Instruction::new(0, 0, 4, 46)),
            ("J1NP", Instruction::new(0, 0, 5, 41)),
            ("J2NP", Instruction::new(0, 0, 5, 42)),
            ("J3NP", Instruction::new(0, 0, 5, 43)),
            ("J4NP", Instruction::new(0, 0, 5, 44)),
            ("J5NP", Instruction::new(0, 0, 5, 45)),
            ("J6NP", Instruction::new(0, 0, 5, 46)),
            ("JXN", Instruction::new(0, 0, 0, 47)),
            ("JXZ", Instruction::new(0, 0, 1, 47)),
            ("JXP", Instruction::new(0, 0, 2, 47)),
            ("JXNN", Instruction::new(0, 0, 3, 47)),
            ("JXNZ", Instruction::new(0, 0, 4, 47)),
            ("JXNP", Instruction::new(0, 0, 5, 47)),
            ("INCA", Instruction::new(0, 0, 0, 48)),
            ("DECA", Instruction::new(0, 0, 1, 48)),
            ("ENTA", Instruction::new(0, 0, 2, 48)),
            ("ENNA", Instruction::new(0, 0, 3, 48)),
            ("INC1", Instruction::new(0, 0, 0, 49)),
            ("INC2", Instruction::new(0, 0, 0, 50)),
            ("INC3", Instruction::new(0, 0, 0, 51)),
            ("INC4", Instruction::new(0, 0, 0, 52)),
            ("INC5", Instruction::new(0, 0, 0, 53)),
            ("INC6", Instruction::new(0, 0, 0, 54)),
            ("DEC1", Instruction::new(0, 0, 1, 49)),
            ("DEC2", Instruction::new(0, 0, 1, 50)),
            ("DEC3", Instruction::new(0, 0, 1, 51)),
            ("DEC4", Instruction::new(0, 0, 1, 52)),
            ("DEC5", Instruction::new(0, 0, 1, 53)),
            ("DEC6", Instruction::new(0, 0, 1, 54)),
            ("ENT1", Instruction::new(0, 0, 2, 49)),
            ("ENT2", Instruction::new(0, 0, 2, 50)),
            ("ENT3", Instruction::new(0, 0, 2, 51)),
            ("ENT4", Instruction::new(0, 0, 2, 52)),
            ("ENT5", Instruction::new(0, 0, 2, 53)),
            ("ENT6", Instruction::new(0, 0, 2, 54)),
            ("ENN1", Instruction::new(0, 0, 3, 49)),
            ("ENN2", Instruction::new(0, 0, 3, 50)),
            ("ENN3", Instruction::new(0, 0, 3, 51)),
            ("ENN4", Instruction::new(0, 0, 3, 52)),
            ("ENN5", Instruction::new(0, 0, 3, 53)),
            ("ENN6", Instruction::new(0, 0, 3, 54)),
            ("INCX", Instruction::new(0, 0, 0, 55)),
            ("DECX", Instruction::new(0, 0, 1, 55)),
            ("ENTX", Instruction::new(0, 0, 2, 55)),
            ("ENNX", Instruction::new(0, 0, 3, 55)),
            ("CMPA", Instruction::new(0, 0, 5, 56)),
            ("CMP1", Instruction::new(0, 0, 5, 57)),
            ("CMP2", Instruction::new(0, 0, 5, 58)),
            ("CMP3", Instruction::new(0, 0, 5, 59)),
            ("CMP4", Instruction::new(0, 0, 5, 60)),
            ("CMP5", Instruction::new(0, 0, 5, 61)),
            ("CMP6", Instruction::new(0, 0, 5, 62)),
            ("CMPX", Instruction::new(0, 0, 5, 63)),
        ]);

        Tags { instructions }
    }

    pub fn get(&self, name: &str) -> Instruction {
        let inst = self.instructions.get(name).expect("not found"); //TODO: if not found then it could be pseudo-operations
        *inst
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print() {
        let t = Tags::new();
        let mut op = t.get("CMPA");
        op.set_aa(-3000);
        op.set_i(3);
        assert_eq!("-3000,3,5,56",op.print());
    }
}
