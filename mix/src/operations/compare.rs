use crate::memory::short_word::ShortWord;
use crate::memory::short_word::MAX_2_BYTES;
use crate::memory::word::Word;
use crate::memory::word_access::WordAccess;
use crate::memory::Bytes;
use crate::memory::Instruction;
use crate::memory::Memory;
use crate::memory::Sign;
use crate::memory::NEGATIVE;
use crate::memory::POSITIVE;
use crate::operations::get_memory_cell;
use crate::registers::Comparison;
use crate::registers::RegisterType;
use crate::registers::Registers;

pub trait CompareOperation {
    fn execute(&self, mem: &Memory, reg: &mut Registers);

    fn compare(
        instruction: impl Instruction,
        r_type: RegisterType,
        mem: &Memory,
        reg: &mut Registers,
    ) {
        let f = instruction.get_f();
        if f.spec == 0 {
            reg.set_comparison(Comparison::EQUAL);
            return;
        }

        let mem_cell = get_memory_cell(instruction, mem, reg);
        let mem_value = Word::new(mem_cell.get_by_access(f)).get_signed_value();

        let reg_cell = reg.get_reg_by_type(r_type);
        let reg_value = Word::new(reg_cell.get_by_access(f)).get_signed_value();

        if reg_value > mem_value {
            reg.set_comparison(Comparison::GREATHER);
        } else if reg_value < mem_value {
            reg.set_comparison(Comparison::LESS);
        } else {
            reg.set_comparison(Comparison::EQUAL);
        }
    }
}

struct CMPA {
    code: u32,
    execution_time: u32,

    instruction: Word,
}

impl CMPA {
    pub fn new(instruction: Word) -> CMPA {
        CMPA {
            code: 56,
            execution_time: 2,
            instruction: instruction,
        }
    }
}

impl CompareOperation for CMPA {
    fn execute(&self, mem: &Memory, reg: &mut Registers) {
        <CMPA as CompareOperation>::compare(self.instruction, RegisterType::A, mem, reg);
    }
}

struct CMPX {
    code: u32,
    execution_time: u32,

    instruction: Word,
}

impl CMPX {
    pub fn new(instruction: Word) -> CMPX {
        CMPX {
            code: 63,
            execution_time: 2,
            instruction: instruction,
        }
    }
}

impl CompareOperation for CMPX {
    fn execute(&self, mem: &Memory, reg: &mut Registers) {
        <CMPX as CompareOperation>::compare(self.instruction, RegisterType::X, mem, reg);
    }
}

struct CMPi {
    code: u32,
    execution_time: u32,
    i: u8,

    instruction: Word,
}

impl CMPi {
    pub fn new(instruction: Word) -> CMPi {
        let i = instruction.get_i();
        CMPi {
            code: 56 + i as u32,
            execution_time: 2,
            i:i,
            instruction: instruction,
        }
    }

    pub fn execute(&self, mem: &Memory, reg: &mut Registers) {
        let f = self.instruction.get_f();
        if f.spec == 0 {
            reg.set_comparison(Comparison::EQUAL);
            return;
        }

        let mut addr = self.instruction.get_address();
        addr = addr.abs();

        let mem_cell = mem.get(addr as usize);
        let mem_value = Word::new(mem_cell.get_by_access(f)).get_signed_value();

        let reg_cell = reg.get_i(self.instruction.get_i() as usize);
        let reg_value = ShortWord::new(reg_cell.get_by_access(f)).get_signed_value();

        if reg_value > mem_value {
            reg.set_comparison(Comparison::GREATHER);
        } else if reg_value < mem_value {
            reg.set_comparison(Comparison::LESS);
        } else {
            reg.set_comparison(Comparison::EQUAL);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmpa() {
        let mut m = Memory::new();
        m.set(2_001, Word::new_from_signed(1).get());
        m.set(2_002, Word::new_from_signed(2).get());
        m.set(2_003, Word::new_from_signed(-1).get());
        m.set(2_004, Word::new_from_signed(-2).get());

        let mut r = Registers::new();
        r.set_i(1, ShortWord::new(1));
        r.set_i(2, ShortWord::new(2));
        r.set_i(3, ShortWord::new(3));
        r.set_i(4, ShortWord::new(4));

        r.set_a(Word::new_from_signed(1));
        let operation = CMPA::new(Word::new_instruction(2_000, 1, WordAccess::new(0, 5), 56));
        operation.execute(&m, &mut r);
        assert_eq!(r.get_comparison(), Comparison::EQUAL);

        let operation = CMPA::new(Word::new_instruction(2_000, 2, WordAccess::new(0, 5), 56));
        operation.execute(&m, &mut r);
        assert_eq!(r.get_comparison(), Comparison::LESS);

        let operation = CMPA::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 56));
        operation.execute(&m, &mut r);
        assert_eq!(r.get_comparison(), Comparison::GREATHER);

        r.set_a(Word::new_from_signed(-1));
        let operation = CMPA::new(Word::new_instruction(2_000, 3, WordAccess::new(0, 5), 56));
        operation.execute(&m, &mut r);
        assert_eq!(r.get_comparison(), Comparison::EQUAL);

        let operation = CMPA::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 56));
        operation.execute(&m, &mut r);
        assert_eq!(r.get_comparison(), Comparison::LESS);

        let operation = CMPA::new(Word::new_instruction(2_000, 4, WordAccess::new(0, 5), 56));
        operation.execute(&m, &mut r);
        assert_eq!(r.get_comparison(), Comparison::GREATHER);
    }

    #[test]
    fn cmpx() {
        let mut m = Memory::new();
        m.set(2_001, Word::new_from_signed(1).get());
        m.set(2_002, Word::new_from_signed(2).get());
        m.set(2_003, Word::new_from_signed(-1).get());
        m.set(2_004, Word::new_from_signed(-2).get());

        let mut r = Registers::new();
        r.set_i(1, ShortWord::new(1));
        r.set_i(2, ShortWord::new(2));
        r.set_i(3, ShortWord::new(3));
        r.set_i(4, ShortWord::new(4));

        r.set_x(Word::new_from_signed(1));
        let operation = CMPX::new(Word::new_instruction(2_000, 1, WordAccess::new(0, 5), 56));
        operation.execute(&m, &mut r);
        assert_eq!(r.get_comparison(), Comparison::EQUAL);

        let operation = CMPX::new(Word::new_instruction(2_000, 2, WordAccess::new(0, 5), 56));
        operation.execute(&m, &mut r);
        assert_eq!(r.get_comparison(), Comparison::LESS);

        let operation = CMPX::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 56));
        operation.execute(&m, &mut r);
        assert_eq!(r.get_comparison(), Comparison::GREATHER);

        r.set_x(Word::new_from_signed(-1));
        let operation = CMPX::new(Word::new_instruction(2_000, 3, WordAccess::new(0, 5), 56));
        operation.execute(&m, &mut r);
        assert_eq!(r.get_comparison(), Comparison::EQUAL);

        let operation = CMPX::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 56));
        operation.execute(&m, &mut r);
        assert_eq!(r.get_comparison(), Comparison::LESS);

        let operation = CMPX::new(Word::new_instruction(2_000, 4, WordAccess::new(0, 5), 56));
        operation.execute(&m, &mut r);
        assert_eq!(r.get_comparison(), Comparison::GREATHER);
    }

    #[test]
    fn cmpi() {
        let mut m = Memory::new();
        m.set(2_001, Word::new_from_signed(1).get());
        m.set(2_002, Word::new_from_signed(2).get());
        m.set(2_003, Word::new_from_signed(-1).get());
        m.set(2_004, Word::new_from_signed(-2).get());

        let mut r = Registers::new();
        r.set_i(1, ShortWord::new(1));
        r.set_i(2, ShortWord::new(2));
        r.set_i(3, ShortWord::new_from_signed(-1));
        r.set_i(4, ShortWord::new_from_signed(-2));

        let operation = CMPi::new(Word::new_instruction(2_001, 1, WordAccess::new(0, 5), 56));
        operation.execute(&m, &mut r);
        assert_eq!(r.get_comparison(), Comparison::EQUAL);

        let operation = CMPi::new(Word::new_instruction(2_001, 2, WordAccess::new(0, 5), 56));
        operation.execute(&m, &mut r);
        assert_eq!(r.get_comparison(), Comparison::GREATHER);

        let operation = CMPi::new(Word::new_instruction(2_001, 3, WordAccess::new(0, 5), 56));
        operation.execute(&m, &mut r);
        assert_eq!(r.get_comparison(), Comparison::LESS);

        let operation = CMPi::new(Word::new_instruction(2_004, 3, WordAccess::new(0, 5), 56));
        operation.execute(&m, &mut r);
        assert_eq!(r.get_comparison(), Comparison::GREATHER);

        let operation = CMPi::new(Word::new_instruction(2_003, 4, WordAccess::new(0, 5), 56));
        operation.execute(&m, &mut r);
        assert_eq!(r.get_comparison(), Comparison::LESS);

        let operation = CMPi::new(Word::new_instruction(2_004, 4, WordAccess::new(0, 5), 56));
        operation.execute(&m, &mut r);
        assert_eq!(r.get_comparison(), Comparison::EQUAL);

        let operation = CMPi::new(Word::new_instruction(2_000, 1, WordAccess::new(0, 0), 56));
        operation.execute(&m, &mut r);
        assert_eq!(r.get_comparison(), Comparison::EQUAL);
    }
}
