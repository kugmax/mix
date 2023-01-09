use crate::memory::word::Word;

pub struct ShortRegister {
    value: Word,
}

pub struct Registers {
    a: Word,
    x: Word,
    // i1: ShortRegister,
    // i2: ShortRegister,
    // i3: ShortRegister,
    // i4: ShortRegister,
    // i5: ShortRegister,
    // i6: ShortRegister,
    //
    // rj: ShortRegister,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: Word::new(0),
            x: Word::new(0),
        }
    }

    pub fn set_a(&mut self, word: Word) {
        self.a = word;
    }

    pub fn get_a(&self) -> Word {
      self.a
    }

    pub fn set_x(&mut self, word: Word) {
        self.x = word;
    }

    pub fn get_x(&self) -> Word {
      self.x
    }
}
