use crate::memory::word::Instruction;
use crate::memory::word::Word;
use crate::memory::word::ShortWord;
use crate::memory::word::WordAccess;
use crate::memory::Memory;
use crate::registers::Registers;

pub trait LoadOperation {
    fn execute(&self, mem: &Memory, reg: &mut Registers);

    fn load(instruction: impl Instruction, set_reg: &mut dyn FnMut(Word), mem: &Memory) {
        let addr = instruction.get_address();
        let addr = addr.abs();

        let mem_cell = mem.get(addr as usize);

        let value = Word::new(mem_cell.get_by_access(instruction.get_f()));

        set_reg(value);
    }
}

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
}

impl LoadOperation for LDA {
    fn execute(&self, mem: &Memory, reg: &mut Registers) {
        let mut set_a = |w| reg.set_a(w);
        <LDA as LoadOperation>::load(self.instruction, &mut set_a, mem);
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
}

impl LoadOperation for LDX {
    fn execute(&self, mem: &Memory, reg: &mut Registers) {
        let mut set = |w| reg.set_x(w);
        <LDA as LoadOperation>::load(self.instruction, &mut set, mem);
    }
}

struct LDi {
    code: u32,
    execution_time: u32,
    i: u8,

    instruction: Word,
}

impl LDi {
    pub fn new(instruction: Word) -> LDi {
        let i = instruction.get_i();
        LDi {
            code: 8 + i as u32,
            execution_time: 2,
            i: i,
            instruction,
        }
    }

// TODO: possibly combine with LDA,LDX
    pub fn execute(&self, mem: &Memory, reg: &mut Registers) {
        let addr = self.instruction.get_address();
        let addr = addr.abs();

        let mem_cell = mem.get(addr as usize);

        let value = ShortWord::new(mem_cell.get_by_access(self.instruction.get_f()));

        reg.set_i(self.i as usize, value);
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

    #[test]
    fn ldi() {
        let word = Word::new_instruction(-80, 3, WordAccess::new(0, 5), 4);

        let mut m = Memory::new();
        m.set(2_000, word.get());

        let mut r = Registers::new();
        println!("registers are created");

        let load = LDi::new(Word::new_instruction(2_000, 1, WordAccess::new(0, 5), 8));
        load.execute(&m, &mut r);

        let ri = r.get_i(1);

        assert_ne!(ri.get_byte(0), 0, "ri sign is wrogn");
        assert_eq!(ri.get_byte(1), 0, "ri byte 1 is wrong");
        assert_eq!(ri.get_byte(2), 0, "ri byte 2 is wrong");
        assert_eq!(ri.get_byte(3), 0, "ri byte 3 is wrong");
        assert_eq!(ri.get_byte(4), 5, "ri byte 4 is wrong");
        assert_eq!(ri.get_byte(5), 4, "ri byte 5 is wrong");
        
    }

    fn assert_instruction(actual: Word, address: i32, i: u8, f: WordAccess, c: u8) {
        assert_eq!(actual.get_address(), address, "address is wrong");
        assert_eq!(actual.get_i(), i, "i is wrong");
        assert_eq!(actual.get_f().spec, f.spec, "spec is wrong");
        assert_eq!(actual.get_c(), c, "c is wrong");
    }
}
