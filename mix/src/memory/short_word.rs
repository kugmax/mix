use crate::memory::word::Word;
use crate::memory::word::BYTES;
use crate::memory::word::BYTE_4;
use crate::memory::word::BYTE_5;
use crate::memory::word::SIGN;
use crate::memory::Bytes;

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

    pub fn get_signed_value(&self) -> i32 {
        let positive_val = (self.value & !SIGN) as i32;

        return if (self.value & SIGN) == 0 {
            positive_val
        } else {
            -positive_val
        };
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

    fn set_bytes(&mut self, byte_numbers: &[u8], value: u32) {
        panic!("not implemented");
        // for b in byte_numbers {
        // self.set_byte(*b, value & BYTES[*b as usize]);
        // }
    }

    fn get_bytes(&self, byte_numbes: &[u8]) -> u32 {
        panic!("not implemented");
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
