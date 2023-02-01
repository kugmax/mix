use crate::memory::short_word::ShortWord;
use crate::memory::Instruction;
use crate::operations::*;
use crate::registers::Comparison;

pub struct JMP {
    code: u32,
    execution_time: u32,
    f: u32,
    instruction: Word,
}
impl JMP {
    pub fn new(instruction: Word) -> JMP {
        JMP {
            code: 39,
            execution_time: 1,
            f: 0,
            instruction: instruction,
        }
    }
}
impl Operation for JMP {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        args.reg.set_j(ShortWord::new(args.addr + 1));

        let next_addr = self.instruction.get_address() as u32; //TODO: should be indexed??
        OperationResult::new(self.execution_time, next_addr)
    }
    fn get_name(&self) -> String {
        String::from("JMP")
    }
}

pub struct JSJ {
    code: u32,
    execution_time: u32,
    f: u32,
    instruction: Word,
}
impl JSJ {
    pub fn new(instruction: Word) -> JSJ {
        JSJ {
            code: 39,
            execution_time: 1,
            f: 1,
            instruction: instruction,
        }
    }
}
impl Operation for JSJ {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let next_addr = self.instruction.get_address() as u32;
        OperationResult::new(self.execution_time, next_addr)
    }
    fn get_name(&self) -> String {
        String::from("JSJ")
    }
}

pub struct JOV {
    code: u32,
    execution_time: u32,
    f: u32,
    instruction: Word,
}
impl JOV {
    pub fn new(instruction: Word) -> JOV {
        JOV {
            code: 39,
            execution_time: 1,
            f: 2,
            instruction: instruction,
        }
    }
}
impl Operation for JOV {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        return if args.reg.is_overflow() {
            args.reg.set_overflow(false);

            args.reg.set_j(ShortWord::new(args.addr + 1));
            let next_addr = self.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            OperationResult::from_args(self.execution_time, args)
        };
    }
    fn get_name(&self) -> String {
        String::from("JOV")
    }
}

pub struct JNOV {
    code: u32,
    execution_time: u32,
    f: u32,
    instruction: Word,
}
impl JNOV {
    pub fn new(instruction: Word) -> JNOV {
        JNOV {
            code: 39,
            execution_time: 1,
            f: 3,
            instruction: instruction,
        }
    }
}
impl Operation for JNOV {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        return if !args.reg.is_overflow() {
            args.reg.set_j(ShortWord::new(args.addr + 1));

            let next_addr = self.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            args.reg.set_overflow(false);
            OperationResult::from_args(self.execution_time, args)
        };
    }
    fn get_name(&self) -> String {
        String::from("JNOV")
    }
}

pub struct JL {
    code: u32,
    execution_time: u32,
    f: u32,
    instruction: Word,
}
impl JL {
    pub fn new(instruction: Word) -> JL {
        JL {
            code: 39,
            execution_time: 1,
            f: 4,
            instruction: instruction,
        }
    }
}
impl Operation for JL {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        return if args.reg.get_comparison() == Comparison::LESS {
            args.reg.set_j(ShortWord::new(args.addr + 1));

            let next_addr = self.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            OperationResult::from_args(self.execution_time, args)
        };
    }
    fn get_name(&self) -> String {
        String::from("JL")
    }
}

pub struct JE {
    code: u32,
    execution_time: u32,
    f: u32,
    instruction: Word,
}
impl JE {
    pub fn new(instruction: Word) -> JE {
        JE {
            code: 39,
            execution_time: 1,
            f: 5,
            instruction: instruction,
        }
    }
}
impl Operation for JE {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        return if args.reg.get_comparison() == Comparison::EQUAL {
            args.reg.set_j(ShortWord::new(args.addr + 1));

            let next_addr = self.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            OperationResult::from_args(self.execution_time, args)
        };
    }
    fn get_name(&self) -> String {
        String::from("JE")
    }
}

pub struct JG {
    code: u32,
    execution_time: u32,
    f: u32,
    instruction: Word,
}
impl JG {
    pub fn new(instruction: Word) -> JG {
        JG {
            code: 39,
            execution_time: 1,
            f: 6,
            instruction: instruction,
        }
    }
}
impl Operation for JG {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        return if args.reg.get_comparison() == Comparison::GREATHER {
            args.reg.set_j(ShortWord::new(args.addr + 1));

            let next_addr = self.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            OperationResult::from_args(self.execution_time, args)
        };
    }
    fn get_name(&self) -> String {
        String::from("JG")
    }
}

pub struct JGE {
    code: u32,
    execution_time: u32,
    f: u32,
    instruction: Word,
}
impl JGE {
    pub fn new(instruction: Word) -> JGE {
        JGE {
            code: 39,
            execution_time: 1,
            f: 7,
            instruction: instruction,
        }
    }
}
impl Operation for JGE {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        return if args.reg.get_comparison() == Comparison::GREATHER
            || args.reg.get_comparison() == Comparison::EQUAL
        {
            args.reg.set_j(ShortWord::new(args.addr + 1));

            let next_addr = self.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            OperationResult::from_args(self.execution_time, args)
        };
    }
    fn get_name(&self) -> String {
        String::from("JGE")
    }
}

pub struct JNE {
    code: u32,
    execution_time: u32,
    f: u32,
    instruction: Word,
}
impl JNE {
    pub fn new(instruction: Word) -> JNE {
        JNE {
            code: 39,
            execution_time: 1,
            f: 8,
            instruction: instruction,
        }
    }
}
impl Operation for JNE {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        return if args.reg.get_comparison() == Comparison::GREATHER
            || args.reg.get_comparison() == Comparison::LESS
        {
            args.reg.set_j(ShortWord::new(args.addr + 1));

            let next_addr = self.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            OperationResult::from_args(self.execution_time, args)
        };
    }
    fn get_name(&self) -> String {
        String::from("JNE")
    }
}

pub struct JLE {
    code: u32,
    execution_time: u32,
    f: u32,
    instruction: Word,
}
impl JLE {
    pub fn new(instruction: Word) -> JLE {
        JLE {
            code: 39,
            execution_time: 1,
            f: 9,
            instruction: instruction,
        }
    }
}
impl Operation for JLE {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        return if args.reg.get_comparison() == Comparison::EQUAL
            || args.reg.get_comparison() == Comparison::LESS
        {
            args.reg.set_j(ShortWord::new(args.addr + 1));

            let next_addr = self.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            OperationResult::from_args(self.execution_time, args)
        };
    }
    fn get_name(&self) -> String {
        String::from("JLE")
    }
}

pub struct JAN {
    code: u32,
    execution_time: u32,
    f: u32,
    instruction: Word,
}
impl JAN {
    pub fn new(instruction: Word) -> JAN {
        JAN {
            code: 40,
            execution_time: 1,
            f: 0,
            instruction: instruction,
        }
    }
}
impl Operation for JAN {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        return if args.reg.get_a().get_signed_value() < 0 {
            args.reg.set_j(ShortWord::new(args.addr + 1));

            let next_addr = self.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            OperationResult::from_args(self.execution_time, args)
        };
    }
    fn get_name(&self) -> String {
        String::from("JAN")
    }
}

pub struct JAZ {
    code: u32,
    execution_time: u32,
    f: u32,
    instruction: Word,
}
impl JAZ {
    pub fn new(instruction: Word) -> JAZ {
        JAZ {
            code: 40,
            execution_time: 1,
            f: 1,
            instruction: instruction,
        }
    }
}
impl Operation for JAZ {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        return if args.reg.get_a().get_signed_value() == 0 {
            args.reg.set_j(ShortWord::new(args.addr + 1));

            let next_addr = self.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            OperationResult::from_args(self.execution_time, args)
        };
    }
    fn get_name(&self) -> String {
        String::from("JAZ")
    }
}

pub struct JAP {
    code: u32,
    execution_time: u32,
    f: u32,
    instruction: Word,
}
impl JAP {
    pub fn new(instruction: Word) -> JAP {
        JAP {
            code: 40,
            execution_time: 1,
            f: 2,
            instruction: instruction,
        }
    }
}
impl Operation for JAP {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        return if args.reg.get_a().get_signed_value() > 0 {
            args.reg.set_j(ShortWord::new(args.addr + 1));

            let next_addr = self.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            OperationResult::from_args(self.execution_time, args)
        };
    }
    fn get_name(&self) -> String {
        String::from("JAP")
    }
}

pub struct JANN {
    code: u32,
    execution_time: u32,
    f: u32,
    instruction: Word,
}
impl JANN {
    pub fn new(instruction: Word) -> JANN {
        JANN {
            code: 40,
            execution_time: 1,
            f: 3,
            instruction: instruction,
        }
    }
}
impl Operation for JANN {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        return if args.reg.get_a().get_signed_value() >= 0 {
            args.reg.set_j(ShortWord::new(args.addr + 1));

            let next_addr = self.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            OperationResult::from_args(self.execution_time, args)
        };
    }
    fn get_name(&self) -> String {
        String::from("JANN")
    }
}

pub struct JANZ {
    code: u32,
    execution_time: u32,
    f: u32,
    instruction: Word,
}
impl JANZ {
    pub fn new(instruction: Word) -> JANZ {
        JANZ {
            code: 40,
            execution_time: 1,
            f: 4,
            instruction: instruction,
        }
    }
}
impl Operation for JANZ {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        return if args.reg.get_a().get_signed_value() != 0 {
            args.reg.set_j(ShortWord::new(args.addr + 1));

            let next_addr = self.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            OperationResult::from_args(self.execution_time, args)
        };
    }
    fn get_name(&self) -> String {
        String::from("JANZ")
    }
}

pub struct JANP {
    code: u32,
    execution_time: u32,
    f: u32,
    instruction: Word,
}
impl JANP {
    pub fn new(instruction: Word) -> JANP {
        JANP {
            code: 40,
            execution_time: 1,
            f: 5,
            instruction: instruction,
        }
    }
}
impl Operation for JANP {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        return if args.reg.get_a().get_signed_value() <= 0 {
            args.reg.set_j(ShortWord::new(args.addr + 1));

            let next_addr = self.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            OperationResult::from_args(self.execution_time, args)
        };
    }
    fn get_name(&self) -> String {
        String::from("JANP")
    }
}

pub struct JXN {
    code: u32,
    execution_time: u32,
    f: u32,
    instruction: Word,
}
impl JXN {
    pub fn new(instruction: Word) -> JXN {
        JXN {
            code: 47,
            execution_time: 1,
            f: 0,
            instruction: instruction,
        }
    }
}
impl Operation for JXN {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        return if args.reg.get_x().get_signed_value() < 0 {
            args.reg.set_j(ShortWord::new(args.addr + 1));

            let next_addr = self.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            OperationResult::from_args(self.execution_time, args)
        };
    }
    fn get_name(&self) -> String {
        String::from("JXN")
    }
}

pub struct JXZ {
    code: u32,
    execution_time: u32,
    f: u32,
    instruction: Word,
}
impl JXZ {
    pub fn new(instruction: Word) -> JXZ {
        JXZ {
            code: 47,
            execution_time: 1,
            f: 1,
            instruction: instruction,
        }
    }
}
impl Operation for JXZ {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        return if args.reg.get_x().get_signed_value() == 0 {
            args.reg.set_j(ShortWord::new(args.addr + 1));

            let next_addr = self.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            OperationResult::from_args(self.execution_time, args)
        };
    }
    fn get_name(&self) -> String {
        String::from("JXZ")
    }
}

pub struct JXP {
    code: u32,
    execution_time: u32,
    f: u32,
    instruction: Word,
}
impl JXP {
    pub fn new(instruction: Word) -> JXP {
        JXP {
            code: 47,
            execution_time: 1,
            f: 2,
            instruction: instruction,
        }
    }
}
impl Operation for JXP {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        return if args.reg.get_x().get_signed_value() > 0 {
            args.reg.set_j(ShortWord::new(args.addr + 1));

            let next_addr = self.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            OperationResult::from_args(self.execution_time, args)
        };
    }
    fn get_name(&self) -> String {
        String::from("JXP")
    }
}

pub struct JXNN {
    code: u32,
    execution_time: u32,
    f: u32,
    instruction: Word,
}
impl JXNN {
    pub fn new(instruction: Word) -> JXNN {
        JXNN {
            code: 47,
            execution_time: 1,
            f: 3,
            instruction: instruction,
        }
    }
}
impl Operation for JXNN {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        return if args.reg.get_x().get_signed_value() >= 0 {
            args.reg.set_j(ShortWord::new(args.addr + 1));

            let next_addr = self.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            OperationResult::from_args(self.execution_time, args)
        };
    }
    fn get_name(&self) -> String {
        String::from("JXNN")
    }
}

pub struct JXNZ {
    code: u32,
    execution_time: u32,
    f: u32,
    instruction: Word,
}
impl JXNZ {
    pub fn new(instruction: Word) -> JXNZ {
        JXNZ {
            code: 47,
            execution_time: 1,
            f: 4,
            instruction: instruction,
        }
    }
}
impl Operation for JXNZ {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        return if args.reg.get_x().get_signed_value() != 0 {
            args.reg.set_j(ShortWord::new(args.addr + 1));

            let next_addr = self.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            OperationResult::from_args(self.execution_time, args)
        };
    }
    fn get_name(&self) -> String {
        String::from("JXNZ")
    }
}

pub struct JXNP {
    code: u32,
    execution_time: u32,
    f: u32,
    instruction: Word,
}
impl JXNP {
    pub fn new(instruction: Word) -> JXNP {
        JXNP {
            code: 47,
            execution_time: 1,
            f: 5,
            instruction: instruction,
        }
    }
}
impl Operation for JXNP {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        return if args.reg.get_x().get_signed_value() <= 0 {
            args.reg.set_j(ShortWord::new(args.addr + 1));

            let next_addr = self.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            OperationResult::from_args(self.execution_time, args)
        };
    }
    fn get_name(&self) -> String {
        String::from("JXNP")
    }
}

pub struct JiN {
    code: u32,
    execution_time: u32,
    f: u32,
    instruction: Word,
}
impl JiN {
    pub fn new(instruction: Word) -> JiN {
        JiN {
            code: 40,
            execution_time: 1,
            f: 0,
            instruction: instruction,
        }
    }
}
impl Operation for JiN {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let i = (self.instruction.get_c() - self.code as u8) as usize;
        return if args.reg.get_i(i).get_signed_value() < 0 {
            args.reg.set_j(ShortWord::new(args.addr + 1));

            let next_addr = self.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            OperationResult::from_args(self.execution_time, args)
        };
    }
    fn get_name(&self) -> String {
        let i = (self.instruction.get_c() - self.code as u8);
        String::from("J") + &i.to_string() + &String::from("N")
    }
}

pub struct JiZ {
    code: u32,
    execution_time: u32,
    f: u32,
    instruction: Word,
}
impl JiZ {
    pub fn new(instruction: Word) -> JiZ {
        JiZ {
            code: 40,
            execution_time: 1,
            f: 1,
            instruction: instruction,
        }
    }
}
impl Operation for JiZ {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let i = (self.instruction.get_c() - self.code as u8) as usize;
        return if args.reg.get_i(i).get_signed_value() == 0 {
            args.reg.set_j(ShortWord::new(args.addr + 1));

            let next_addr = self.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            OperationResult::from_args(self.execution_time, args)
        };
    }
    fn get_name(&self) -> String {
        let i = (self.instruction.get_c() - self.code as u8);
        String::from("J") + &i.to_string() + &String::from("Z")
    }
}

pub struct JiP {
    code: u32,
    execution_time: u32,
    f: u32,
    instruction: Word,
}
impl JiP {
    pub fn new(instruction: Word) -> JiP {
        JiP {
            code: 40,
            execution_time: 1,
            f: 2,
            instruction: instruction,
        }
    }
}
impl Operation for JiP {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let i = (self.instruction.get_c() - self.code as u8) as usize;
        return if args.reg.get_i(i).get_signed_value() > 0 {
            args.reg.set_j(ShortWord::new(args.addr + 1));

            let next_addr = self.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            OperationResult::from_args(self.execution_time, args)
        };
    }
    fn get_name(&self) -> String {
        let i = (self.instruction.get_c() - self.code as u8);
        String::from("J") + &i.to_string() + &String::from("P")
    }
}

pub struct JiNN {
    code: u32,
    execution_time: u32,
    f: u32,
    instruction: Word,
}
impl JiNN {
    pub fn new(instruction: Word) -> JiNN {
        JiNN {
            code: 40,
            execution_time: 1,
            f: 3,
            instruction: instruction,
        }
    }
}
impl Operation for JiNN {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let i = (self.instruction.get_c() - self.code as u8) as usize;
        return if args.reg.get_i(i).get_signed_value() >= 0 {
            args.reg.set_j(ShortWord::new(args.addr + 1));

            let next_addr = self.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            OperationResult::from_args(self.execution_time, args)
        };
    }
    fn get_name(&self) -> String {
        let i = (self.instruction.get_c() - self.code as u8);
        String::from("J") + &i.to_string() + &String::from("NN")
    }
}

pub struct JiNZ {
    code: u32,
    execution_time: u32,
    f: u32,
    instruction: Word,
}
impl JiNZ {
    pub fn new(instruction: Word) -> JiNZ {
        JiNZ {
            code: 40,
            execution_time: 1,
            f: 4,
            instruction: instruction,
        }
    }
}
impl Operation for JiNZ {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let i = (self.instruction.get_c() - self.code as u8) as usize;
        return if args.reg.get_i(i).get_signed_value() != 0 {
            args.reg.set_j(ShortWord::new(args.addr + 1));

            let next_addr = self.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            OperationResult::from_args(self.execution_time, args)
        };
    }
    fn get_name(&self) -> String {
        let i = (self.instruction.get_c() - self.code as u8);
        String::from("J") + &i.to_string() + &String::from("NZ")
    }
}

pub struct JiNP {
    code: u32,
    execution_time: u32,
    f: u32,
    instruction: Word,
}
impl JiNP {
    pub fn new(instruction: Word) -> JiNP {
        JiNP {
            code: 40,
            execution_time: 1,
            f: 5,
            instruction: instruction,
        }
    }
}
impl Operation for JiNP {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let i = (self.instruction.get_c() - self.code as u8) as usize;
        return if args.reg.get_i(i).get_signed_value() <= 0 {
            args.reg.set_j(ShortWord::new(args.addr + 1));

            let next_addr = self.instruction.get_address() as u32;
            OperationResult::new(self.execution_time, next_addr)
        } else {
            OperationResult::from_args(self.execution_time, args)
        };
    }
    fn get_name(&self) -> String {
        let i = (self.instruction.get_c() - self.code as u8);
        String::from("J") + &i.to_string() + &String::from("NP")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jmp() {
        let mut m = Memory::new();
        let mut r = Registers::new();

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = JMP::new(Word::new_instruction(2_000, 1, WordAccess::new(0, 5), 56));
        let result = operation.execute(args);
        assert_eq!(2_000, result.next_addr_instruction);
        assert_eq!(2, r.get_j().get());

        let args = OperationArgs::new(3, &mut m, &mut r);
        let operation = JSJ::new(Word::new_instruction(3_000, 1, WordAccess::new(0, 5), 56));
        let result = operation.execute(args);
        assert_eq!(3_000, result.next_addr_instruction);
        assert_eq!(2, r.get_j().get());

        let args = OperationArgs::new(3, &mut m, &mut r);
        let operation = JOV::new(Word::new_instruction(3_000, 1, WordAccess::new(0, 5), 56));
        let result = operation.execute(args);
        assert_eq!(4, result.next_addr_instruction);
        assert_eq!(2, r.get_j().get());

        r.set_overflow(true);
        let args = OperationArgs::new(3, &mut m, &mut r);
        let operation = JOV::new(Word::new_instruction(3_000, 1, WordAccess::new(0, 5), 56));
        let result = operation.execute(args);
        assert_eq!(3_000, result.next_addr_instruction);
        assert_eq!(4, r.get_j().get());

        r.set_overflow(true);
        let args = OperationArgs::new(4, &mut m, &mut r);
        let operation = JNOV::new(Word::new_instruction(3_000, 1, WordAccess::new(0, 5), 56));
        let result = operation.execute(args);
        assert_eq!(5, result.next_addr_instruction);
        assert_eq!(4, r.get_j().get());

        r.set_overflow(false);
        let args = OperationArgs::new(4, &mut m, &mut r);
        let operation = JNOV::new(Word::new_instruction(3_000, 1, WordAccess::new(0, 5), 56));
        let result = operation.execute(args);
        assert_eq!(3_000, result.next_addr_instruction);
        assert_eq!(5, r.get_j().get());
    }

    #[test]
    fn jl() {
        let mut m = Memory::new();
        let mut r = Registers::new();

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = JL::new(Word::new_instruction(2_000, 1, WordAccess::new(0, 5), 56));
        let result = operation.execute(args);
        assert_eq!(2, result.next_addr_instruction);
        assert_eq!(0, r.get_j().get());

        r.set_comparison(Comparison::LESS);
        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = JL::new(Word::new_instruction(2_000, 1, WordAccess::new(0, 5), 56));
        let result = operation.execute(args);
        assert_eq!(2_000, result.next_addr_instruction);
        assert_eq!(2, r.get_j().get());
    }

    #[test]
    fn je() {
        let mut m = Memory::new();
        let mut r = Registers::new();

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = JE::new(Word::new_instruction(2_000, 1, WordAccess::new(0, 5), 56));
        let result = operation.execute(args);
        assert_eq!(2, result.next_addr_instruction);
        assert_eq!(0, r.get_j().get());

        r.set_comparison(Comparison::EQUAL);
        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = JE::new(Word::new_instruction(2_000, 1, WordAccess::new(0, 5), 56));
        let result = operation.execute(args);
        assert_eq!(2_000, result.next_addr_instruction);
        assert_eq!(2, r.get_j().get());
    }

    #[test]
    fn jg() {
        let mut m = Memory::new();
        let mut r = Registers::new();

        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = JG::new(Word::new_instruction(2_000, 1, WordAccess::new(0, 5), 56));
        let result = operation.execute(args);
        assert_eq!(2, result.next_addr_instruction);
        assert_eq!(0, r.get_j().get());

        r.set_comparison(Comparison::GREATHER);
        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = JG::new(Word::new_instruction(2_000, 1, WordAccess::new(0, 5), 56));
        let result = operation.execute(args);
        assert_eq!(2_000, result.next_addr_instruction);
        assert_eq!(2, r.get_j().get());
    }

    #[test]
    fn jge() {
        let mut m = Memory::new();
        let mut r = Registers::new();

        r.set_comparison(Comparison::EQUAL);
        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = JGE::new(Word::new_instruction(2_000, 1, WordAccess::new(0, 5), 56));
        let result = operation.execute(args);
        assert_eq!(2_000, result.next_addr_instruction);
        assert_eq!(2, r.get_j().get());

        r.set_comparison(Comparison::GREATHER);
        let args = OperationArgs::new(2, &mut m, &mut r);
        let operation = JGE::new(Word::new_instruction(2_001, 1, WordAccess::new(0, 5), 56));
        let result = operation.execute(args);
        assert_eq!(2_001, result.next_addr_instruction);
        assert_eq!(3, r.get_j().get());
    }

    #[test]
    fn jne() {
        let mut m = Memory::new();
        let mut r = Registers::new();

        r.set_comparison(Comparison::LESS);
        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = JNE::new(Word::new_instruction(2_000, 1, WordAccess::new(0, 5), 56));
        let result = operation.execute(args);
        assert_eq!(2_000, result.next_addr_instruction);
        assert_eq!(2, r.get_j().get());

        r.set_comparison(Comparison::GREATHER);
        let args = OperationArgs::new(2, &mut m, &mut r);
        let operation = JNE::new(Word::new_instruction(2_001, 1, WordAccess::new(0, 5), 56));
        let result = operation.execute(args);
        assert_eq!(2_001, result.next_addr_instruction);
        assert_eq!(3, r.get_j().get());
    }

    #[test]
    fn jle() {
        let mut m = Memory::new();
        let mut r = Registers::new();

        r.set_comparison(Comparison::LESS);
        let args = OperationArgs::new(1, &mut m, &mut r);
        let operation = JLE::new(Word::new_instruction(2_000, 1, WordAccess::new(0, 5), 56));
        let result = operation.execute(args);
        assert_eq!(2_000, result.next_addr_instruction);
        assert_eq!(2, r.get_j().get());

        r.set_comparison(Comparison::EQUAL);
        let args = OperationArgs::new(2, &mut m, &mut r);
        let operation = JLE::new(Word::new_instruction(2_001, 1, WordAccess::new(0, 5), 56));
        let result = operation.execute(args);
        assert_eq!(2_001, result.next_addr_instruction);
        assert_eq!(3, r.get_j().get());
    }
}
