use crate::memory::word::Word;
use crate::memory::word::BYTES;
use crate::memory::word::BYTE_4;
use crate::memory::word::BYTE_5;
use crate::memory::word::SIGN;
use crate::memory::Bytes;
use crate::memory::word_access::WordAccess;

pub const MAX_2_BYTES: i32 = 4095; 

/// ShortWord: 2 bytes (BYTE_4,BYTE_5) and +- sign
/// byte is 6 bits from 0-63
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ShortWord {
    value: u32,
}

impl ShortWord {
    pub fn new(value: u32) -> ShortWord {
        ShortWord {
            value: ShortWord::to_short_value(value),
        }
    }

    pub fn new_from_signed(value: i32) -> ShortWord {
        let sign = Word::get_sign_mask_from_value(value);
        let value = value.abs() as u32;
        let value = ShortWord::to_short_value(value);
        let result = value | sign;
        ShortWord { value: result }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_from_signed() {
        let short = ShortWord::new_from_signed(4_000);
        assert_eq!(4_000, short.get());
        assert_eq!(0, short.get_sign());

        let short = ShortWord::new_from_signed(-4_000);
        assert_eq!(-4_000, short.get_signed_value());
        assert_eq!(-1, short.get_sign());
    }
}
