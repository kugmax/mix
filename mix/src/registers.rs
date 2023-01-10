use crate::memory::word::ShortWord;
use crate::memory::word::Word;

pub struct Registers {
    a: Word,
    x: Word,
    j_i: [ShortWord; 7], // 0 = reg J, 1-6 reg I
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: Word::new(0),
            x: Word::new(0),
            j_i: [
                ShortWord::new(0),
                ShortWord::new(0),
                ShortWord::new(0),
                ShortWord::new(0),
                ShortWord::new(0),
                ShortWord::new(0),
                ShortWord::new(0),
            ],
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

    pub fn set_j(&mut self, word: ShortWord) {
        self.j_i[0] = word;
    }

    pub fn get_j(&self) -> ShortWord {
        self.j_i[0]
    }

    pub fn set_i(&mut self, i: usize, word: ShortWord) {
        if i < 1 || i > 6 {
          panic!("{} wrong register I index", i)
        }
        self.j_i[i] = word;
    }

    pub fn get_i(&self, i: usize) -> ShortWord {
        if i < 1 || i > 6 {
          panic!("{} wrong register I index", i)
        }
        self.j_i[i]
    }
}
