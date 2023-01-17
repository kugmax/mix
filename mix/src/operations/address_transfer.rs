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
        // TODO: -0
        if m != 0 {
            reg.set_a(Word::new_from_signed(m));
            return;
        }

        let i = self.instruction.get_i();
        let ri = reg.get_i(i as usize);
        let mut ra = Word::new(ri.get());
        // ra.set_sign(); //TODO: -0
        reg.set_a(ra);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enta() {}
}
