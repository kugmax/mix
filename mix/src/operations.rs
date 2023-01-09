use crate::memory::word::Instruction;
use crate::memory::word::Word;
use crate::memory::word::WordAccess;
use crate::memory::Memory;
use crate::registers::Registers;

struct LDA {
    code: u32,
    execution_time: u32,

    instruction: Word,
}

impl LDA {
    pub fn new(instruction: Word) -> LDA {
        LDA {
            code: 8,
            execution_time: 2,
            instruction,
        }
    }

    pub fn execute(&self, mem: &Memory, reg: &mut Registers) {
        let addr = self.instruction.get_address();
        let addr = addr.abs();

        let mem_cell = mem.get(addr as usize);

        let value = Word::new(mem_cell.get_by_access(self.instruction.get_f()));

        reg.set_a(value);
    }
}
struct LDX {
    code: u32,
    execution_time: u32,

    instruction: Word,
}

impl LDX {
    pub fn new(instruction: Word) -> LDX {
        LDX {
            code: 15,
            execution_time: 2,
            instruction,
        }
    }

    pub fn execute(&self, mem: &Memory, reg: &mut Registers) {
        let addr = self.instruction.get_address();
        let addr = addr.abs();

        let mem_cell = mem.get(addr as usize);

        let value = Word::new(mem_cell.get_by_access(self.instruction.get_f()));

        reg.set_x(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lda() {
        let word = Word::new_instruction(-80, 3, WordAccess::new(0, 5), 4);

        let mut m = Memory::new();
        m.set(2_000, word.get());

        let mut r = Registers::new();

        let lda = LDA::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 8));
        lda.execute(&m, &mut r);
        assert_instruction(r.get_a(), -80, 3, WordAccess::new(0, 5), 4);

        let lda = LDA::new(Word::new_instruction(2_000, 0, WordAccess::new(1, 5), 8));
        lda.execute(&m, &mut r);
        assert_instruction(r.get_a(), 80, 3, WordAccess::new(0, 5), 4);

        let lda = LDA::new(Word::new_instruction(2_000, 0, WordAccess::new(3, 5), 8));
        lda.execute(&m, &mut r);
        assert_instruction(r.get_a(), 0, 3, WordAccess::new(0, 5), 4);

        let lda = LDA::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 3), 8));
        lda.execute(&m, &mut r);
        assert_eq!(
            r.get_a().get_by_access(WordAccess::new(0, 0)),
            0b10_000000_000000_000000_000000_000000
        );
        assert_eq!(r.get_a().get_by_access(WordAccess::new(1, 2)), 0);
        assert_eq!(r.get_a().get_by_access(WordAccess::new(3, 4)), 80);
        assert_eq!(r.get_a().get_by_access(WordAccess::new(5, 5)), 3);

        let lda = LDA::new(Word::new_instruction(2_000, 0, WordAccess::new(4, 4), 8));
        lda.execute(&m, &mut r);
        assert_instruction(r.get_a(), 0, 0, WordAccess::new(0, 0), 5);

        let lda = LDA::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 0), 8));
        lda.execute(&m, &mut r);
        assert_eq!(
            r.get_a().get_by_access(WordAccess::new(0, 0)),
            0b10_000000_000000_000000_000000_000000
        );
        assert_eq!(r.get_a().get_by_access(WordAccess::new(1, 5)), 0);

        let lda = LDA::new(Word::new_instruction(2_000, 0, WordAccess::new(1, 1), 8));
        lda.execute(&m, &mut r);
        assert_eq!(
            r.get_a().get_by_access(WordAccess::new(0, 0)),
            0b00_000000_000000_000000_000000_000000
        );
        assert_eq!(r.get_a().get_by_access(WordAccess::new(1, 5)), 1);
    }
    #[test]
    fn ldx() {
        let word = Word::new_instruction(-80, 3, WordAccess::new(0, 5), 4);

        let mut m = Memory::new();
        m.set(2_000, word.get());

        let mut r = Registers::new();

        let ldx = LDX::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 8));
        ldx.execute(&m, &mut r);
        assert_instruction(r.get_x(), -80, 3, WordAccess::new(0, 5), 4);
    }

    fn assert_instruction(actual: Word, address: i32, i: u8, f: WordAccess, c: u8) {
        assert_eq!(actual.get_address(), address, "address is wrong");
        assert_eq!(actual.get_i(), i, "i is wrong");
        assert_eq!(actual.get_f().spec, f.spec, "spec is wrong");
        assert_eq!(actual.get_c(), c, "c is wrong");
    }
}