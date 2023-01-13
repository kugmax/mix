use crate::memory::word::Bytes;
use crate::memory::word::Instruction;
use crate::memory::word::ShortWord;
use crate::memory::word::Word;
use crate::memory::word::WordAccess;
use crate::memory::Memory;
use crate::registers::Registers;

pub trait StoreOperation {
    fn execute(&self, mem: &mut Memory, reg: &Registers);

    fn store(instruction: impl Instruction, reg: Word, mem: &mut Memory) {
        let addr = instruction.get_address();
        let addr = addr.abs() as usize;

        let f = instruction.get_f();

        let from = reg;
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
}

struct STA {
    code: u32,
    execution_time: u32,

    instruction: Word,
}

impl STA {
    pub fn new(instruction: Word) -> STA {
        STA {
            code: 24,
            execution_time: 2,
            instruction: instruction,
        }
    }
}

impl StoreOperation for STA {
    fn execute(&self, mem: &mut Memory, reg: &Registers) {
        <STA as StoreOperation>::store(self.instruction, reg.get_a(), mem);
    }
}

struct STX {
    code: u32,
    execution_time: u32,

    instruction: Word,
}

impl STX {
    pub fn new(instruction: Word) -> STX {
        STX {
            code: 31,
            execution_time: 2,
            instruction: instruction,
        }
    }
}

impl StoreOperation for STX {
    fn execute(&self, mem: &mut Memory, reg: &Registers) {
        <STX as StoreOperation>::store(self.instruction, reg.get_x(), mem);
    }
}

struct STi {
    code: u32,
    execution_time: u32,
    i: u8,

    instruction: Word,
}

impl STi {
    pub fn new(instruction: Word) -> STi {
        let i = instruction.get_i();
        STi {
            code: 24 + i as u32,
            execution_time: 2,
            i: i,
            instruction,
        }
    }

    pub fn execute(&self, mem: &mut Memory, reg: &Registers) {
        let addr = self.instruction.get_address();
        let addr = addr.abs() as usize;

        let f = self.instruction.get_f();

        let from = reg.get_i(self.i as usize);
        let mut to = mem.get(addr);
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

        mem.set(addr, to.get());
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
        let store = STA::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 8));
        store.execute(&mut m, &r);
        assert_by_bytes(m.get(2_000), 0, 6, 7, 8, 9, 0);

        m.set(2_000, m_initial.get());
        let store = STA::new(Word::new_instruction(2_000, 0, WordAccess::new(1, 5), 8));
        store.execute(&mut m, &mut r);
        assert_by_bytes(m.get(2_000), -1, 6, 7, 8, 9, 0);

        m.set(2_000, m_initial.get());
        let store = STA::new(Word::new_instruction(2_000, 0, WordAccess::new(5, 5), 8));
        store.execute(&mut m, &mut r);
        assert_by_bytes(m.get(2_000), -1, 1, 2, 3, 4, 0);

        m.set(2_000, m_initial.get());
        let store = STA::new(Word::new_instruction(2_000, 0, WordAccess::new(2, 2), 8));
        store.execute(&mut m, &mut r);
        assert_by_bytes(m.get(2_000), -1, 1, 0, 3, 4, 5);

        m.set(2_000, m_initial.get());
        let store = STA::new(Word::new_instruction(2_000, 0, WordAccess::new(2, 3), 8));
        store.execute(&mut m, &mut r);
        assert_by_bytes(m.get(2_000), -1, 1, 9, 0, 4, 5);

        m.set(2_000, m_initial.get());
        let store = STA::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 1), 8));
        store.execute(&mut m, &mut r);
        assert_by_bytes(m.get(2_000), 0, 0, 2, 3, 4, 5);
    }

    #[test]
    fn stx() {
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
        let store = STX::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 8));
        store.execute(&mut m, &r);
        assert_by_bytes(m.get(2_000), 0, 6, 7, 8, 9, 0);

        m.set(2_000, m_initial.get());
        let store = STX::new(Word::new_instruction(2_000, 0, WordAccess::new(1, 5), 8));
        store.execute(&mut m, &mut r);
        assert_by_bytes(m.get(2_000), -1, 6, 7, 8, 9, 0);

        m.set(2_000, m_initial.get());
        let store = STX::new(Word::new_instruction(2_000, 0, WordAccess::new(5, 5), 8));
        store.execute(&mut m, &mut r);
        assert_by_bytes(m.get(2_000), -1, 1, 2, 3, 4, 0);

        m.set(2_000, m_initial.get());
        let store = STX::new(Word::new_instruction(2_000, 0, WordAccess::new(2, 2), 8));
        store.execute(&mut m, &mut r);
        assert_by_bytes(m.get(2_000), -1, 1, 0, 3, 4, 5);

        m.set(2_000, m_initial.get());
        let store = STX::new(Word::new_instruction(2_000, 0, WordAccess::new(2, 3), 8));
        store.execute(&mut m, &mut r);
        assert_by_bytes(m.get(2_000), -1, 1, 9, 0, 4, 5);

        m.set(2_000, m_initial.get());
        let store = STX::new(Word::new_instruction(2_000, 0, WordAccess::new(0, 1), 8));
        store.execute(&mut m, &mut r);
        assert_by_bytes(m.get(2_000), 0, 0, 2, 3, 4, 5);
    }

    #[test]
    fn sti() {
        let m_initial = Word::new_by_bytes(-1, &[1, 2, 3, 4, 5]);
        let r_initial = ShortWord::new_by_bytes(0, &[6, 7]);

        let mut m = Memory::new();

        let mut r = Registers::new();
        r.set_i(2, r_initial);

        m.set(2_000, m_initial.get());
        let store = STi::new(Word::new_instruction(2_000, 2, WordAccess::new(0, 5), 24));
        store.execute(&mut m, &r);
        assert_by_bytes(m.get(2_000), 0, 0, 0, 0, 6, 7);

        m.set(2_000, m_initial.get());
        let store = STi::new(Word::new_instruction(2_000, 2, WordAccess::new(4, 5), 24));
        store.execute(&mut m, &r);
        assert_by_bytes(m.get(2_000), -1, 0, 0, 0, 6, 7);

        m.set(2_000, m_initial.get());
        let store = STi::new(Word::new_instruction(2_000, 2, WordAccess::new(4, 4), 24));
        store.execute(&mut m, &r);
        assert_by_bytes(m.get(2_000), -1, 0, 0, 0, 7, 5);

        m.set(2_000, m_initial.get());
        let store = STi::new(Word::new_instruction(2_000, 2, WordAccess::new(5, 5), 24));
        store.execute(&mut m, &r);
        assert_by_bytes(m.get(2_000), -1, 0, 0, 0, 4, 7);
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
        assert_eq!(actual.get_byte(2), byte_2, "byte 1 is wrong");
        assert_eq!(actual.get_byte(3), byte_3, "byte 1 is wrong");
        assert_eq!(actual.get_byte(4), byte_4, "byte 1 is wrong");
        assert_eq!(actual.get_byte(5), byte_5, "byte 1 is wrong");
    }
}
