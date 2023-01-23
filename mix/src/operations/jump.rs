use crate::memory::short_word::ShortWord;
use crate::memory::word::Word;
use crate::memory::word_access::WordAccess;
use crate::memory::Instruction;
use crate::memory::Memory;
use crate::memory::Sign;
use crate::memory::NEGATIVE;
use crate::memory::POSITIVE;
use crate::operations::get_memory_cell;
use crate::operations::*;
use crate::registers::Comparison;
use crate::registers::RegisterType;
use crate::registers::Registers;

pub struct JMP {
    code: u32,
    execution_time: u32,
    f: u32,
}
impl JMP {
    pub fn new() -> JMP {
        JMP {
            code: 39,
            execution_time: 1,
            f: 0,
        }
    }
}
impl Operation for JMP {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        args.reg.set_j(ShortWord::new(args.addr + 1));

        let next_addr = args.instruction.get_address() as u32;
        OperationResult::new(self.execution_time, next_addr)
    }
}

pub struct JSJ {
    code: u32,
    execution_time: u32,
    f: u32,
}
impl JSJ {
    pub fn new() -> JSJ {
        JSJ {
            code: 39,
            execution_time: 1,
            f: 1,
        }
    }
}
impl Operation for JSJ {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let next_addr = args.instruction.get_address() as u32;
        OperationResult::new(self.execution_time, next_addr)
    }
}

pub struct JOV {
    code: u32,
    execution_time: u32,
    f: u32,
}
impl JOV {
    pub fn new() -> JOV {
        JOV {
            code: 39,
            execution_time: 1,
            f: 2,
        }
    }
}
impl Operation for JOV {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        return if args.reg.is_overflow() {
            args.reg.set_overflow(false);

            args.reg.set_j(ShortWord::new(args.addr + 1));
            let next_addr = args.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            OperationResult::from_args(self.execution_time, args)
        };
    }
}

pub struct JNOV {
    code: u32,
    execution_time: u32,
    f: u32,
}
impl JNOV {
    pub fn new() -> JNOV {
        JNOV {
            code: 39,
            execution_time: 1,
            f: 3,
        }
    }
}
impl Operation for JNOV {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        return if !args.reg.is_overflow() {
            args.reg.set_j(ShortWord::new(args.addr + 1));

            let next_addr = args.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            args.reg.set_overflow(false);
            OperationResult::from_args(self.execution_time, args)
        };
    }
}

pub struct JL {
    code: u32,
    execution_time: u32,
    f: u32,
}
impl JL {
    pub fn new() -> JL {
        JL {
            code: 39,
            execution_time: 1,
            f: 4,
        }
    }
}
impl Operation for JL {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        return if args.reg.get_comparison() == Comparison::LESS {
            args.reg.set_j(ShortWord::new(args.addr + 1));

            let next_addr = args.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            OperationResult::from_args(self.execution_time, args)
        };
    }
}

pub struct JE {
    code: u32,
    execution_time: u32,
    f: u32,
}
impl JE {
    pub fn new() -> JE {
        JE {
            code: 39,
            execution_time: 1,
            f: 5,
        }
    }
}
impl Operation for JE {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        return if args.reg.get_comparison() == Comparison::EQUAL {
            args.reg.set_j(ShortWord::new(args.addr + 1));

            let next_addr = args.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            OperationResult::from_args(self.execution_time, args)
        };
    }
}

pub struct JG {
    code: u32,
    execution_time: u32,
    f: u32,
}
impl JG {
    pub fn new() -> JG {
        JG {
            code: 39,
            execution_time: 1,
            f: 6,
        }
    }
}
impl Operation for JG {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        return if args.reg.get_comparison() == Comparison::GREATHER {
            args.reg.set_j(ShortWord::new(args.addr + 1));

            let next_addr = args.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            OperationResult::from_args(self.execution_time, args)
        };
    }
}

pub struct JGE {
    code: u32,
    execution_time: u32,
    f: u32,
}
impl JGE {
    pub fn new() -> JGE {
        JGE {
            code: 39,
            execution_time: 1,
            f: 7,
        }
    }
}
impl Operation for JGE {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        return if args.reg.get_comparison() == Comparison::GREATHER
            || args.reg.get_comparison() == Comparison::EQUAL
        {
            args.reg.set_j(ShortWord::new(args.addr + 1));

            let next_addr = args.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            OperationResult::from_args(self.execution_time, args)
        };
    }
}

pub struct JNE {
    code: u32,
    execution_time: u32,
    f: u32,
}
impl JNE {
    pub fn new() -> JNE {
        JNE {
            code: 39,
            execution_time: 1,
            f: 8,
        }
    }
}
impl Operation for JNE {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        return if args.reg.get_comparison() == Comparison::GREATHER
            || args.reg.get_comparison() == Comparison::LESS
        {
            args.reg.set_j(ShortWord::new(args.addr + 1));

            let next_addr = args.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            OperationResult::from_args(self.execution_time, args)
        };
    }
}

pub struct JLE {
    code: u32,
    execution_time: u32,
    f: u32,
}
impl JLE {
    pub fn new() -> JLE {
        JLE {
            code: 39,
            execution_time: 1,
            f: 9,
        }
    }
}
impl Operation for JLE {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        return if args.reg.get_comparison() == Comparison::EQUAL
            || args.reg.get_comparison() == Comparison::LESS
        {
            args.reg.set_j(ShortWord::new(args.addr + 1));

            let next_addr = args.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            OperationResult::from_args(self.execution_time, args)
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jmp() {
        let mut m = Memory::new();
        let mut r = Registers::new();

        let operation = JMP::new();
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 1, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        let result = operation.execute(args);
        assert_eq!(2_000, result.next_addr_instruction);
        assert_eq!(2, r.get_j().get());

        let operation = JSJ::new();
        let args = OperationArgs::new(
            3,
            Word::new_instruction(3_000, 1, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        let result = operation.execute(args);
        assert_eq!(3_000, result.next_addr_instruction);
        assert_eq!(2, r.get_j().get());

        let operation = JOV::new();
        let args = OperationArgs::new(
            3,
            Word::new_instruction(3_000, 1, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        let result = operation.execute(args);
        assert_eq!(4, result.next_addr_instruction);
        assert_eq!(2, r.get_j().get());

        r.set_overflow(true);
        let operation = JOV::new();
        let args = OperationArgs::new(
            3,
            Word::new_instruction(3_000, 1, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        let result = operation.execute(args);
        assert_eq!(3_000, result.next_addr_instruction);
        assert_eq!(4, r.get_j().get());

        r.set_overflow(true);
        let operation = JNOV::new();
        let args = OperationArgs::new(
            4,
            Word::new_instruction(3_000, 1, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        let result = operation.execute(args);
        assert_eq!(5, result.next_addr_instruction);
        assert_eq!(4, r.get_j().get());

        r.set_overflow(false);
        let operation = JNOV::new();
        let args = OperationArgs::new(
            4,
            Word::new_instruction(3_000, 1, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        let result = operation.execute(args);
        assert_eq!(3_000, result.next_addr_instruction);
        assert_eq!(5, r.get_j().get());
    }

    #[test]
    fn jl() {
        let mut m = Memory::new();
        let mut r = Registers::new();

        let operation = JL::new();
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 1, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        let result = operation.execute(args);
        assert_eq!(2, result.next_addr_instruction);
        assert_eq!(0, r.get_j().get());

        r.set_comparison(Comparison::LESS);
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 1, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        let result = operation.execute(args);
        assert_eq!(2_000, result.next_addr_instruction);
        assert_eq!(2, r.get_j().get());
    }

    #[test]
    fn je() {
        let mut m = Memory::new();
        let mut r = Registers::new();
        let operation = JE::new();

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 1, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        let result = operation.execute(args);
        assert_eq!(2, result.next_addr_instruction);
        assert_eq!(0, r.get_j().get());

        r.set_comparison(Comparison::EQUAL);
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 1, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        let result = operation.execute(args);
        assert_eq!(2_000, result.next_addr_instruction);
        assert_eq!(2, r.get_j().get());
    }

    #[test]
    fn jg() {
        let mut m = Memory::new();
        let mut r = Registers::new();
        let operation = JG::new();

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 1, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        let result = operation.execute(args);
        assert_eq!(2, result.next_addr_instruction);
        assert_eq!(0, r.get_j().get());

        r.set_comparison(Comparison::GREATHER);
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 1, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        let result = operation.execute(args);
        assert_eq!(2_000, result.next_addr_instruction);
        assert_eq!(2, r.get_j().get());
    }

    #[test]
    fn jge() {
        let mut m = Memory::new();
        let mut r = Registers::new();
        let operation = JGE::new();

        r.set_comparison(Comparison::EQUAL);
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 1, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        let result = operation.execute(args);
        assert_eq!(2_000, result.next_addr_instruction);
        assert_eq!(2, r.get_j().get());

        r.set_comparison(Comparison::GREATHER);
        let args = OperationArgs::new(
            2,
            Word::new_instruction(2_001, 1, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        let result = operation.execute(args);
        assert_eq!(2_001, result.next_addr_instruction);
        assert_eq!(3, r.get_j().get());
    }

    #[test]
    fn jne() {
        let mut m = Memory::new();
        let mut r = Registers::new();
        let operation = JNE::new();

        r.set_comparison(Comparison::LESS);
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 1, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        let result = operation.execute(args);
        assert_eq!(2_000, result.next_addr_instruction);
        assert_eq!(2, r.get_j().get());

        r.set_comparison(Comparison::GREATHER);
        let args = OperationArgs::new(
            2,
            Word::new_instruction(2_001, 1, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        let result = operation.execute(args);
        assert_eq!(2_001, result.next_addr_instruction);
        assert_eq!(3, r.get_j().get());
    }

    #[test]
    fn jle() {
        let mut m = Memory::new();
        let mut r = Registers::new();
        let operation = JLE::new();

        r.set_comparison(Comparison::LESS);
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 1, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        let result = operation.execute(args);
        assert_eq!(2_000, result.next_addr_instruction);
        assert_eq!(2, r.get_j().get());

        r.set_comparison(Comparison::EQUAL);
        let args = OperationArgs::new(
            2,
            Word::new_instruction(2_001, 1, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        let result = operation.execute(args);
        assert_eq!(2_001, result.next_addr_instruction);
        assert_eq!(3, r.get_j().get());
    }
}
