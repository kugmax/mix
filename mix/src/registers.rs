use crate::memory::short_word::ShortWord;
use crate::memory::word::Word;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RegisterType {
  A,
  X,
  I,
  J
}

#[derive(Debug, Copy, Clone)]
pub enum Comparison {
    LESS,
    EQUAL,
    GREATHER,
}

pub struct Registers {
    a: Word,
    x: Word,
    j_i: [ShortWord; 7], // 0 = reg J, 1-6 reg I
    is_overflow: bool,
    comparison: Comparison,
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
            is_overflow: false,
            comparison: Comparison::EQUAL,
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

    pub fn set_overflow(&mut self, is_overflow: bool) {
        self.is_overflow = is_overflow;
    }

    pub fn is_overflow(&self) -> bool {
        self.is_overflow
    }

    pub fn set_comparison(&mut self, comparison: Comparison) {
        self.comparison = comparison;
    }

    pub fn get_comparison(&self) -> Comparison {
        self.comparison
    }
}
