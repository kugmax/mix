use crate::memory::short_word::ShortWord;
use crate::memory::word::Word;
use crate::memory::word_access::WordAccess;
use crate::memory::Bytes;
use crate::memory::Instruction;
use crate::memory::Memory;
use crate::operations::*;
use crate::registers::RegisterType;
use crate::registers::Registers;

fn load(instruction: impl Instruction, r_type: RegisterType, mem: &Memory, reg: &mut Registers) {
    let f = instruction.get_f();
    let mem_cell = get_memory_cell(instruction, mem, reg);
    let value = Word::new(mem_cell.get_by_access(f));

    if r_type == RegisterType::A {
        reg.set_a(value);
    } else if r_type == RegisterType::X {
        reg.set_x(value);
    } else {
        panic!("operation is not supported for register {:#?}", r_type);
    }
}

fn load_negative(
    instruction: impl Instruction,
    r_type: RegisterType,
    mem: &Memory,
    reg: &mut Registers,
) {
    let f = instruction.get_f();
    let mem_cell = get_memory_cell(instruction, mem, reg);

    let value = Word::new(mem_cell.get_negative_by_access(f));

    if r_type == RegisterType::A {
        reg.set_a(value);
    } else if r_type == RegisterType::X {
        reg.set_x(value);
    } else {
        panic!("operation is not supported for register {:#?}", r_type);
    }
}

pub struct LDA {
    code: u32,
    execution_time: u32,
    instruction: Word,
}

impl LDA {
    pub fn new(instruction: Word) -> LDA {
        LDA {
            code: 8,
            execution_time: 2,
            instruction: instruction,
        }
    }
}

impl Operation for LDA {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        load(self.instruction, RegisterType::A, args.mem, args.reg);

        OperationResult::from_args(self.execution_time, args)
    }
    fn get_name(&self) -> String {
        String::from("LDA")
    }
}

pub struct LDX {
    code: u32,
    execution_time: u32,
    instruction: Word,
}

impl LDX {
    pub fn new(instruction: Word) -> LDX {
        LDX {
            code: 15,
            execution_time: 2,
            instruction: instruction,
        }
    }
}

impl Operation for LDX {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        load(self.instruction, RegisterType::X, args.mem, args.reg);

        OperationResult::from_args(self.execution_time, args)
    }
    fn get_name(&self) -> String {
        String::from("LDX")
    }
}

pub struct LDi {
    code: u32,
    execution_time: u32,
    instruction: Word,
}

impl LDi {
    pub fn new(instruction: Word) -> LDi {
        LDi {
            code: 8,
            execution_time: 2,
            instruction: instruction,
        }
    }
}

impl Operation for LDi {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let mem_cell = get_memory_cell(self.instruction, args.mem, args.reg);
        let to = (self.instruction.get_c() - self.code as u8) as usize;

        let value = ShortWord::new(mem_cell.get_by_access(self.instruction.get_f()));

        // println!("### {to} {}", value.get_signed_value());

        args.reg.set_i(to, value);

        OperationResult::from_args(self.execution_time, args)
    }
    fn get_name(&self) -> String {
        let i = (self.instruction.get_c() - self.code as u8);
        String::from("LD") + &i.to_string() 
    }
}

pub struct LDAN {
    code: u32,
    execution_time: u32,
    instruction: Word,
}

impl LDAN {
    pub fn new(instruction: Word) -> LDAN {
        LDAN {
            code: 16,
            execution_time: 2,
            instruction: instruction,
        }
    }
}

impl Operation for LDAN {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let mut set_a = |w| args.reg.set_a(w);
        load_negative(self.instruction, RegisterType::A, args.mem, args.reg);

        OperationResult::from_args(self.execution_time, args)
    }
    fn get_name(&self) -> String {
        String::from("LDAN")
    }
}

pub struct LDXN {
    code: u32,
    execution_time: u32,
    instruction: Word,
}

impl LDXN {
    pub fn new(instruction: Word) -> LDXN {
        LDXN {
            code: 23,
            execution_time: 2,
            instruction: instruction,
        }
    }
}

impl Operation for LDXN {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let mut set_a = |w| args.reg.set_a(w);
        load_negative(self.instruction, RegisterType::X, args.mem, args.reg);

        OperationResult::from_args(self.execution_time, args)
    }
    fn get_name(&self) -> String {
        String::from("LDXN")
    }
}

pub struct LDiN {
    code: u32,
    execution_time: u32,
    instruction: Word,
}

impl LDiN {
    pub fn new(instruction: Word) -> LDiN {
        LDiN {
            code: 16,
            execution_time: 2,
            instruction: instruction,
        }
    }
}

impl Operation for LDiN {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let mem_cell = get_memory_cell(self.instruction, args.mem, args.reg);
        let value = ShortWord::new(mem_cell.get_negative_by_access(self.instruction.get_f()));

        let to = (self.instruction.get_c() - self.code as u8) as usize;
        args.reg.set_i(to, value);

        OperationResult::from_args(self.execution_time, args)
    }
    fn get_name(&self) -> String {
        let i = (self.instruction.get_c() - self.code as u8);
        String::from("LD") + &i.to_string() + &String::from("N")
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

        let args = OperationArgs::new(1, &mut m, &mut r);
        let lda = LDA::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 8));
        lda.execute(args);
        assert_instruction(r.get_a(), -80, 3, WordAccess::new(0, 5), 4);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let lda = LDA::new(Word::new_instruction(2_000, 0, WordAccess::new(1, 5), 8));
        lda.execute(args);
        assert_instruction(r.get_a(), 80, 3, WordAccess::new(0, 5), 4);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let lda = LDA::new(Word::new_instruction(2_000, 0, WordAccess::new(3, 5), 8));
        lda.execute(args);
        assert_instruction(r.get_a(), 0, 3, WordAccess::new(0, 5), 4);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let lda = LDA::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 3), 8));
        lda.execute(args);
        assert_eq!(
            r.get_a().get_by_access(WordAccess::new(0, 0)),
            0b10_000000_000000_000000_000000_000000
        );
        assert_eq!(r.get_a().get_by_access(WordAccess::new(1, 2)), 0);
        assert_eq!(r.get_a().get_by_access(WordAccess::new(3, 4)), 80);
        assert_eq!(r.get_a().get_by_access(WordAccess::new(5, 5)), 3);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let lda = LDA::new(Word::new_instruction(2_000, 0, WordAccess::new(4, 4), 8));
        lda.execute(args);
        assert_instruction(r.get_a(), 0, 0, WordAccess::new(0, 0), 5);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let lda = LDA::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 0), 8));
        lda.execute(args);
        assert_eq!(
            r.get_a().get_by_access(WordAccess::new(0, 0)),
            0b10_000000_000000_000000_000000_000000
        );
        assert_eq!(r.get_a().get_by_access(WordAccess::new(1, 5)), 0);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let lda = LDA::new(Word::new_instruction(2_000, 0, WordAccess::new(1, 1), 8));
        lda.execute(args);
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
        m.set(2_010, word.get());

        let mut r = Registers::new();
        r.set_i(1, ShortWord::new(10));

        let args = OperationArgs::new(1, &mut m, &mut r);
        let ldx = LDX::new(Word::new_instruction(2_000, 1, WordAccess::new(0, 5), 8));
        ldx.execute(args);
        assert_instruction(r.get_x(), -80, 3, WordAccess::new(0, 5), 4);
    }

    #[test]
    fn ldi() {
        let word = Word::new_instruction(-80, 3, WordAccess::new(0, 5), 4);

        let mut m = Memory::new();
        m.set(2_000, word.get());

        let mut r = Registers::new();

        let args = OperationArgs::new(1, &mut m, &mut r);
        let load = LDi::new(Word::new_instruction(2_000, 2, WordAccess::new(0, 5), 10));
        load.execute(args);

        let ri = r.get_i(2);

        assert_eq!(ri.get_sign(), -1, "ri sign is wrogn");
        assert_eq!(ri.get_byte(4), 5, "ri byte 4 is wrong");
        assert_eq!(ri.get_byte(5), 4, "ri byte 5 is wrong");
    }

    #[test]
    fn ldan() {
        let word = Word::new_instruction(-80, 3, WordAccess::new(0, 5), 4);

        let mut m = Memory::new();
        m.set(2_000, word.get());

        let mut r = Registers::new();

        let args = OperationArgs::new(1, &mut m, &mut r);
        let load = LDAN::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 8));
        load.execute(args);
        assert_instruction(r.get_a(), 80, 3, WordAccess::new(0, 5), 4);
    }

    #[test]
    fn ldxn() {
        let word = Word::new_instruction(-80, 3, WordAccess::new(0, 5), 4);

        let mut m = Memory::new();
        m.set(2_000, word.get());

        let mut r = Registers::new();

        let args = OperationArgs::new(1, &mut m, &mut r);
        let load = LDXN::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 8));
        load.execute(args);
        assert_instruction(r.get_x(), 80, 3, WordAccess::new(0, 5), 4);
    }

    #[test]
    fn ldin() {
        let word = Word::new_instruction(-80, 3, WordAccess::new(0, 5), 4);

        let mut m = Memory::new();
        m.set(2_000, word.get());

        let mut r = Registers::new();

        let args = OperationArgs::new(1, &mut m, &mut r);
        let load = LDiN::new(Word::new_instruction(2_000, 2, WordAccess::new(0, 5), 18));
        load.execute(args);

        let ri = r.get_i(2);

        assert_eq!(ri.get_sign(), 0, "ri sign is wrogn");
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
