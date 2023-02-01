use crate::memory::short_word::ShortWord;
use crate::memory::short_word::MAX_2_BYTES;
use crate::memory::word::Word;
use crate::memory::Bytes;
use crate::memory::Instruction;
use crate::operations::*;
use crate::registers::RegisterType;
use crate::registers::Registers;

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

pub struct INCA {
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

impl Operation for INCA {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let mut sum = |v1, v2| v1 + v2;
        inc(self.instruction, &mut sum, RegisterType::A, args.reg);

        OperationResult::from_args(self.execution_time, args)
    }

    fn get_name(&self) -> String {
        String::from("INCA")
    }
}

pub struct INCX {
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

impl Operation for INCX {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let mut sum = |v1, v2| v1 + v2;
        inc(self.instruction, &mut sum, RegisterType::X, args.reg);

        OperationResult::from_args(self.execution_time, args)
    }
    fn get_name(&self) -> String {
        String::from("INCX")
    }
}

pub struct DECA {
    code: u32,
    execution_time: u32,
    f: u8,
    instruction: Word,
}

impl DECA {
    pub fn new(instruction: Word) -> DECA {
        DECA {
            code: 48,
            execution_time: 1,
            f: 1,
            instruction: instruction,
        }
    }
}

impl Operation for DECA {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let mut sum = |v1, v2| v1 - v2;
        inc(self.instruction, &mut sum, RegisterType::A, args.reg);

        OperationResult::from_args(self.execution_time, args)
    }
    fn get_name(&self) -> String {
        String::from("DECA")
    }
}

pub struct DECX {
    code: u32,
    execution_time: u32,
    f: u8,
    instruction: Word,
}

impl DECX {
    pub fn new(instruction: Word) -> DECX {
        DECX {
            code: 55,
            execution_time: 1,
            f: 1,
            instruction: instruction,
        }
    }
}

impl Operation for DECX {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let mut sum = |v1, v2| v1 - v2;
        inc(self.instruction, &mut sum, RegisterType::X, args.reg);

        OperationResult::from_args(self.execution_time, args)
    }
    fn get_name(&self) -> String {
        String::from("DECX")
    }
}

pub struct INCi {
    code: u32,
    execution_time: u32,
    f: u8,
    instruction: Word,
}

impl INCi {
    pub fn new(instruction: Word) -> INCi {
        INCi {
            code: 48,
            execution_time: 1,
            f: 0,
            instruction: instruction,
        }
    }
}

impl Operation for INCi {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let i = (self.instruction.get_c() - self.code as u8) as usize;
        let m = self.instruction.get_address(); // it's value not address

        let reg_value = args.reg.get_i(i);
        let result: i32 = m + reg_value.get_signed_value();

        if result == 0 {
            let mut result = ShortWord::new(0);
            result.set_sign(reg_value.get_sign());
            args.reg.set_i(i as usize, result);
            return OperationResult::from_args(self.execution_time, args);
        }

        if result >= -MAX_2_BYTES && result <= MAX_2_BYTES {
            args.reg
                .set_i(i as usize, ShortWord::new_from_signed(result));
            return OperationResult::from_args(self.execution_time, args);
        }

        panic!("INCi overflow {m}");
    }

    fn get_name(&self) -> String {
        String::from("INCi")
    }
}

pub struct DECi {
    code: u32,
    execution_time: u32,
    f: u8,
    instruction: Word,
}

impl DECi {
    pub fn new(instruction: Word) -> DECi {
        DECi {
            code: 48,
            execution_time: 1,
            f: 1,
            instruction: instruction,
        }
    }
}

impl Operation for DECi {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let i = (self.instruction.get_c() - self.code as u8) as usize;
        let m = self.instruction.get_address(); // it's value not address

        let reg_value = args.reg.get_i(i);
        let result: i32 = reg_value.get_signed_value() - m;

        if result == 0 {
            let mut result = ShortWord::new(0);
            result.set_sign(reg_value.get_sign());
            args.reg.set_i(i, result);
            return OperationResult::from_args(self.execution_time, args);
        }

        if result >= -MAX_2_BYTES && result <= MAX_2_BYTES {
            args.reg.set_i(i, ShortWord::new_from_signed(result));
            return OperationResult::from_args(self.execution_time, args);
        }

        panic!("DNCi overflow {m}");
    }

    fn get_name(&self) -> String {
        String::from("DECi")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inca() {
        let mut r = Registers::new();
        let mut m = Memory::new();

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = INCA::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 0), 48));
        operation.execute(args);
        assert_eq!(r.get_a(), Word::new(2_000));
        assert_eq!(r.is_overflow(), false);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = INCA::new(Word::new_instruction(-500, 0, WordAccess::new(0, 0), 48));
        operation.execute(args);
        assert_eq!(r.get_a(), Word::new(1_500));
        assert_eq!(r.is_overflow(), false);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = INCA::new(Word::new_instruction(-1_500, 0, WordAccess::new(0, 0), 48));
        operation.execute(args);
        assert_eq!(r.get_a(), Word::new(0));
        assert_eq!(r.get_a().get_sign(), 0);
        assert_eq!(r.is_overflow(), false);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = INCA::new(Word::new_instruction(-1_500, 0, WordAccess::new(0, 0), 48));
        operation.execute(args);
        assert_eq!(r.get_a(), Word::new_from_signed(-1_500));
        assert_eq!(r.get_a().get_sign(), -1);
        assert_eq!(r.is_overflow(), false);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = INCA::new(Word::new_instruction(1_500, 0, WordAccess::new(0, 0), 48));
        operation.execute(args);
        assert_eq!(r.get_a().get_signed_value(), 0);
        assert_eq!(r.get_a().get_sign(), -1);
        assert_eq!(r.is_overflow(), false);

        r.set_a(Word::new(1));
        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = INCA::new(Word::new_instruction(
            MAX_2_BYTES,
            0,
            WordAccess::new(0, 0),
            48,
        ));
        operation.execute(args);
        assert_eq!(r.get_a().get_signed_value(), 0);
        assert_eq!(r.get_a().get_sign(), 0);
        assert_eq!(r.is_overflow(), true);

        r.set_a(Word::new_from_signed(-2));
        r.set_overflow(false);
        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = INCA::new(Word::new_instruction(
            -MAX_2_BYTES,
            0,
            WordAccess::new(0, 0),
            48,
        ));
        operation.execute(args);
        assert_eq!(r.get_a().get_signed_value(), 0);
        assert_eq!(r.get_a().get_sign(), 0);
        assert_eq!(r.is_overflow(), true);
    }

    #[test]
    fn incx() {
        let mut r = Registers::new();
        let mut m = Memory::new();

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = INCX::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 0), 48));
        operation.execute(args);
        assert_eq!(r.get_x(), Word::new(2_000));
        assert_eq!(r.is_overflow(), false);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = INCX::new(Word::new_instruction(-500, 0, WordAccess::new(0, 0), 48));
        operation.execute(args);
        assert_eq!(r.get_x(), Word::new(1_500));
        assert_eq!(r.is_overflow(), false);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = INCX::new(Word::new_instruction(-1_500, 0, WordAccess::new(0, 0), 48));
        operation.execute(args);
        assert_eq!(r.get_x(), Word::new(0));
        assert_eq!(r.get_x().get_sign(), 0);
        assert_eq!(r.is_overflow(), false);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = INCX::new(Word::new_instruction(-1_500, 0, WordAccess::new(0, 0), 48));
        operation.execute(args);
        assert_eq!(r.get_x(), Word::new_from_signed(-1_500));
        assert_eq!(r.get_x().get_sign(), -1);
        assert_eq!(r.is_overflow(), false);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = INCX::new(Word::new_instruction(1_500, 0, WordAccess::new(0, 0), 48));
        operation.execute(args);
        assert_eq!(r.get_x().get_signed_value(), 0);
        assert_eq!(r.get_x().get_sign(), -1);
        assert_eq!(r.is_overflow(), false);

        r.set_x(Word::new(1));
        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = INCX::new(Word::new_instruction(
            MAX_2_BYTES,
            0,
            WordAccess::new(0, 0),
            48,
        ));
        operation.execute(args);
        assert_eq!(r.get_x().get_signed_value(), 0);
        assert_eq!(r.get_x().get_sign(), 0);
        assert_eq!(r.is_overflow(), true);

        r.set_x(Word::new_from_signed(-2));
        r.set_overflow(false);
        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = INCX::new(Word::new_instruction(
            -MAX_2_BYTES,
            0,
            WordAccess::new(0, 0),
            48,
        ));
        operation.execute(args);
        assert_eq!(r.get_x().get_signed_value(), 0);
        assert_eq!(r.get_x().get_sign(), 0);
        assert_eq!(r.is_overflow(), true);
    }

    #[test]
    fn deca() {
        let mut r = Registers::new();
        let mut m = Memory::new();

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = DECA::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 0), 48));
        operation.execute(args);
        assert_eq!(r.get_a(), Word::new_from_signed(-2_000));
        assert_eq!(r.is_overflow(), false);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = DECA::new(Word::new_instruction(-500, 0, WordAccess::new(0, 0), 48));
        operation.execute(args);
        assert_eq!(r.get_a(), Word::new_from_signed(-1_500));
        assert_eq!(r.is_overflow(), false);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = DECA::new(Word::new_instruction(-1_500, 0, WordAccess::new(0, 0), 48));
        operation.execute(args);
        assert_eq!(r.get_a().get_signed_value(), 0);
        assert_eq!(r.get_a().get_sign(), -1);
        assert_eq!(r.is_overflow(), false);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = DECA::new(Word::new_instruction(-1_500, 0, WordAccess::new(0, 0), 48));
        operation.execute(args);
        assert_eq!(r.get_a(), Word::new_from_signed(1_500));
        assert_eq!(r.get_a().get_sign(), 0);
        assert_eq!(r.is_overflow(), false);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = DECA::new(Word::new_instruction(1_500, 0, WordAccess::new(0, 0), 48));
        operation.execute(args);
        assert_eq!(r.get_a().get_signed_value(), 0);
        assert_eq!(r.get_a().get_sign(), 0);
        assert_eq!(r.is_overflow(), false);

        r.set_a(Word::new_from_signed(-2));
        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = DECA::new(Word::new_instruction(
            MAX_2_BYTES,
            0,
            WordAccess::new(0, 0),
            48,
        ));
        operation.execute(args);
        assert_eq!(r.get_a().get_signed_value(), 0);
        assert_eq!(r.get_a().get_sign(), 0);
        assert_eq!(r.is_overflow(), true);

        r.set_a(Word::new_from_signed(1));
        r.set_overflow(false);
        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = DECA::new(Word::new_instruction(
            -MAX_2_BYTES,
            0,
            WordAccess::new(0, 0),
            48,
        ));
        operation.execute(args);
        assert_eq!(r.get_a().get_signed_value(), 0);
        assert_eq!(r.get_a().get_sign(), 0);
        assert_eq!(r.is_overflow(), true);
    }

    #[test]
    fn decx() {
        let mut r = Registers::new();
        let mut m = Memory::new();

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = DECX::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 0), 48));
        operation.execute(args);
        assert_eq!(r.get_x(), Word::new_from_signed(-2_000));
        assert_eq!(r.is_overflow(), false);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = DECX::new(Word::new_instruction(-500, 0, WordAccess::new(0, 0), 48));
        operation.execute(args);
        assert_eq!(r.get_x(), Word::new_from_signed(-1_500));
        assert_eq!(r.is_overflow(), false);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = DECX::new(Word::new_instruction(-1_500, 0, WordAccess::new(0, 0), 48));
        operation.execute(args);
        assert_eq!(r.get_x().get_signed_value(), 0);
        assert_eq!(r.get_x().get_sign(), -1);
        assert_eq!(r.is_overflow(), false);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = DECX::new(Word::new_instruction(-1_500, 0, WordAccess::new(0, 0), 48));
        operation.execute(args);
        assert_eq!(r.get_x(), Word::new_from_signed(1_500));
        assert_eq!(r.get_x().get_sign(), 0);
        assert_eq!(r.is_overflow(), false);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = DECX::new(Word::new_instruction(1_500, 0, WordAccess::new(0, 0), 48));
        operation.execute(args);
        assert_eq!(r.get_x().get_signed_value(), 0);
        assert_eq!(r.get_x().get_sign(), 0);
        assert_eq!(r.is_overflow(), false);

        r.set_x(Word::new_from_signed(-2));
        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = DECX::new(Word::new_instruction(
            MAX_2_BYTES,
            0,
            WordAccess::new(0, 0),
            48,
        ));
        operation.execute(args);
        assert_eq!(r.get_x().get_signed_value(), 0);
        assert_eq!(r.get_x().get_sign(), 0);
        assert_eq!(r.is_overflow(), true);

        r.set_x(Word::new_from_signed(1));
        r.set_overflow(false);
        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = DECX::new(Word::new_instruction(
            -MAX_2_BYTES,
            0,
            WordAccess::new(0, 0),
            48,
        ));
        operation.execute(args);
        assert_eq!(r.get_x().get_signed_value(), 0);
        assert_eq!(r.get_x().get_sign(), 0);
        assert_eq!(r.is_overflow(), true);
    }

    #[test]
    fn inci() {
        let mut r = Registers::new();
        let mut m = Memory::new();

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = INCi::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 0), 50));
        operation.execute(args);
        assert_eq!(r.get_i(2), ShortWord::new(2_000));
        assert_eq!(r.is_overflow(), false);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = INCi::new(Word::new_instruction(-2_000, 0, WordAccess::new(0, 0), 50));
        operation.execute(args);
        assert_eq!(r.get_i(2), ShortWord::new_from_signed(0));
        assert_eq!(r.is_overflow(), false);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = INCi::new(Word::new_instruction(-2_000, 0, WordAccess::new(0, 0), 50));
        operation.execute(args);
        assert_eq!(r.get_i(2), ShortWord::new_from_signed(-2_000));
        assert_eq!(r.is_overflow(), false);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = INCi::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 0), 50));
        operation.execute(args);
        assert_eq!(r.get_i(2).get_signed_value(), 0);
        assert_eq!(r.get_i(2).get_sign(), -1);
        assert_eq!(r.is_overflow(), false);
    }

    #[test]
    fn deci() {
        let mut r = Registers::new();
        let mut m = Memory::new();

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = DECi::new(Word::new_instruction(-2_000, 0, WordAccess::new(0, 0), 50));
        operation.execute(args);
        assert_eq!(r.get_i(2), ShortWord::new(2_000));
        assert_eq!(r.is_overflow(), false);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = DECi::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 0), 50));
        operation.execute(args);
        assert_eq!(r.get_i(2), ShortWord::new_from_signed(0));
        assert_eq!(r.is_overflow(), false);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = DECi::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 0), 50));
        operation.execute(args);
        assert_eq!(r.get_i(2), ShortWord::new_from_signed(-2_000));
        assert_eq!(r.is_overflow(), false);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = DECi::new(Word::new_instruction(-2_000, 0, WordAccess::new(0, 0), 50));
        operation.execute(args);
        assert_eq!(r.get_i(2).get_signed_value(), 0);
        assert_eq!(r.get_i(2).get_sign(), -1);
        assert_eq!(r.is_overflow(), false);
    }
}
