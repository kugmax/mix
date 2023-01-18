use crate::memory::short_word::ShortWord;
use crate::memory::swap_sign;
use crate::memory::word::Word;
use crate::memory::word_access::WordAccess;
use crate::memory::Bytes;
use crate::memory::Instruction;
use crate::memory::Sign;
use crate::memory::NEGATIVE;
use crate::memory::POSITIVE;
use crate::registers::RegisterType;
use crate::registers::Registers;

pub trait EnterOperation {
    fn execute(&self, reg: &mut Registers);

    fn enter(instruction: Word, op_sing: Sign, r_type: RegisterType, reg: &mut Registers) {
        let m = instruction.get_address();

        let mut sign = instruction.get_sign();
        if op_sing == NEGATIVE {
            sign = swap_sign(sign);
        }

        let mut result = Word::new_from_signed(m);
        result.set_sign(sign);
        if m != 0 {
            reg.set_reg_by_type(r_type, result);
            return;
        }

        let i = instruction.get_i();
        if i != 0 {
            let ri = reg.get_i(i as usize);
            let mut ra = Word::new(ri.get());
            ra.set_sign(sign);
            reg.set_reg_by_type(r_type, ra);
            return;
        }

        let mut ra = reg.get_reg_by_type(r_type);
        ra.set_sign(sign);
        reg.set_reg_by_type(r_type, ra);
    }
}

struct ENTA {
    code: u32,
    execution_time: u32,
    f: u8,

    instruction: Word,
}

impl ENTA {
    pub fn new(instruction: Word) -> ENTA {
        ENTA {
            code: 48,
            execution_time: 1,
            f: 2,
            instruction: instruction,
        }
    }
}

impl EnterOperation for ENTA {
    fn execute(&self, reg: &mut Registers) {
        <ENTA as EnterOperation>::enter(self.instruction, POSITIVE, RegisterType::A, reg);
    }
}

struct ENTX {
    code: u32,
    execution_time: u32,
    f: u8,

    instruction: Word,
}

impl ENTX {
    pub fn new(instruction: Word) -> ENTX {
        ENTX {
            code: 55,
            execution_time: 1,
            f: 2,
            instruction: instruction,
        }
    }
}

impl EnterOperation for ENTX {
    fn execute(&self, reg: &mut Registers) {
        <ENTX as EnterOperation>::enter(self.instruction, POSITIVE, RegisterType::X, reg);
    }
}

struct ENTi {
    code: u32,
    execution_time: u32,
    f: u8,
    i: u8,

    instruction: Word,
}

impl ENTi {
    pub fn new(instruction: Word) -> ENTi {
        let i = instruction.get_i();
        ENTi {
            code: 48 + i as u32,
            execution_time: 1,
            f: 2,
            i: i,
            instruction: instruction,
        }
    }

    fn execute(&self, reg: &mut Registers) {
        let m = self.instruction.get_address();
        let i = self.instruction.get_i();
        if m != 0 {
            reg.set_i(i as usize, ShortWord::new_from_signed(m));
            return;
        }

        let sign = self.instruction.get_sign();
        let mut ri = reg.get_i(i as usize);
        ri.set_sign(sign);
        reg.set_i(i as usize, ri);
    }
}

struct ENNA {
    code: u32,
    execution_time: u32,
    f: u8,

    instruction: Word,
}

impl ENNA {
    pub fn new(instruction: Word) -> ENNA {
        ENNA {
            code: 48,
            execution_time: 1,
            f: 3,
            instruction: instruction,
        }
    }
}

impl EnterOperation for ENNA {
    fn execute(&self, reg: &mut Registers) {
        <ENNA as EnterOperation>::enter(self.instruction, NEGATIVE, RegisterType::A, reg);
    }
}

struct ENNX {
    code: u32,
    execution_time: u32,
    f: u8,

    instruction: Word,
}

impl ENNX {
    pub fn new(instruction: Word) -> ENNX {
        ENNX {
            code: 55,
            execution_time: 1,
            f: 3,
            instruction: instruction,
        }
    }
}

impl EnterOperation for ENNX {
    fn execute(&self, reg: &mut Registers) {
        <ENNX as EnterOperation>::enter(self.instruction, NEGATIVE, RegisterType::X, reg);
    }
}

struct ENNi {
    code: u32,
    execution_time: u32,
    f: u8,
    i: u8,

    instruction: Word,
}

impl ENNi {
    pub fn new(instruction: Word) -> ENNi {
        let i = instruction.get_i();
        ENNi {
            code: 48 + i as u32,
            execution_time: 1,
            f: 3,
            i: i,
            instruction: instruction,
        }
    }

    fn execute(&self, reg: &mut Registers) {
        let m = self.instruction.get_address();
        let i = self.instruction.get_i();
        let sign = swap_sign(self.instruction.get_sign());

        if m != 0 {
            let mut result = ShortWord::new_from_signed(m);
            result.set_sign(sign);
            reg.set_i(i as usize, result);
            return;
        }

        let mut ri = reg.get_i(i as usize);
        ri.set_sign(sign);
        reg.set_i(i as usize, ri);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enta_zero() {
        let mut r = Registers::new();

        let op = ENTA::new(Word::new_instruction(
            2_001,
            0,
            WordAccess::new_by_spec(2),
            48,
        ));
        op.execute(&mut r);
        assert_eq!(r.get_a(), Word::new_from_signed(2_001));

        let op = ENTA::new(Word::new_instruction(
            -2_001,
            0,
            WordAccess::new_by_spec(2),
            48,
        ));
        op.execute(&mut r);
        assert_eq!(r.get_a(), Word::new_from_signed(-2_001));

        let op = ENTA::new(Word::new_instruction(0, 0, WordAccess::new_by_spec(2), 48));
        op.execute(&mut r);
        assert_eq!(r.get_a(), Word::new_from_signed(2_001));

        let mut instruction = Word::new_instruction(0, 0, WordAccess::new_by_spec(2), 48);
        instruction.set_sign(-1);
        let op = ENTA::new(instruction);
        op.execute(&mut r);
        assert_eq!(r.get_a(), Word::new_from_signed(-2_001));
    }

    #[test]
    fn enta_indexing() {
        let mut r = Registers::new();
        r.set_i(1, ShortWord::new(2_001));

        let op = ENTA::new(Word::new_instruction(0, 1, WordAccess::new_by_spec(2), 48));
        op.execute(&mut r);
        assert_eq!(r.get_a(), Word::new_from_signed(2_001));

        r.set_i(1, ShortWord::new(2_002));
        let mut instruction = Word::new_instruction(0, 1, WordAccess::new_by_spec(2), 48);
        instruction.set_sign(-1);
        let op = ENTA::new(instruction);
        op.execute(&mut r);
        assert_eq!(r.get_a(), Word::new_from_signed(-2_002));
    }

    #[test]
    fn entx_zero() {
        let mut r = Registers::new();

        let op = ENTX::new(Word::new_instruction(
            2_001,
            0,
            WordAccess::new_by_spec(2),
            48,
        ));
        op.execute(&mut r);
        assert_eq!(r.get_x(), Word::new_from_signed(2_001));

        let op = ENTX::new(Word::new_instruction(
            -2_001,
            0,
            WordAccess::new_by_spec(2),
            48,
        ));
        op.execute(&mut r);
        assert_eq!(r.get_x(), Word::new_from_signed(-2_001));

        let op = ENTX::new(Word::new_instruction(0, 0, WordAccess::new_by_spec(2), 48));
        op.execute(&mut r);
        assert_eq!(r.get_x(), Word::new_from_signed(2_001));

        let mut instruction = Word::new_instruction(0, 0, WordAccess::new_by_spec(2), 48);
        instruction.set_sign(-1);
        let op = ENTX::new(instruction);
        op.execute(&mut r);
        assert_eq!(r.get_x(), Word::new_from_signed(-2_001));
    }

    #[test]
    fn entx_indexing() {
        let mut r = Registers::new();
        r.set_i(1, ShortWord::new(2_001));

        let op = ENTX::new(Word::new_instruction(0, 1, WordAccess::new_by_spec(2), 48));
        op.execute(&mut r);
        assert_eq!(r.get_x(), Word::new_from_signed(2_001));

        r.set_i(1, ShortWord::new(2_002));
        let mut instruction = Word::new_instruction(0, 1, WordAccess::new_by_spec(2), 48);
        instruction.set_sign(-1);
        let op = ENTX::new(instruction);
        op.execute(&mut r);
        assert_eq!(r.get_x(), Word::new_from_signed(-2_002));
    }

    #[test]
    fn enti() {
        let mut r = Registers::new();
        r.set_i(1, ShortWord::new(2_001));

        let op = ENTi::new(Word::new_instruction(11, 1, WordAccess::new_by_spec(2), 48));
        op.execute(&mut r);
        assert_eq!(r.get_i(1), ShortWord::new_from_signed(11));

        let op = ENTi::new(Word::new_instruction(
            -12,
            1,
            WordAccess::new_by_spec(2),
            48,
        ));
        op.execute(&mut r);
        assert_eq!(r.get_i(1), ShortWord::new_from_signed(-12));

        let op = ENTi::new(Word::new_instruction(0, 1, WordAccess::new_by_spec(2), 48));
        op.execute(&mut r);
        assert_eq!(r.get_i(1), ShortWord::new_from_signed(12));

        let mut instruction = Word::new_instruction(0, 1, WordAccess::new_by_spec(2), 48);
        instruction.set_sign(-1);
        let op = ENTi::new(instruction);
        op.execute(&mut r);
        assert_eq!(r.get_i(1), ShortWord::new_from_signed(-12));
    }

    #[test]
    fn enna_zero() {
        let mut r = Registers::new();

        let op = ENNA::new(Word::new_instruction(
            2_001,
            0,
            WordAccess::new_by_spec(2),
            48,
        ));
        op.execute(&mut r);
        assert_eq!(r.get_a(), Word::new_from_signed(-2_001));

        let op = ENNA::new(Word::new_instruction(
            -2_001,
            0,
            WordAccess::new_by_spec(2),
            48,
        ));
        op.execute(&mut r);
        assert_eq!(r.get_a(), Word::new_from_signed(2_001));

        let op = ENNA::new(Word::new_instruction(0, 0, WordAccess::new_by_spec(2), 48));
        op.execute(&mut r);
        assert_eq!(r.get_a(), Word::new_from_signed(-2_001));

        let mut instruction = Word::new_instruction(0, 0, WordAccess::new_by_spec(2), 48);
        instruction.set_sign(-1);
        let op = ENNA::new(instruction);
        op.execute(&mut r);
        assert_eq!(r.get_a(), Word::new_from_signed(2_001));
    }

    #[test]
    fn enna_indexing() {
        let mut r = Registers::new();
        r.set_i(1, ShortWord::new(2_001));

        let op = ENNA::new(Word::new_instruction(0, 1, WordAccess::new_by_spec(2), 48));
        op.execute(&mut r);
        assert_eq!(r.get_a(), Word::new_from_signed(-2_001));

        r.set_i(1, ShortWord::new(2_002));
        let mut instruction = Word::new_instruction(0, 1, WordAccess::new_by_spec(2), 48);
        instruction.set_sign(-1);
        let op = ENNA::new(instruction);
        op.execute(&mut r);
        assert_eq!(r.get_a(), Word::new_from_signed(2_002));
    }

    #[test]
    fn ennx_zero() {
        let mut r = Registers::new();

        let op = ENNX::new(Word::new_instruction(
            2_001,
            0,
            WordAccess::new_by_spec(2),
            48,
        ));
        op.execute(&mut r);
        assert_eq!(r.get_x(), Word::new_from_signed(-2_001));

        let op = ENNX::new(Word::new_instruction(
            -2_001,
            0,
            WordAccess::new_by_spec(2),
            48,
        ));
        op.execute(&mut r);
        assert_eq!(r.get_x(), Word::new_from_signed(2_001));

        let op = ENNX::new(Word::new_instruction(0, 0, WordAccess::new_by_spec(2), 48));
        op.execute(&mut r);
        assert_eq!(r.get_x(), Word::new_from_signed(-2_001));

        let mut instruction = Word::new_instruction(0, 0, WordAccess::new_by_spec(2), 48);
        instruction.set_sign(-1);
        let op = ENNX::new(instruction);
        op.execute(&mut r);
        assert_eq!(r.get_x(), Word::new_from_signed(2_001));
    }

    #[test]
    fn ennx_indexing() {
        let mut r = Registers::new();
        r.set_i(1, ShortWord::new(2_001));

        let op = ENNX::new(Word::new_instruction(0, 1, WordAccess::new_by_spec(2), 48));
        op.execute(&mut r);
        assert_eq!(r.get_x(), Word::new_from_signed(-2_001));

        r.set_i(1, ShortWord::new(2_002));
        let mut instruction = Word::new_instruction(0, 1, WordAccess::new_by_spec(2), 48);
        instruction.set_sign(-1);
        let op = ENNX::new(instruction);
        op.execute(&mut r);
        assert_eq!(r.get_x(), Word::new_from_signed(2_002));
    }

    #[test]
    fn enni() {
        let mut r = Registers::new();
        r.set_i(1, ShortWord::new(2_001));

        let op = ENNi::new(Word::new_instruction(11, 1, WordAccess::new_by_spec(2), 48));
        op.execute(&mut r);
        assert_eq!(r.get_i(1), ShortWord::new_from_signed(-11));

        let op = ENNi::new(Word::new_instruction(
            -12,
            1,
            WordAccess::new_by_spec(2),
            48,
        ));
        op.execute(&mut r);
        assert_eq!(r.get_i(1), ShortWord::new_from_signed(12));

        let op = ENNi::new(Word::new_instruction(0, 1, WordAccess::new_by_spec(2), 48));
        op.execute(&mut r);
        assert_eq!(r.get_i(1), ShortWord::new_from_signed(-12));

        let mut instruction = Word::new_instruction(0, 1, WordAccess::new_by_spec(2), 48);
        instruction.set_sign(-1);
        let op = ENNi::new(instruction);
        op.execute(&mut r);
        assert_eq!(r.get_i(1), ShortWord::new_from_signed(12));
    }
}
