use crate::memory::short_word::ShortWord;
use crate::memory::word::Word;
use crate::memory::Instruction;
use crate::memory::Memory;
use crate::operations::get_memory_cell;
use crate::operations::*;
use crate::registers::Comparison;
use crate::registers::RegisterType;
use crate::registers::Registers;

fn compare(instruction: impl Instruction, r_type: RegisterType, mem: &Memory, reg: &mut Registers) {
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

pub struct CMPA {
    code: u32,
    execution_time: u32,
}

impl CMPA {
    pub fn new() -> CMPA {
        CMPA {
            code: 56,
            execution_time: 2,
        }
    }
}

impl Operation for CMPA {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        compare(args.instruction, RegisterType::A, args.mem, args.reg);

        OperationResult::from_args(self.execution_time, args)
    }
}

pub struct CMPX {
    code: u32,
    execution_time: u32,
}

impl CMPX {
    pub fn new() -> CMPX {
        CMPX {
            code: 63,
            execution_time: 2,
        }
    }
}

impl Operation for CMPX {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        compare(args.instruction, RegisterType::X, args.mem, args.reg);

        OperationResult::from_args(self.execution_time, args)
    }
}

pub struct CMPi {
    code: u32,
    execution_time: u32,
}

impl CMPi {
    pub fn new() -> CMPi {
        CMPi {
            code: 56,
            execution_time: 2,
        }
    }
}

impl Operation for CMPi {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let f = args.instruction.get_f();
        if f.spec == 0 {
            args.reg.set_comparison(Comparison::EQUAL);
            return OperationResult::from_args(self.execution_time, args);
        }

        let mem_cell = get_memory_cell(args.instruction, args.mem, args.reg);
        let mem_value = Word::new(mem_cell.get_by_access(f)).get_signed_value();

        let i = (args.instruction.get_c() - self.code as u8) as usize;
        let reg_cell = args.reg.get_i(i);
        let reg_value = ShortWord::new(reg_cell.get_by_access(f)).get_signed_value();

        if reg_value > mem_value {
            args.reg.set_comparison(Comparison::GREATHER);
        } else if reg_value < mem_value {
            args.reg.set_comparison(Comparison::LESS);
        } else {
            args.reg.set_comparison(Comparison::EQUAL);
        }

        OperationResult::from_args(self.execution_time, args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmpa() {
        let operation = CMPA::new();

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
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 1, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::EQUAL);

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 2, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::LESS);

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::GREATHER);

        r.set_a(Word::new_from_signed(-1));
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 3, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::EQUAL);

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::LESS);

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 4, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::GREATHER);
    }

    #[test]
    fn cmpx() {
        let operation = CMPX::new();
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
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 1, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::EQUAL);

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 2, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::LESS);

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::GREATHER);

        r.set_x(Word::new_from_signed(-1));
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 3, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::EQUAL);

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::LESS);

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 4, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::GREATHER);
    }

    #[test]
    fn cmpi() {
        let operation = CMPi::new();
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

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_001, 0, WordAccess::new(0, 5), 57),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::EQUAL);

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_001, 0, WordAccess::new(0, 5), 58),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::GREATHER);

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_001, 0, WordAccess::new(0, 5), 59),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::LESS);

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_004, 0, WordAccess::new(0, 5), 59),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::GREATHER);

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_003, 0, WordAccess::new(0, 5), 60),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::LESS);

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_004, 0, WordAccess::new(0, 5), 60),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::EQUAL);

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(0, 0), 57),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::EQUAL);
    }
}
