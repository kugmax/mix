use crate::memory::word::Word;

pub struct ShortRegister {
    value: Word,
}

pub struct Registers {
    rA: Word,
    rX: Word,
    // rI1: ShortRegister,
    // rI2: ShortRegister,
    // rI3: ShortRegister,
    // rI4: ShortRegister,
    // rI5: ShortRegister,
    // rI6: ShortRegister,
    //
    // rJ: ShortRegister,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            rA: Word::new(0),
            rX: Word::new(0),
        }
    }

    pub fn set_rA(&mut self, word: Word) {
        self.rA = word;
    }

    pub fn get_rA(&self) -> Word {
      self.rA
    }
}
