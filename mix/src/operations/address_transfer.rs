use crate::memory::short_word::ShortWord;
use crate::memory::word::Word;
use crate::memory::word_access::WordAccess;
use crate::memory::Bytes;
use crate::memory::Instruction;
use crate::registers::Registers;

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

    fn execute(&self, reg: &mut Registers) {
        let m = self.instruction.get_address();
        if m != 0 {
            reg.set_a(Word::new_from_signed(m));
            return;
        }

        let sign = self.instruction.get_sign();
        let i = self.instruction.get_i();
        if i != 0 {
            let ri = reg.get_i(i as usize);
            let mut ra = Word::new(ri.get());
            ra.set_sign(sign);
            reg.set_a(ra);
            return;
        }

        let mut ra = reg.get_a();
        ra.set_sign(sign);
        reg.set_a(ra);
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
}
