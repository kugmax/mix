use crate::memory::word::Word;
use crate::memory::word_access::WordAccess;

pub mod word;
pub mod word_access;
pub mod short_word;

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
        Word::new(self.mem.get(i).expect("out of range {i}").get())
    }

    pub fn set(&mut self, i: usize, value: u32) {
        self.mem.get_mut(i).expect("out of range {i}").set(value);
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
