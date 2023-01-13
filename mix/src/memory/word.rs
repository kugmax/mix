const SIGN: u32 = 0b10_000000_000000_000000_000000_000000;
const BYTE_1: u32 = 0b00_111111_000000_000000_000000_000000;
const BYTE_2: u32 = 0b00_000000_111111_000000_000000_000000;
const BYTE_3: u32 = 0b00_000000_000000_111111_000000_000000;
const BYTE_4: u32 = 0b00_000000_000000_000000_111111_000000;
const BYTE_5: u32 = 0b00_000000_000000_000000_000000_111111;

const BYTES: [u32; 6] = [SIGN, BYTE_1, BYTE_2, BYTE_3, BYTE_4, BYTE_5];

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
}

/// Word: 5 bytes and +- sign
/// byte is 6 bits from 0-63
#[derive(Debug, Copy, Clone)]
pub struct Word {
    value: u32,
}

impl Word {
    pub fn new(value: u32) -> Word {
        Word { value }
    }

    pub fn new_from_signed(value: i32) -> Word {
        let sign = Word::get_sign_mask_from_value(value);
        let result = (value.abs() as u32) | sign;
        Word { value: result }
    }

    pub fn set(&mut self, value: u32) {
        self.value = value;
    }

    pub fn get(&self) -> u32 {
        self.value
    }

    pub fn get_by_access(&self, access: WordAccess) -> u32 {
        if access.left == 0 && access.right == 0 {
            return self.value & SIGN;
        }

        let mut result = 0;
        for b in access.left..access.right + 1 {
            if b == 0 {
                continue;
            }
            result |= self.value & BYTES[b as usize];
            // println!("{:#034b}", result);
        }

        result >>= 6 * (5 - access.right);
        if access.left == 0 {
            result |= self.value & SIGN;
        }

        result
    }

    pub fn get_negative_by_access(&self, access: WordAccess) -> u32 {
        let positive_value = self.get_by_access(access);
        let sign = positive_value & SIGN;
        let result = if sign == 0 {
            positive_value | SIGN
        } else {
            positive_value & !SIGN
        };
        result
    }

    pub fn get_signed_value(&self) -> i32 {
        let positive_val = (self.value & !SIGN) as i32;

        return if (self.value & SIGN) == 0 {
            positive_val
        } else {
            -positive_val
        };
    }

    pub fn get_sign_mask_from_value(value: i32) -> u32 {
        if value < 0 {
            SIGN
        } else {
            0
        }
    }
}

impl Instruction for Word {
    // TODO: needs to add asserts for 6 bit
    fn new_instruction(address: i32, i: u8, f: WordAccess, c: u8) -> Word {
        let sign = Word::get_sign_mask_from_value(address);

        let value = address.abs() as u32;

        let value = value << 6;
        let value = value | i as u32;

        let value = value << 6;
        let value = value | f.spec as u32;

        let value = value << 6;
        let value = value | c as u32;

        let value = value | sign;

        Word { value }
    }

    fn get_address(&self) -> i32 {
        let positive_val: i32 = ((self.value & (BYTE_1 | BYTE_2)) >> 6 * 3) as i32;

        return if (self.value & SIGN) == 0 {
            positive_val
        } else {
            -positive_val
        };
    }

    fn get_i(&self) -> u8 {
        ((self.value & BYTE_3) >> 6 * 2) as u8
    }

    fn get_f(&self) -> WordAccess {
        WordAccess::new_by_spec(((self.value & BYTE_4) >> 6) as u8)
    }

    fn get_c(&self) -> u8 {
        (self.value & BYTE_5) as u8
    }
}

impl Bytes for Word {
    type Item = Word;

    fn new_by_bytes(sign: i8, bytes: &[u8]) -> Word {
        let value = bytes[0] as u32;

        let value = value << 6;
        let value = value | bytes[1] as u32;

        let value = value << 6;
        let value = value | bytes[2] as u32;

        let value = value << 6;
        let value = value | bytes[3] as u32;

        let value = value << 6;
        let value = value | bytes[4] as u32;

        let value = value | Word::get_sign_mask_from_value(sign as i32);

        Word { value }
    }

    fn get_byte(&self, byte_number: u8) -> u8 {
        if byte_number < 1 || byte_number > 5 {
            panic!("{byte_number} is out of scope");
        }

        let mut result = self.value & BYTES[byte_number as usize];
        result >>= 6 * (5 - byte_number);
        result as u8
    }

    fn set_byte(&mut self, byte_number: u8, value: u8) {
        if byte_number < 1 || byte_number > 5 {
            panic!("{byte_number} is out of scope");
        }
        let value = value as u32;
        let value = value << 6 * (5 - byte_number);
        let result = self.value & !BYTES[byte_number as usize];
        self.value = value | result;
    }

    fn get_sign(&self) -> i8 {
        if self.value & SIGN == 0 {
            0
        } else {
            -1
        }
    }

    fn set_sign(&mut self, sign: i8) {
        let value = Word::get_sign_mask_from_value(sign as i32);
        let result = self.value & !SIGN;
        self.value = value | result;
    }
}

impl PartialEq for Word {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value 
    }
}

/// ShortWord: 2 bytes (BYTE_4,BYTE_5) and +- sign
/// byte is 6 bits from 0-63
#[derive(Debug, Copy, Clone)]
pub struct ShortWord {
    value: u32,
}

impl ShortWord {
    pub fn new(value: u32) -> ShortWord {
        ShortWord {
            value: ShortWord::to_short_value(value),
        }
    }

    pub fn set(&mut self, value: u32) {
        self.value = ShortWord::to_short_value(value);
    }

    pub fn get(&self) -> u32 {
        self.value
    }

    fn to_short_value(value: u32) -> u32 {
        value & (SIGN | BYTE_4 | BYTE_5)
    }
}

impl Bytes for ShortWord {
    type Item = ShortWord;

    fn new_by_bytes(sign: i8, bytes: &[u8]) -> ShortWord {
        let value = bytes[0] as u32;

        let value = value << 6;
        let value = value | bytes[1] as u32;

        let value = value | Word::get_sign_mask_from_value(sign as i32);

        ShortWord { value }
    }

    fn get_byte(&self, byte_number: u8) -> u8 {
        if byte_number < 1 || byte_number > 5 {
            panic!("{byte_number} is out of scope");
        }

        let byte_number = byte_number;
        let mut result = self.value & BYTES[byte_number as usize];
        result >>= 6 * (5 - byte_number);
        result as u8
    }

    fn set_byte(&mut self, byte_number: u8, value: u8) {
        if byte_number < 1 || byte_number > 5 {
            panic!("{byte_number} is out of scope");
        }
        let byte_number = byte_number;
        let value = value as u32;
        let value = value << 6 * (5 - byte_number);
        let result = self.value & !BYTES[byte_number as usize];
        self.value = value | result;
    }

    fn get_sign(&self) -> i8 {
        if self.value & SIGN == 0 {
            0
        } else {
            -1
        }
    }

    fn set_sign(&mut self, sign: i8) {
        let value = Word::get_sign_mask_from_value(sign as i32);
        let result = self.value & !SIGN;
        self.value = value | result;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct WordAccess {
    pub left: u8,
    pub right: u8,
    pub spec: u8,
}

impl WordAccess {
    pub fn new(left: u8, right: u8) -> WordAccess {
        if left > 5 || right > 5 {
            panic!("wrong left, right values {}:{}", left, right)
        }

        WordAccess {
            left,
            right,
            spec: (8 * left + right),
        }
    }

    fn new_by_spec(spec: u8) -> WordAccess {
        let left: u8 = spec / 8;
        let right: u8 = spec - left * 8;

        if left > right {
            panic!("left can't be greather then right {left}:{right}={spec}");
        }
        if left > 5 || right > 5 {
            panic!("wrong parsed values {left}:{right}={spec}");
        }

        WordAccess { left, right, spec }
    }
}

impl PartialEq for WordAccess {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.right == other.right && self.spec == other.spec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_access_new() {
        for l in 0..6 {
            for r in l..6 {
                assert_eq!(WordAccess::new(l, r), WordAccess::new_by_spec(8 * l + r));
                //println!("{l} {r}")
            }
        }
    }

    #[test]
    fn word_get_by_access() {
        let word = Word::new(0b10_101111_110111_111011_111101_111110);

        // let result = word.get_by_access(WordAccess::new(0, 1));
        // println!("{:#034b}", result);

        assert_eq!(
            0b10_000000_000000_000000_000000_000000,
            word.get_by_access(WordAccess::new(0, 0)),
            "0:0"
        );
        assert_eq!(
            0b10_000000_000000_000000_000000_101111,
            word.get_by_access(WordAccess::new(0, 1)),
            "0:1"
        );
        assert_eq!(
            0b10_000000_000000_000000_101111_110111,
            word.get_by_access(WordAccess::new(0, 2)),
            "0:2"
        );
        assert_eq!(
            0b10_000000_000000_101111_110111_111011,
            word.get_by_access(WordAccess::new(0, 3)),
            "0:3"
        );
        assert_eq!(
            0b10_000000_101111_110111_111011_111101,
            word.get_by_access(WordAccess::new(0, 4)),
            "0:4"
        );
        assert_eq!(
            0b10_101111_110111_111011_111101_111110,
            word.get_by_access(WordAccess::new(0, 5)),
            "0:5"
        );

        assert_eq!(
            0b00_000000_000000_000000_000000_101111,
            word.get_by_access(WordAccess::new(1, 1)),
            "1:1"
        );
        assert_eq!(
            0b00_000000_000000_000000_101111_110111,
            word.get_by_access(WordAccess::new(1, 2)),
            "1:2"
        );
        assert_eq!(
            0b00_000000_000000_101111_110111_111011,
            word.get_by_access(WordAccess::new(1, 3)),
            "1:3"
        );
        assert_eq!(
            0b00_000000_101111_110111_111011_111101,
            word.get_by_access(WordAccess::new(1, 4)),
            "1:4"
        );
        assert_eq!(
            0b00_101111_110111_111011_111101_111110,
            word.get_by_access(WordAccess::new(1, 5)),
            "1:5"
        );

        assert_eq!(
            0b00_000000_000000_000000_000000_110111,
            word.get_by_access(WordAccess::new(2, 2)),
            "2:2"
        );
        assert_eq!(
            0b00_000000_000000_000000_110111_111011,
            word.get_by_access(WordAccess::new(2, 3)),
            "2:3"
        );
        assert_eq!(
            0b00_000000_000000_110111_111011_111101,
            word.get_by_access(WordAccess::new(2, 4)),
            "2:4"
        );
        assert_eq!(
            0b00_000000_110111_111011_111101_111110,
            word.get_by_access(WordAccess::new(2, 5)),
            "2:5"
        );

        assert_eq!(
            0b00_000000_000000_000000_000000_111011,
            word.get_by_access(WordAccess::new(3, 3)),
            "3:3"
        );
        assert_eq!(
            0b00_000000_000000_000000_111011_111101,
            word.get_by_access(WordAccess::new(3, 4)),
            "3:4"
        );
        assert_eq!(
            0b00_000000_000000_111011_111101_111110,
            word.get_by_access(WordAccess::new(3, 5)),
            "3:5"
        );

        assert_eq!(
            0b00_000000_000000_000000_000000_111101,
            word.get_by_access(WordAccess::new(4, 4)),
            "4:4"
        );
        assert_eq!(
            0b00_000000_000000_000000_111101_111110,
            word.get_by_access(WordAccess::new(4, 5)),
            "4:5"
        );

        assert_eq!(
            0b00_000000_000000_000000_000000_111110,
            word.get_by_access(WordAccess::new(5, 5)),
            "5:5"
        );
    }

    #[test]
    fn get_negativev_by_access() {
        let word = Word::new(0b10_101111_110111_111011_111101_111110);

        let result = word.get_by_access(WordAccess::new(0, 1));
        // println!("{:#034b}", result);

        assert_eq!(
            0b00_000000_000000_000000_000000_000000,
            word.get_negative_by_access(WordAccess::new(0, 0)),
            "0:0"
        );
        assert_eq!(
            0b00_000000_000000_000000_000000_101111,
            word.get_negative_by_access(WordAccess::new(0, 1)),
            "0:1"
        );
        assert_eq!(
            0b00_000000_000000_000000_101111_110111,
            word.get_negative_by_access(WordAccess::new(0, 2)),
            "0:2"
        );
        assert_eq!(
            0b00_000000_000000_101111_110111_111011,
            word.get_negative_by_access(WordAccess::new(0, 3)),
            "0:3"
        );
        assert_eq!(
            0b00_000000_101111_110111_111011_111101,
            word.get_negative_by_access(WordAccess::new(0, 4)),
            "0:4"
        );
        assert_eq!(
            0b00_101111_110111_111011_111101_111110,
            word.get_negative_by_access(WordAccess::new(0, 5)),
            "0:5"
        );

        assert_eq!(
            0b10_000000_000000_000000_000000_101111,
            word.get_negative_by_access(WordAccess::new(1, 1)),
            "1:1"
        );
        assert_eq!(
            0b10_000000_000000_000000_101111_110111,
            word.get_negative_by_access(WordAccess::new(1, 2)),
            "1:2"
        );
        assert_eq!(
            0b10_000000_000000_101111_110111_111011,
            word.get_negative_by_access(WordAccess::new(1, 3)),
            "1:3"
        );
        assert_eq!(
            0b10_000000_101111_110111_111011_111101,
            word.get_negative_by_access(WordAccess::new(1, 4)),
            "1:4"
        );
        assert_eq!(
            0b10_101111_110111_111011_111101_111110,
            word.get_negative_by_access(WordAccess::new(1, 5)),
            "1:5"
        );

        assert_eq!(
            0b10_000000_000000_000000_000000_110111,
            word.get_negative_by_access(WordAccess::new(2, 2)),
            "2:2"
        );
        assert_eq!(
            0b10_000000_000000_000000_110111_111011,
            word.get_negative_by_access(WordAccess::new(2, 3)),
            "2:3"
        );
        assert_eq!(
            0b10_000000_000000_110111_111011_111101,
            word.get_negative_by_access(WordAccess::new(2, 4)),
            "2:4"
        );
        assert_eq!(
            0b10_000000_110111_111011_111101_111110,
            word.get_negative_by_access(WordAccess::new(2, 5)),
            "2:5"
        );

        assert_eq!(
            0b10_000000_000000_000000_000000_111011,
            word.get_negative_by_access(WordAccess::new(3, 3)),
            "3:3"
        );
        assert_eq!(
            0b10_000000_000000_000000_111011_111101,
            word.get_negative_by_access(WordAccess::new(3, 4)),
            "3:4"
        );
        assert_eq!(
            0b10_000000_000000_111011_111101_111110,
            word.get_negative_by_access(WordAccess::new(3, 5)),
            "3:5"
        );

        assert_eq!(
            0b10_000000_000000_000000_000000_111101,
            word.get_negative_by_access(WordAccess::new(4, 4)),
            "4:4"
        );
        assert_eq!(
            0b10_000000_000000_000000_111101_111110,
            word.get_negative_by_access(WordAccess::new(4, 5)),
            "4:5"
        );

        assert_eq!(
            0b10_000000_000000_000000_000000_111110,
            word.get_negative_by_access(WordAccess::new(5, 5)),
            "5:5"
        );
    }

    #[test]
    fn word_get_set_bytes() {
        let mut w = Word::new_by_bytes(0, &[1, 2, 3, 4, 5]);

        w.set_sign(-1);
        assert_eq!(-1, w.get_sign());

        w.set_byte(1, 9);
        assert_eq!(9, w.get_byte(1));

        w.set_byte(2, 8);
        assert_eq!(8, w.get_byte(2));

        w.set_byte(3, 7);
        assert_eq!(7, w.get_byte(3));

        w.set_byte(4, 6);
        assert_eq!(6, w.get_byte(4));

        w.set_byte(5, 10);
        assert_eq!(10, w.get_byte(5));

        w.set_sign(0);
        assert_eq!(0, w.get_sign());
    }

    #[test]
    fn word_as_instruction() {
        let w = Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 8);
        assert_eq!(2_000, w.get_address());
        assert_eq!(0, w.get_i());
        assert_eq!(5, w.get_f().spec);
        assert_eq!(8, w.get_c());

        let w = Word::new_instruction(-2_000, 2, WordAccess::new(1, 3), 8);
        assert_eq!(-2_000, w.get_address());
        assert_eq!(2, w.get_i());
        assert_eq!(11, w.get_f().spec);
        assert_eq!(8, w.get_c());
    }
}
