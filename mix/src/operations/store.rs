use crate::memory::short_word::ShortWord;
use crate::memory::word::Word;
use crate::memory::Bytes;
use crate::memory::Instruction;
use crate::memory::Memory;
use crate::operations::*;
use crate::registers::Registers;

fn store(instruction: impl Instruction, from: Word, mem: &mut Memory, reg: &Registers) {
    let f = instruction.get_f();
    let addr = get_indexed_addr(instruction, reg) as usize;

    let mut to = mem.get(addr);

    for i in 0..f.right - f.left + 1 {
        let b_from = 5 - i;
        let b_to = f.right - i;
        if b_from == 0 || b_to == 0 {
            continue;
        }
        to.set_byte(b_to, from.get_byte(b_from));
    }

    if f.left == 0 {
        to.set_sign(from.get_sign());
    }

    mem.set(addr, to.get());
}

pub struct STA {
    code: u32,
    execution_time: u32,
}

impl STA {
    pub fn new() -> STA {
        STA {
            code: 24,
            execution_time: 2,
        }
    }
}

impl Operation for STA {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        store(args.instruction, args.reg.get_a(), args.mem, args.reg);
        OperationResult::from_args(self.execution_time, args)
    }
}

pub struct STX {
    code: u32,
    execution_time: u32,
}

impl STX {
    pub fn new() -> STX {
        STX {
            code: 31,
            execution_time: 2,
        }
    }
}

impl Operation for STX {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        store(args.instruction, args.reg.get_x(), args.mem, args.reg);
        OperationResult::from_args(self.execution_time, args)
    }
}

pub struct STi {
    code: u32,
    execution_time: u32,
}

impl STi {
    pub fn new() -> STi {
        STi {
            code: 24,
            execution_time: 2,
        }
    }
}

impl Operation for STi {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let addr = get_indexed_addr(args.instruction, args.reg) as usize;
        let i = (args.instruction.get_c() - self.code as u8) as usize;
        let f = args.instruction.get_f();

        let from = args.reg.get_i(i);
        let mut to = args.mem.get(addr);
        to.set_byte(1, 0);
        to.set_byte(2, 0);
        to.set_byte(3, 0);

        for i in 0..f.right - f.left + 1 {
            let b_from = 5 - i;
            let b_to = f.right - i;
            if b_from == 0 || b_to == 0 {
                continue;
            }
            to.set_byte(b_to, from.get_byte(b_from));
        }

        if f.left == 0 {
            to.set_sign(from.get_sign());
        }

        args.mem.set(addr, to.get());

        OperationResult::from_args(self.execution_time, args)
    }
}

pub struct STJ {
    code: u32,
    execution_time: u32,
}

impl STJ {
    pub fn new() -> STJ {
        STJ {
            code: 32,
            execution_time: 2,
        }
    }
}

impl Operation for STJ {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let addr = get_indexed_addr(args.instruction, args.reg) as usize;

        let is_set_sign = args.instruction.get_f().left == 0;
        let left = args.instruction.get_f().left;
        let right = args.instruction.get_f().right;

        let from = args.reg.get_j();
        let mut to = args.mem.get(addr);

        for i in 0..right - left + 1 {
            let b_from = 5 - i;
            let b_to = right - i;
            if b_from == 0 || b_to == 0 {
                continue;
            }
            to.set_byte(b_to, from.get_byte(b_from));
        }

        if is_set_sign {
            to.set_sign(from.get_sign());
        }
        args.mem.set(addr, to.get());

        OperationResult::from_args(self.execution_time, args)
    }
}

pub struct STZ {
    code: u32,
    execution_time: u32,
}

impl STZ {
    pub fn new() -> STZ {
        STZ {
            code: 33,
            execution_time: 2,
        }
    }
}

impl Operation for STZ {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        store(args.instruction, Word::new(0), args.mem, args.reg);
        OperationResult::from_args(self.execution_time, args)
    }
}

fn print_by_bytes(w: &impl Bytes) {
    let sign = if w.get_sign() == 0 { "+" } else { "-" };
    println!(
        "{}{}{}{}{}{}",
        sign,
        w.get_byte(1),
        w.get_byte(2),
        w.get_byte(3),
        w.get_byte(4),
        w.get_byte(5)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sta() {
        let store = STA::new();

        let m_initial = Word::new_by_bytes(-1, &[1, 2, 3, 4, 5]);
        let r_initial = Word::new_by_bytes(0, &[6, 7, 8, 9, 0]);

        // print!("###### from: ");
        // print_by_bytes(&r_initial);
        // print!("###### to  : ");
        // print_by_bytes(&m_initial);

        let mut m = Memory::new();

        let mut r = Registers::new();
        r.set_a(r_initial);

        m.set(2_000, m_initial.get());
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 8),
            &mut m,
            &mut r,
        );
        store.execute(args);
        assert_by_bytes(m.get(2_000), 0, 6, 7, 8, 9, 0);

        m.set(2_000, m_initial.get());
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(1, 5), 8),
            &mut m,
            &mut r,
        );
        store.execute(args);
        assert_by_bytes(m.get(2_000), -1, 6, 7, 8, 9, 0);

        m.set(2_000, m_initial.get());
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(5, 5), 8),
            &mut m,
            &mut r,
        );
        store.execute(args);
        assert_by_bytes(m.get(2_000), -1, 1, 2, 3, 4, 0);

        m.set(2_000, m_initial.get());
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(2, 2), 8),
            &mut m,
            &mut r,
        );
        store.execute(args);
        assert_by_bytes(m.get(2_000), -1, 1, 0, 3, 4, 5);

        m.set(2_000, m_initial.get());
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(2, 3), 8),
            &mut m,
            &mut r,
        );
        store.execute(args);
        assert_by_bytes(m.get(2_000), -1, 1, 9, 0, 4, 5);

        m.set(2_000, m_initial.get());
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(0, 1), 8),
            &mut m,
            &mut r,
        );
        store.execute(args);
        assert_by_bytes(m.get(2_000), 0, 0, 2, 3, 4, 5);
    }

    #[test]
    fn stx() {
        let store = STX::new();

        let m_initial = Word::new_by_bytes(-1, &[1, 2, 3, 4, 5]);
        let r_initial = Word::new_by_bytes(0, &[6, 7, 8, 9, 0]);

        // print!("###### from: ");
        // print_by_bytes(&r_initial);
        // print!("###### to  : ");
        // print_by_bytes(&m_initial);

        let mut m = Memory::new();

        let mut r = Registers::new();
        r.set_x(r_initial);

        m.set(2_000, m_initial.get());
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 8),
            &mut m,
            &mut r,
        );
        store.execute(args);
        assert_by_bytes(m.get(2_000), 0, 6, 7, 8, 9, 0);

        m.set(2_000, m_initial.get());
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(1, 5), 8),
            &mut m,
            &mut r,
        );
        store.execute(args);
        assert_by_bytes(m.get(2_000), -1, 6, 7, 8, 9, 0);

        m.set(2_000, m_initial.get());
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(5, 5), 8),
            &mut m,
            &mut r,
        );
        store.execute(args);
        assert_by_bytes(m.get(2_000), -1, 1, 2, 3, 4, 0);

        m.set(2_000, m_initial.get());
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(2, 2), 8),
            &mut m,
            &mut r,
        );
        store.execute(args);
        assert_by_bytes(m.get(2_000), -1, 1, 0, 3, 4, 5);

        m.set(2_000, m_initial.get());
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(2, 3), 8),
            &mut m,
            &mut r,
        );
        store.execute(args);
        assert_by_bytes(m.get(2_000), -1, 1, 9, 0, 4, 5);

        m.set(2_000, m_initial.get());
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(0, 1), 8),
            &mut m,
            &mut r,
        );
        store.execute(args);
        assert_by_bytes(m.get(2_000), 0, 0, 2, 3, 4, 5);
    }

    #[test]
    fn sti() {
        let store = STi::new();
        let m_initial = Word::new_by_bytes(-1, &[1, 2, 3, 4, 5]);
        let r_initial = ShortWord::new_by_bytes(0, &[6, 7]);

        let mut m = Memory::new();

        let mut r = Registers::new();
        r.set_i(2, r_initial);

        m.set(2_000, m_initial.get());
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 26),
            &mut m,
            &mut r,
        );
        store.execute(args);
        assert_by_bytes(m.get(2_000), 0, 0, 0, 0, 6, 7);

        m.set(2_000, m_initial.get());
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(4, 5), 26),
            &mut m,
            &mut r,
        );
        store.execute(args);
        assert_by_bytes(m.get(2_000), -1, 0, 0, 0, 6, 7);

        m.set(2_000, m_initial.get());
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(4, 4), 26),
            &mut m,
            &mut r,
        );
        store.execute(args);
        assert_by_bytes(m.get(2_000), -1, 0, 0, 0, 7, 5);

        m.set(2_000, m_initial.get());
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(5, 5), 26),
            &mut m,
            &mut r,
        );
        store.execute(args);
        assert_by_bytes(m.get(2_000), -1, 0, 0, 0, 4, 7);
    }

    #[test]
    fn stj() {
        let store = STJ::new();

        let m_initial = Word::new_by_bytes(-1, &[1, 2, 3, 4, 5]);
        let r_initial = ShortWord::new_by_bytes(0, &[6, 7]);

        let mut m = Memory::new();

        let mut r = Registers::new();
        r.set_j(r_initial);

        m.set(2_000, m_initial.get());
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 2, WordAccess::new(0, 2), 24),
            &mut m,
            &mut r,
        );
        store.execute(args);
        assert_by_bytes(m.get(2_000), 0, 6, 7, 3, 4, 5);

        m.set(2_000, m_initial.get());
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 2, WordAccess::new(1, 2), 24),
            &mut m,
            &mut r,
        );
        store.execute(args);
        assert_by_bytes(m.get(2_000), -1, 6, 7, 3, 4, 5);

        m.set(2_000, m_initial.get());
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 2, WordAccess::new(1, 1), 24),
            &mut m,
            &mut r,
        );
        store.execute(args);
        assert_by_bytes(m.get(2_000), -1, 7, 2, 3, 4, 5);

        m.set(2_000, m_initial.get());
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 2, WordAccess::new(2, 2), 24),
            &mut m,
            &mut r,
        );
        store.execute(args);
        assert_by_bytes(m.get(2_000), -1, 1, 7, 3, 4, 5);
    }

    #[test]
    fn stz() {
        let store = STZ::new();
        let m_initial = Word::new_by_bytes(-1, &[1, 2, 3, 4, 5]);

        let mut m = Memory::new();

        let mut r = Registers::new();

        m.set(2_000, m_initial.get());
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 33),
            &mut m,
            &mut r,
        );
        store.execute(args);
        assert_by_bytes(m.get(2_000), 0, 0, 0, 0, 0, 0);

        m.set(2_000, m_initial.get());
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(1, 5), 33),
            &mut m,
            &mut r,
        );
        store.execute(args);
        assert_by_bytes(m.get(2_000), -1, 0, 0, 0, 0, 0);

        m.set(2_000, m_initial.get());
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(5, 5), 33),
            &mut m,
            &mut r,
        );
        store.execute(args);
        assert_by_bytes(m.get(2_000), -1, 1, 2, 3, 4, 0);

        m.set(2_000, m_initial.get());
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(2, 2), 33),
            &mut m,
            &mut r,
        );
        store.execute(args);
        assert_by_bytes(m.get(2_000), -1, 1, 0, 3, 4, 5);

        m.set(2_000, m_initial.get());
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(2, 3), 33),
            &mut m,
            &mut r,
        );
        store.execute(args);
        assert_by_bytes(m.get(2_000), -1, 1, 0, 0, 4, 5);

        m.set(2_000, m_initial.get());
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(0, 1), 33),
            &mut m,
            &mut r,
        );
        store.execute(args);
        assert_by_bytes(m.get(2_000), 0, 0, 2, 3, 4, 5);
    }

    #[test]
    fn sta_indexing() {
        let m_initial = Word::new_by_bytes(-1, &[1, 2, 3, 4, 5]);
        let r_initial = Word::new_by_bytes(0, &[6, 7, 8, 9, 0]);

        let mut m = Memory::new();

        let mut r = Registers::new();
        r.set_a(r_initial);
        r.set_i(3, ShortWord::new(10));

        m.set(2_000, m_initial.get());
        let args = OperationArgs::new(
            1,
            Word::new_instruction(1_990, 3, WordAccess::new(0, 5), 8),
            &mut m,
            &mut r,
        );
        let store = STA::new();
        store.execute(args);
        assert_by_bytes(m.get(2_000), 0, 6, 7, 8, 9, 0);
    }

    fn assert_by_bytes(
        actual: Word,
        sign: i8,
        byte_1: u8,
        byte_2: u8,
        byte_3: u8,
        byte_4: u8,
        byte_5: u8,
    ) {
        assert_eq!(actual.get_sign(), sign, "sing is wrong");
        assert_eq!(actual.get_byte(1), byte_1, "byte 1 is wrong");
        assert_eq!(actual.get_byte(2), byte_2, "byte 2 is wrong");
        assert_eq!(actual.get_byte(3), byte_3, "byte 3 is wrong");
        assert_eq!(actual.get_byte(4), byte_4, "byte 4 is wrong");
        assert_eq!(actual.get_byte(5), byte_5, "byte 5 is wrong");
    }
}
