const SIGN: u32 = 0b10_000000_000000_000000_000000_000000;
const BYTE_1: u32 = 0b00_111111_000000_000000_000000_000000;
const BYTE_2: u32 = 0b00_000000_111111_000000_000000_000000;
const BYTE_3: u32 = 0b00_000000_000000_111111_000000_000000;
const BYTE_4: u32 = 0b00_000000_000000_000000_111111_000000;
const BYTE_5: u32 = 0b00_000000_000000_000000_000000_111111;

const BYTES: [u32; 6] = [SIGN, BYTE_1, BYTE_2, BYTE_3, BYTE_4, BYTE_5];

/// Word: 5 bytes and +- sign
/// byte is 6 bits from 0-63
#[derive(Debug)]
pub struct Word {
    value: u32,
}

impl Word {
    fn new(value: u32) -> Word {
        Word { value }
    }

    fn get(&self) -> u32 {
        self.value
    }

    fn get_by_access(&self, access: WordAccess) -> u32 {
        if access.left == 0 && access.right == 0 {
            return self.value & SIGN;
        }

        let mut result = 0;
        for b in access.left..access.right + 1 {
            if b == 0 {
                continue;
            }
            result |= self.value & BYTES[b as usize];
            // println!("{:#034b}", result)
        }

        result >>= 6 * (5 - access.right);
        if access.left == 0 {
            result |= self.value & SIGN;
        }

        result
    }
}

#[derive(Debug)]
pub struct WordAccess {
    pub left: u8,
    pub right: u8,
    pub spec: u8,
}

impl WordAccess {
    fn new(left: u8, right: u8) -> WordAccess {
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

        assert_eq!(0b10_000000_000000_000000_000000_000000, word.get_by_access(WordAccess::new(0, 0)), "0:0");
        assert_eq!(0b10_000000_000000_000000_000000_101111, word.get_by_access(WordAccess::new(0, 1)), "0:1");
        assert_eq!(0b10_000000_000000_000000_101111_110111, word.get_by_access(WordAccess::new(0, 2)), "0:2");
        assert_eq!(0b10_000000_000000_101111_110111_111011, word.get_by_access(WordAccess::new(0, 3)), "0:3");
        assert_eq!(0b10_000000_101111_110111_111011_111101, word.get_by_access(WordAccess::new(0, 4)), "0:4");
        assert_eq!(0b10_101111_110111_111011_111101_111110, word.get_by_access(WordAccess::new(0, 5)), "0:5");

        assert_eq!(0b00_000000_000000_000000_000000_101111, word.get_by_access(WordAccess::new(1, 1)), "1:1");
        assert_eq!(0b00_000000_000000_000000_101111_110111, word.get_by_access(WordAccess::new(1, 2)), "1:2");
        assert_eq!(0b00_000000_000000_101111_110111_111011, word.get_by_access(WordAccess::new(1, 3)), "1:3");
        assert_eq!(0b00_000000_101111_110111_111011_111101, word.get_by_access(WordAccess::new(1, 4)), "1:4");
        assert_eq!(0b00_101111_110111_111011_111101_111110, word.get_by_access(WordAccess::new(1, 5)), "1:5");

        assert_eq!(0b00_000000_000000_000000_000000_110111, word.get_by_access(WordAccess::new(2, 2)), "2:2");
        assert_eq!(0b00_000000_000000_000000_110111_111011, word.get_by_access(WordAccess::new(2, 3)), "2:3");
        assert_eq!(0b00_000000_000000_110111_111011_111101, word.get_by_access(WordAccess::new(2, 4)), "2:4");
        assert_eq!(0b00_000000_110111_111011_111101_111110, word.get_by_access(WordAccess::new(2, 5)), "2:5");

        assert_eq!(0b00_000000_000000_000000_000000_111011, word.get_by_access(WordAccess::new(3, 3)), "3:3");
        assert_eq!(0b00_000000_000000_000000_111011_111101, word.get_by_access(WordAccess::new(3, 4)), "3:4");
        assert_eq!(0b00_000000_000000_111011_111101_111110, word.get_by_access(WordAccess::new(3, 5)), "3:5");
        
        assert_eq!(0b00_000000_000000_000000_000000_111101, word.get_by_access(WordAccess::new(4, 4)), "4:4");
        assert_eq!(0b00_000000_000000_000000_111101_111110, word.get_by_access(WordAccess::new(4, 5)), "4:5");

        assert_eq!(0b00_000000_000000_000000_000000_111110, word.get_by_access(WordAccess::new(5, 5)), "5:5");
    }
}
