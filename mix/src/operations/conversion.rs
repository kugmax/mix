use crate::memory::word::Word;
use crate::memory::word::MAX_5_BYTES;
use crate::memory::Bytes;
use crate::operations::*;

pub const SYMBOLS: [char; 56] = [
    ' ', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', '\u{0394}', 'J', 'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', '\u{2211}', '\u{03A0}', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2',
    '3', '4', '5', '6', '7', '8', '9', '.', ',', '(', ')', '+', '-', '*', '/', '=', '$', '<', '>',
    '@', ';', ':', '\'',
];

pub struct NUM {
    code: u32,
    execution_time: u32,
    f: u32,
    instruction: Word,
}
impl NUM {
    pub fn new(instruction: Word) -> NUM {
        NUM {
            code: 5,
            execution_time: 10,
            f: 0,
            instruction: instruction,
        }
    }
}
impl Operation for NUM {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let mut result: u64 = 0;
        for i in 0..10 {
            result *= 10;
            let mut val: u64 = 0;
            if i < 5 {
                val = args.reg.get_a().get_byte(i + 1) as u64;
            } else {
                val = args.reg.get_x().get_byte(i - 4) as u64;
            }
            val %= 10;
            result += val;
        }

        if result > MAX_5_BYTES as u64 {
            result /= (5 as u64).pow(5);
        }

        let mut ra = Word::new(result as u32);
        ra.set_sign(args.reg.get_a().get_sign());

        args.reg.set_a(ra);

        OperationResult::from_args(self.execution_time, args)
    }
    fn get_name(&self) -> String {
        String::from("NUM")
    }
}

pub struct CHAR {
    code: u32,
    execution_time: u32,
    f: u32,
    instruction: Word,
}
impl CHAR {
    pub fn new(instruction: Word) -> CHAR {
        CHAR {
            code: 5,
            execution_time: 10,
            f: 1,
            instruction: instruction,
        }
    }
}
impl Operation for CHAR {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let mut to_convert: i32 = args.reg.get_a().get_signed_value().abs();
        let mut ra = args.reg.get_a();
        let mut rx = args.reg.get_x();

        for i in 0..10 {
            let val: u8 = 30 + (to_convert % 10) as u8;

            if i < 5 {
                rx.set_byte(5 - i, val);
            } else {
                ra.set_byte(10 - i, val);
            }
            to_convert /= 10;
        }
        args.reg.set_a(ra);
        args.reg.set_x(rx);

        OperationResult::from_args(self.execution_time, args)
    }
    fn get_name(&self) -> String {
        String::from("CHAR")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbols_table() {
        assert_eq!('A', SYMBOLS[1]);
        assert_eq!('\u{0394}', SYMBOLS[10]);
        assert_eq!('\u{2211}', SYMBOLS[20]);
        assert_eq!('\u{03A0}', SYMBOLS[21]);
        assert_eq!('\'', SYMBOLS[55]);

        for i in 0..56 {
            print!("{} ", SYMBOLS[i]);
            if i % 10 == 0 {
                println!("");
            }
        }
    }

    #[test]
    fn num() {
        let mut r = Registers::new();
        let mut m = Memory::new();

        let ra = Word::new_by_bytes(-1, &[0, 0, 31, 32, 39]);
        let rx = Word::new_by_bytes(0, &[37, 57, 47, 30, 30]);

        r.set_a(ra);
        r.set_x(rx);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let op = NUM::new(Word::new_instruction(0, 0, WordAccess::new(0, 5), 0));
        op.execute(args);
        assert_eq!(-12977700, r.get_a().get_signed_value());

        let ra = Word::new_by_bytes(0, &[9, 39, 39, 39, 39]);
        let rx = Word::new_by_bytes(0, &[39, 59, 49, 39, 39]);

        r.set_a(ra);
        r.set_x(rx);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let op = NUM::new(Word::new_instruction(0, 0, WordAccess::new(0, 5), 0));
        op.execute(args);

        assert_eq!(3_199_999, r.get_a().get_signed_value());
    }

    #[test]
    fn char() {
        let mut r = Registers::new();
        let mut m = Memory::new();

        let ra = Word::new_from_signed(-12977699);
        let rx = Word::new_by_bytes(0, &[37, 57, 47, 30, 30]);

        r.set_a(ra);
        r.set_x(rx);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let op = CHAR::new(Word::new_instruction(0, 0, WordAccess::new(0, 5), 0));
        op.execute(args);
        assert_by_bytes(r.get_a(), -1, 30, 30, 31, 32, 39);
        assert_by_bytes(r.get_x(), 0, 37, 37, 36, 39, 39);
    }

    fn assert_by_bytes(
        actual: Word,
        sign: i8,
        byte_1: u8,
        byte_2: u8,
        byte_3: u8,
        byte_4: u8,
        byte_5: u8,
    ) {
        assert_eq!(actual.get_sign(), sign, "sing is wrong");
        assert_eq!(actual.get_byte(1), byte_1, "byte 1 is wrong");
        assert_eq!(actual.get_byte(2), byte_2, "byte 2 is wrong");
        assert_eq!(actual.get_byte(3), byte_3, "byte 3 is wrong");
        assert_eq!(actual.get_byte(4), byte_4, "byte 4 is wrong");
        assert_eq!(actual.get_byte(5), byte_5, "byte 5 is wrong");
    }
}
