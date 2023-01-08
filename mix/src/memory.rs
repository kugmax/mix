use crate::memory::data::Word;

pub mod data;

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

    pub fn get(&self, i: usize) -> &Word {
        self.mem.get(i).expect("out of range {i}")
    }
    
    pub fn set(&mut self, i:usize, value:u32) {
        self.mem.get_mut(i).expect("out of range {i}").set(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_value_by_index() {

        let mut m = Memory::new();
        println!("#######################3 {:#?}", m.get(0));
        m.set(1, 66);
        println!("#######################3 {:#?}", m.get(1));
        
    }
}
