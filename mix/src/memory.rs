use crate::memory::word::Word;
use crate::memory::word_access::WordAccess;

pub mod short_word;
pub mod word;
pub mod word_access;

pub const POSITIVE: Sign = Sign::PLUS(0);
pub const NEGATIVE: Sign = Sign::MINUS(-1);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Sign {
    PLUS(i32),
    MINUS(i32),
}

// pub fn swap_sign(sign: Sign) -> Sign {
// return match sign {
// POSITIVE => NEGATIVE,
// NEGATIVE => POSITIVE,
// _ => panic!("unsupported sign {:#?}", sign)
// }
// }

pub fn swap_sign(sign: i8) -> i8 {
    return if sign == 0 { -1 } else { 0 };
}

pub trait Instruction {
    fn new_instruction(address: i32, i: u8, f: WordAccess, c: u8) -> Word;

    fn get_address(&self) -> i32;
    fn get_i(&self) -> u8;
    fn get_f(&self) -> WordAccess;
    fn get_c(&self) -> u8;
}

pub trait Bytes {
    type Item;

    fn new_by_bytes(sign: i8, bytes: &[u8]) -> Self::Item;
    fn get_byte(&self, byte_number: u8) -> u8;
    fn set_byte(&mut self, byte_number: u8, value: u8);
    fn get_sign(&self) -> i8; // 0 or -1
    fn set_sign(&mut self, sign: i8); // 0 or -1
                                      //
    fn set_bytes(&mut self, byte_numbes: &[u8], value: u32);
    fn get_bytes(&self, byte_numbes: &[u8]) -> u32;
}

pub struct Memory {
    mem: Vec<Box<Word>>,
}

impl Memory {
    pub fn new() -> Memory {
        let mut mem = Vec::new();
        for _i in 0..4_0000 {
            mem.push(Box::new(Word::new(0)));
        }

        Memory { mem: mem }
    }

    pub fn get(&self, i: usize) -> Word {
        Word::new(self.mem.get(i).expect("memory is out of range {i}").get())
    }

    pub fn set(&mut self, i: usize, value: u32) {
        self.mem
            .get_mut(i)
            .expect("memory is out of range {i}")
            .set(value);
    }

    pub fn set_instruction(&mut self, mem_i: usize, address: i32, i: u8, f: u8, c: u8) {
        self.mem
            .get_mut(mem_i)
            .expect("memory is out of range {i}")
            .set(Word::new_instruction(address, i, WordAccess::new_by_spec(f), c).get());
    }

    pub fn set_instr_as_bytes(&mut self, mem_i: usize, address: i32, i: u8, f: u8, c: u8) {
        let mut word = Word::new_instruction(address, i, WordAccess::new_by_spec(0), c);
        word.set_byte(4, f);
        self.mem
            .get_mut(mem_i)
            .expect("memory is out of range {i}")
            .set(word.get());
    }

    pub fn set_word(&mut self, i: usize, word: Word) {
        self.mem
            .get_mut(i)
            .expect("memory is out of range {i}")
            .set(word.get());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_value_by_index() {
        let mut m = Memory::new();
        // println!("#######################3 {:#?}", m.get(0));
        m.set(1, 66);
        // println!("#######################3 {:#?}", m.get(1));
        assert_eq!(66, m.get(1).get());
    }
}
