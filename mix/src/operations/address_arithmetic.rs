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
use crate::registers::RegisterType;
use crate::registers::Registers;

trait IncOperation {
    fn execute(&self, reg: &mut Registers);

    fn inc(
        instruction: Word,
        op: &mut dyn Fn(i32, i32) -> i32,
        r_type: RegisterType,
        reg: &mut Registers,
    ) {
        let m = instruction.get_address();

        let reg_value = reg.get_reg_by_type(r_type);
        let result: i32 = op(reg_value.get_signed_value(), m);

        if result == 0 {
            let mut result = Word::new(0);
            result.set_sign(reg_value.get_sign());
            reg.set_reg_by_type(r_type, result);
            return;
        }

        if result >= -MAX_2_BYTES && result <= MAX_2_BYTES {
            reg.set_reg_by_type(r_type, Word::new_from_signed(result));
            return;
        }

        reg.set_overflow(true);
        reg.set_reg_by_type(r_type, Word::new(0));
    }
}

struct INCA {
    code: u32,
    execution_time: u32,
    f: u8,

    instruction: Word,
}

impl INCA {
    pub fn new(instruction: Word) -> INCA {
        INCA {
            code: 48,
            execution_time: 1,
            f: 0,
            instruction: instruction,
        }
    }
}

impl IncOperation for INCA {
    fn execute(&self, reg: &mut Registers) {
        let mut sum = |v1, v2| v1 + v2;
        <INCA as IncOperation>::inc(self.instruction, &mut sum, RegisterType::A, reg);
    }
}

struct INCX {
    code: u32,
    execution_time: u32,
    f: u8,

    instruction: Word,
}

impl INCX {
    pub fn new(instruction: Word) -> INCX {
        INCX {
            code: 55,
            execution_time: 1,
            f: 0,
            instruction: instruction,
        }
    }
}

impl IncOperation for INCX {
    fn execute(&self, reg: &mut Registers) {
        let mut sum = |v1, v2| v1 + v2;
        <INCX as IncOperation>::inc(self.instruction, &mut sum, RegisterType::X, reg);
    }
}

struct DNCA {
    code: u32,
    execution_time: u32,
    f: u8,

    instruction: Word,
}

impl DNCA {
    pub fn new(instruction: Word) -> DNCA {
        DNCA {
            code: 48,
            execution_time: 1,
            f: 1,
            instruction: instruction,
        }
    }
}

impl IncOperation for DNCA {
    fn execute(&self, reg: &mut Registers) {
        let mut sum = |v1, v2| v1 - v2;
        <DNCA as IncOperation>::inc(self.instruction, &mut sum, RegisterType::A, reg);
    }
}

struct DNCX {
    code: u32,
    execution_time: u32,
    f: u8,

    instruction: Word,
}

impl DNCX {
    pub fn new(instruction: Word) -> DNCX {
        DNCX {
            code: 55,
            execution_time: 1,
            f: 1,
            instruction: instruction,
        }
    }
}

impl IncOperation for DNCX {
    fn execute(&self, reg: &mut Registers) {
        let mut sum = |v1, v2| v1 - v2;
        <DNCX as IncOperation>::inc(self.instruction, &mut sum, RegisterType::X, reg);
    }
}

struct INCi {
    code: u32,
    execution_time: u32,
    f: u8,
    i: u8,

    instruction: Word,
}

impl INCi {
    pub fn new(instruction: Word) -> INCi {
        let i = instruction.get_i();
        INCi {
            code: 48+i as u32,
            execution_time: 1,
            f: 0,
            i: i,
            instruction: instruction,
        }
    }

    fn execute(&self, reg: &mut Registers) {
        let m = self.instruction.get_address();

        let reg_value = reg.get_i(self.i as usize);
        let result: i32 = m + reg_value.get_signed_value();

        if result == 0 {
            let mut result = ShortWord::new(0);
            result.set_sign(reg_value.get_sign());
            reg.set_i(self.i as usize, result);
            return;
        }

        if result >= -MAX_2_BYTES && result <= MAX_2_BYTES {
            reg.set_i(self.i as usize, ShortWord::new_from_signed(result));
            return;
        }

        panic!("INCi overflow {m}");
    }
}

struct DNCi {
    code: u32,
    execution_time: u32,
    f: u8,
    i: u8,

    instruction: Word,
}

impl DNCi {
    pub fn new(instruction: Word) -> DNCi {
        let i = instruction.get_i();
        DNCi {
            code: 48+i as u32,
            execution_time: 1,
            f: 1,
            i: i,
            instruction: instruction,
        }
    }

    fn execute(&self, reg: &mut Registers) {
        let m = self.instruction.get_address();

        let reg_value = reg.get_i(self.i as usize);
        let result: i32 = reg_value.get_signed_value() - m;

        if result == 0 {
            let mut result = ShortWord::new(0);
            result.set_sign(reg_value.get_sign());
            reg.set_i(self.i as usize, result);
            return;
        }

        if result >= -MAX_2_BYTES && result <= MAX_2_BYTES {
            reg.set_i(self.i as usize, ShortWord::new_from_signed(result));
            return;
        }

        panic!("DNCi overflow {m}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inca() {
        let mut r = Registers::new();

        let operation = INCA::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_a(), Word::new(2_000));
        assert_eq!(r.is_overflow(), false);

        let operation = INCA::new(Word::new_instruction(-500, 0, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_a(), Word::new(1_500));
        assert_eq!(r.is_overflow(), false);

        let operation = INCA::new(Word::new_instruction(-1_500, 0, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_a(), Word::new(0));
        assert_eq!(r.get_a().get_sign(), 0);
        assert_eq!(r.is_overflow(), false);
        
        let operation = INCA::new(Word::new_instruction(-1_500, 0, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_a(), Word::new_from_signed(-1_500));
        assert_eq!(r.get_a().get_sign(), -1);
        assert_eq!(r.is_overflow(), false);

        let operation = INCA::new(Word::new_instruction(1_500, 0, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_a().get_signed_value(), 0);
        assert_eq!(r.get_a().get_sign(), -1);
        assert_eq!(r.is_overflow(), false);
        
        r.set_a(Word::new(1));
        let operation = INCA::new(Word::new_instruction(MAX_2_BYTES, 0, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_a().get_signed_value(), 0);
        assert_eq!(r.get_a().get_sign(), 0);
        assert_eq!(r.is_overflow(), true);

        r.set_a(Word::new_from_signed(-2));
        r.set_overflow(false);
        let operation = INCA::new(Word::new_instruction(-MAX_2_BYTES, 0, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_a().get_signed_value(), 0);
        assert_eq!(r.get_a().get_sign(), 0);
        assert_eq!(r.is_overflow(), true);
    }

    #[test]
    fn incx() {
        let mut r = Registers::new();

        let operation = INCX::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_x(), Word::new(2_000));
        assert_eq!(r.is_overflow(), false);

        let operation = INCX::new(Word::new_instruction(-500, 0, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_x(), Word::new(1_500));
        assert_eq!(r.is_overflow(), false);

        let operation = INCX::new(Word::new_instruction(-1_500, 0, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_x(), Word::new(0));
        assert_eq!(r.get_x().get_sign(), 0);
        assert_eq!(r.is_overflow(), false);
        
        let operation = INCX::new(Word::new_instruction(-1_500, 0, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_x(), Word::new_from_signed(-1_500));
        assert_eq!(r.get_x().get_sign(), -1);
        assert_eq!(r.is_overflow(), false);

        let operation = INCX::new(Word::new_instruction(1_500, 0, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_x().get_signed_value(), 0);
        assert_eq!(r.get_x().get_sign(), -1);
        assert_eq!(r.is_overflow(), false);
        
        r.set_x(Word::new(1));
        let operation = INCX::new(Word::new_instruction(MAX_2_BYTES, 0, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_x().get_signed_value(), 0);
        assert_eq!(r.get_x().get_sign(), 0);
        assert_eq!(r.is_overflow(), true);

        r.set_x(Word::new_from_signed(-2));
        r.set_overflow(false);
        let operation = INCX::new(Word::new_instruction(-MAX_2_BYTES, 0, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_x().get_signed_value(), 0);
        assert_eq!(r.get_x().get_sign(), 0);
        assert_eq!(r.is_overflow(), true);
    }

    #[test]
    fn dnca() {
        let mut r = Registers::new();

        let operation = DNCA::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_a(), Word::new_from_signed(-2_000));
        assert_eq!(r.is_overflow(), false);

        let operation = DNCA::new(Word::new_instruction(-500, 0, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_a(), Word::new_from_signed(-1_500));
        assert_eq!(r.is_overflow(), false);

        let operation = DNCA::new(Word::new_instruction(-1_500, 0, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_a().get_signed_value(), 0);
        assert_eq!(r.get_a().get_sign(), -1);
        assert_eq!(r.is_overflow(), false);
        
        let operation = DNCA::new(Word::new_instruction(-1_500, 0, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_a(), Word::new_from_signed(1_500));
        assert_eq!(r.get_a().get_sign(), 0);
        assert_eq!(r.is_overflow(), false);

        let operation = DNCA::new(Word::new_instruction(1_500, 0, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_a().get_signed_value(), 0);
        assert_eq!(r.get_a().get_sign(), 0);
        assert_eq!(r.is_overflow(), false);
        
        r.set_a(Word::new_from_signed(-2));
        let operation = DNCA::new(Word::new_instruction(MAX_2_BYTES, 0, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_a().get_signed_value(), 0);
        assert_eq!(r.get_a().get_sign(), 0);
        assert_eq!(r.is_overflow(), true);

        r.set_a(Word::new_from_signed(1));
        r.set_overflow(false);
        let operation = DNCA::new(Word::new_instruction(-MAX_2_BYTES, 0, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_a().get_signed_value(), 0);
        assert_eq!(r.get_a().get_sign(), 0);
        assert_eq!(r.is_overflow(), true);
    }

    #[test]
    fn dncx() {
        let mut r = Registers::new();

        let operation = DNCX::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_x(), Word::new_from_signed(-2_000));
        assert_eq!(r.is_overflow(), false);

        let operation = DNCX::new(Word::new_instruction(-500, 0, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_x(), Word::new_from_signed(-1_500));
        assert_eq!(r.is_overflow(), false);

        let operation = DNCX::new(Word::new_instruction(-1_500, 0, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_x().get_signed_value(), 0);
        assert_eq!(r.get_x().get_sign(), -1);
        assert_eq!(r.is_overflow(), false);
        
        let operation = DNCX::new(Word::new_instruction(-1_500, 0, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_x(), Word::new_from_signed(1_500));
        assert_eq!(r.get_x().get_sign(), 0);
        assert_eq!(r.is_overflow(), false);

        let operation = DNCX::new(Word::new_instruction(1_500, 0, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_x().get_signed_value(), 0);
        assert_eq!(r.get_x().get_sign(), 0);
        assert_eq!(r.is_overflow(), false);
        
        r.set_x(Word::new_from_signed(-2));
        let operation = DNCX::new(Word::new_instruction(MAX_2_BYTES, 0, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_x().get_signed_value(), 0);
        assert_eq!(r.get_x().get_sign(), 0);
        assert_eq!(r.is_overflow(), true);

        r.set_x(Word::new_from_signed(1));
        r.set_overflow(false);
        let operation = DNCX::new(Word::new_instruction(-MAX_2_BYTES, 0, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_x().get_signed_value(), 0);
        assert_eq!(r.get_x().get_sign(), 0);
        assert_eq!(r.is_overflow(), true);
    }

    #[test]
    fn inci() {
        let mut r = Registers::new();

        let operation = INCi::new(Word::new_instruction(2_000, 2, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_i(2), ShortWord::new(2_000));
        assert_eq!(r.is_overflow(), false);

        let operation = INCi::new(Word::new_instruction(-2_000, 2, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_i(2), ShortWord::new_from_signed(0));
        assert_eq!(r.is_overflow(), false);

        let operation = INCi::new(Word::new_instruction(-2_000, 2, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_i(2), ShortWord::new_from_signed(-2_000));
        assert_eq!(r.is_overflow(), false);

        let operation = INCi::new(Word::new_instruction(2_000, 2, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_i(2).get_signed_value(), 0);
        assert_eq!(r.get_i(2).get_sign(), -1);
        assert_eq!(r.is_overflow(), false);
    }

    #[test]
    fn dnci() {
        let mut r = Registers::new();

        let operation = DNCi::new(Word::new_instruction(-2_000, 2, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_i(2), ShortWord::new(2_000));
        assert_eq!(r.is_overflow(), false);

        let operation = DNCi::new(Word::new_instruction(2_000, 2, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_i(2), ShortWord::new_from_signed(0));
        assert_eq!(r.is_overflow(), false);

        let operation = DNCi::new(Word::new_instruction(2_000, 2, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_i(2), ShortWord::new_from_signed(-2_000));
        assert_eq!(r.is_overflow(), false);

        let operation = DNCi::new(Word::new_instruction(-2_000, 2, WordAccess::new(0, 0), 48));
        operation.execute(&mut r);
        assert_eq!(r.get_i(2).get_signed_value(), 0);
        assert_eq!(r.get_i(2).get_sign(), -1);
        assert_eq!(r.is_overflow(), false);
    }
}
