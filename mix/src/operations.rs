use crate::memory::word::Word;

struct LDA {
    code: u32,
    execution_time: u32,

    instruction: Word,
}

impl LDA {
    pub fn new(instruction: Word) -> LDA {
        LDA(8, 2, instruction)
    }

    pub fn execute(mem: &Memory, &mut reg: Registers) {
// go to memory by AA get value store to the rA
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_value_from_memory() {

        let mut m = Memory::new();
        m.set(1, 66);


        let instruct1 = Word::new();

        let lda = LDA::new()
        
    }
}
