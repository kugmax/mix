use crate::memory::word::Bytes;
use crate::memory::word::Instruction;
use crate::memory::word::ShortWord;
use crate::memory::word::Word;
use crate::memory::word::WordAccess;
use crate::memory::Memory;
use crate::registers::Registers;

const MAX_5_BYTES: i32 = 1_073_741_823;

struct ADD {
    code: u32,
    execution_time: u32,

    instruction: Word,
}

impl ADD {
    pub fn new(instruction: Word) -> ADD {
        ADD {
            code: 1,
            execution_time: 2,
            instruction: instruction,
        }
    }

    pub fn execute(&self, mem: &Memory, reg: &mut Registers) {
        let addr = self.instruction.get_address();
        let addr = addr.abs();

        let mem_cell = mem.get(addr as usize);

        let value: i32 =
            Word::new(mem_cell.get_by_access(self.instruction.get_f())).get_signed_value();
        let result: i32 = reg.get_a().get_signed_value() + value;

        if result == 0 {
            // reg.set_overflow(false); //TODO: how to clean overflow flag ?
            let mut result = Word::new(0);
            result.set_sign(reg.get_a().get_sign());
            reg.set_a(result);
            return;
        }

        if result >= -MAX_5_BYTES && result <= MAX_5_BYTES {
            // reg.set_overflow(false); //TODO: how to clean overflow flag ?
            reg.set_a(Word::new_from_signed(result));
            return;
        }

        reg.set_overflow(true);
        reg.set_a(Word::new(0));//TODO: the behaviour have to be different
    }
}

struct SUB {
    code: u32,
    execution_time: u32,

    instruction: Word,
}

impl SUB {
    pub fn new(instruction: Word) -> SUB {
        SUB {
            code: 2,
            execution_time: 2,
            instruction: instruction,
        }
    }

    pub fn execute(&self, mem: &Memory, reg: &mut Registers) {
        let addr = self.instruction.get_address();
        let addr = addr.abs();

        let mem_cell = mem.get(addr as usize);

        let value: i32 =
            Word::new(mem_cell.get_by_access(self.instruction.get_f())).get_signed_value();
        let result: i32 = reg.get_a().get_signed_value() - value;

        if result == 0 {
            // reg.set_overflow(false); //TODO: how to clean overflow flag ?
            let mut result = Word::new(0);
            result.set_sign(reg.get_a().get_sign());
            reg.set_a(result);
            return;
        }

        if result >= -MAX_5_BYTES && result <= MAX_5_BYTES {
            // reg.set_overflow(false); //TODO: how to clean overflow flag ?
            reg.set_a(Word::new_from_signed(result));
            return;
        }

        reg.set_overflow(true);
        reg.set_a(Word::new(0));//TODO: the behaviour have to be different
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let mut m = Memory::new();

        let mem_value = Word::new(1_001);
        m.set(2_000, mem_value.get());

        let mem_value = Word::new_from_signed(-1_001);
        m.set(2_001, mem_value.get());

        let mem_value = Word::new_from_signed(MAX_5_BYTES);
        m.set(2_002, mem_value.get());

        let mem_value = Word::new_from_signed(-MAX_5_BYTES);
        m.set(2_003, mem_value.get());

        let mut r = Registers::new();

        let operation = ADD::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 1));
        operation.execute(&m, &mut r);
        assert_eq!(r.get_a(), Word::new(1_001));
        assert_eq!(r.is_overflow(), false);

        let operation = ADD::new(Word::new_instruction(2_001, 0, WordAccess::new(0, 5), 1));
        operation.execute(&m, &mut r);
        assert_eq!(r.get_a(), Word::new(0));
        assert_eq!(r.is_overflow(), false);

        let operation = ADD::new(Word::new_instruction(2_002, 0, WordAccess::new(0, 5), 1));
        operation.execute(&m, &mut r);
        assert_eq!(r.get_a(), Word::new_from_signed(MAX_5_BYTES));
        assert_eq!(r.is_overflow(), false);

        let operation = ADD::new(Word::new_instruction(2_003, 0, WordAccess::new(0, 5), 1));
        operation.execute(&m, &mut r);
        assert_eq!(r.get_a(), Word::new_from_signed(0));
        assert_eq!(r.is_overflow(), false);
    }
    
    #[test]
    fn add_result_is_0() {
        let mut m = Memory::new();

        let mem_value = Word::new(1);
        m.set(2_000, mem_value.get());

        let mem_value = Word::new_from_signed(-1);
        m.set(2_001, mem_value.get());

        let mut r = Registers::new();

        let operation = ADD::new(Word::new_instruction(2_001, 0, WordAccess::new(0, 5), 1));
        operation.execute(&m, &mut r);
        assert_eq!(r.is_overflow(), false);
        assert_eq!(r.get_a(), Word::new_from_signed(-1));

        let operation = ADD::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 1));
        operation.execute(&m, &mut r);
        assert_eq!(r.is_overflow(), false);
        assert_eq!(r.get_a(), Word::new_by_bytes(-1, &[0, 0, 0, 0, 0]));

        let operation = ADD::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 1));
        operation.execute(&m, &mut r);
        assert_eq!(r.is_overflow(), false);
        assert_eq!(r.get_a(), Word::new_from_signed(1));

        let operation = ADD::new(Word::new_instruction(2_001, 0, WordAccess::new(0, 5), 1));
        operation.execute(&m, &mut r);
        assert_eq!(r.is_overflow(), false);
        assert_eq!(r.get_a(), Word::new_by_bytes(0, &[0, 0, 0, 0, 0]));
    }

    #[test]
    fn add_overflow() {
        let mut m = Memory::new();

        let mem_value = Word::new(2);
        m.set(2_000, mem_value.get());

        let mem_value = Word::new_from_signed(-2);
        m.set(2_001, mem_value.get());

        let mem_value = Word::new_from_signed(MAX_5_BYTES);
        m.set(2_002, mem_value.get());

        let mem_value = Word::new_from_signed(-MAX_5_BYTES);
        m.set(2_003, mem_value.get());

        let mut r = Registers::new();

        let operation = ADD::new(Word::new_instruction(2_002, 0, WordAccess::new(0, 5), 1));
        operation.execute(&m, &mut r);
        assert_eq!(r.is_overflow(), false);
        assert_eq!(r.get_a(), Word::new_from_signed(MAX_5_BYTES));

        let operation = ADD::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 1));
        operation.execute(&m, &mut r);
        assert_eq!(r.is_overflow(), true);
        assert_eq!(r.get_a(), Word::new(0));

        let operation = ADD::new(Word::new_instruction(2_003, 0, WordAccess::new(0, 5), 1));
        operation.execute(&m, &mut r);
        assert_eq!(r.is_overflow(), true);
        assert_eq!(r.get_a(), Word::new_from_signed(-MAX_5_BYTES));

        let operation = ADD::new(Word::new_instruction(2_001, 0, WordAccess::new(0, 5), 1));
        operation.execute(&m, &mut r);
        assert_eq!(r.is_overflow(), true);
        assert_eq!(r.get_a(), Word::new(0));
    }
    
    #[test]
    fn sub() {
        let mut m = Memory::new();

        let mem_value = Word::new(1_001);
        m.set(2_000, mem_value.get());

        let mem_value = Word::new_from_signed(-1_001);
        m.set(2_001, mem_value.get());

        let mem_value = Word::new_from_signed(MAX_5_BYTES);
        m.set(2_002, mem_value.get());

        let mem_value = Word::new_from_signed(-MAX_5_BYTES);
        m.set(2_003, mem_value.get());

        let mut r = Registers::new();

        let operation = SUB::new(Word::new_instruction(2_001, 0, WordAccess::new(0, 5), 1));
        operation.execute(&m, &mut r);
        assert_eq!(r.get_a(), Word::new(1_001));
        assert_eq!(r.is_overflow(), false);

        let operation = SUB::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 1));
        operation.execute(&m, &mut r);
        assert_eq!(r.get_a(), Word::new(0));
        assert_eq!(r.is_overflow(), false);

        let operation = SUB::new(Word::new_instruction(2_003, 0, WordAccess::new(0, 5), 1));
        operation.execute(&m, &mut r);
        assert_eq!(r.get_a(), Word::new_from_signed(MAX_5_BYTES));
        assert_eq!(r.is_overflow(), false);

        let operation = SUB::new(Word::new_instruction(2_002, 0, WordAccess::new(0, 5), 1));
        operation.execute(&m, &mut r);
        assert_eq!(r.get_a(), Word::new_from_signed(0));
        assert_eq!(r.is_overflow(), false);
    }
}
