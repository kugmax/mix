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

impl Operation for CMPA {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        compare(self.instruction, RegisterType::A, args.mem, args.reg);

        OperationResult::from_args(self.execution_time, args)
    }
    fn get_name(&self) -> String {
        String::from("CMPA")
    }
}

pub struct CMPX {
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

impl Operation for CMPX {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        compare(self.instruction, RegisterType::X, args.mem, args.reg);

        OperationResult::from_args(self.execution_time, args)
    }
    fn get_name(&self) -> String {
        String::from("CMPX")
    }
}

pub struct CMPi {
    code: u32,
    execution_time: u32,
    instruction: Word,
}

impl CMPi {
    pub fn new(instruction: Word) -> CMPi {
        CMPi {
            code: 56,
            execution_time: 2,
            instruction: instruction,
        }
    }
}

impl Operation for CMPi {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let f = self.instruction.get_f();
        if f.spec == 0 {
            args.reg.set_comparison(Comparison::EQUAL);
            return OperationResult::from_args(self.execution_time, args);
        }

        let mem_cell = get_memory_cell(self.instruction, args.mem, args.reg);
        let mem_value = Word::new(mem_cell.get_by_access(f)).get_signed_value();

        let i = (self.instruction.get_c() - self.code as u8) as usize;
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
    fn get_name(&self) -> String {
        let i = (self.instruction.get_c() - self.code as u8);
        String::from("CMP") + &i.to_string() 
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
        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = CMPA::new(Word::new_instruction(2_000, 1, WordAccess::new(0, 5), 56));
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::EQUAL);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = CMPA::new(Word::new_instruction(2_000, 2, WordAccess::new(0, 5), 56));
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::LESS);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = CMPA::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 56));
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::GREATHER);

        r.set_a(Word::new_from_signed(-1));
        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = CMPA::new(Word::new_instruction(2_000, 3, WordAccess::new(0, 5), 56));
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::EQUAL);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = CMPA::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 56));
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::LESS);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = CMPA::new(Word::new_instruction(2_000, 4, WordAccess::new(0, 5), 56));
        operation.execute(args);
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
        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = CMPX::new(Word::new_instruction(2_000, 1, WordAccess::new(0, 5), 56));
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::EQUAL);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = CMPX::new(Word::new_instruction(2_000, 2, WordAccess::new(0, 5), 56));
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::LESS);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = CMPX::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 56));
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::GREATHER);

        r.set_x(Word::new_from_signed(-1));
        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = CMPX::new(Word::new_instruction(2_000, 3, WordAccess::new(0, 5), 56));
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::EQUAL);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = CMPX::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 56));
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::LESS);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = CMPX::new(Word::new_instruction(2_000, 4, WordAccess::new(0, 5), 56));
        operation.execute(args);
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

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = CMPi::new(Word::new_instruction(2_001, 0, WordAccess::new(0, 5), 57));
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::EQUAL);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = CMPi::new(Word::new_instruction(2_001, 0, WordAccess::new(0, 5), 58));
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::GREATHER);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = CMPi::new(Word::new_instruction(2_001, 0, WordAccess::new(0, 5), 59));
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::LESS);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = CMPi::new(Word::new_instruction(2_004, 0, WordAccess::new(0, 5), 59));
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::GREATHER);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = CMPi::new(Word::new_instruction(2_003, 0, WordAccess::new(0, 5), 60));
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::LESS);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = CMPi::new(Word::new_instruction(2_004, 0, WordAccess::new(0, 5), 60));
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::EQUAL);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = CMPi::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 0), 57));
        operation.execute(args);
        assert_eq!(r.get_comparison(), Comparison::EQUAL);
    }
}
