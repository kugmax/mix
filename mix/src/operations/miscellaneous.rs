use crate::memory::short_word::ShortWord;
use crate::memory::word::Word;
use crate::memory::word::ABS;
use crate::memory::word::BYTE_1_FROM_10;
use crate::memory::word::BYTE_10_FROM_10;
use crate::memory::word_access::WordAccess;
use crate::memory::Bytes;
use crate::memory::Instruction;
use crate::memory::Memory;
use crate::operations::*;
use crate::registers::RegisterType;
use crate::registers::Registers;

fn shift(value: Word, shift_bytes: u32, op: &mut dyn Fn(u32, u32) -> u32) -> Word {
    if shift_bytes == 0 {
        return value;
    }

    let sign = value.get_sign();

    if shift_bytes >= 5 {
        let mut result = Word::new(0);
        result.set_sign(sign);
        return result;
    }

    let mut tmp = value.get();
    tmp &= ABS;
    tmp = op(tmp, shift_bytes);
    tmp &= ABS;

    let mut result = Word::new(tmp);
    result.set_sign(sign);

    return result;
}

fn shift_ax(
    ra: Word,
    rx: Word,
    shift_bytes: u32,
    op: &mut dyn Fn(u64, u32) -> u64,
) -> (Word, Word) {
    if shift_bytes == 0 {
        return (ra, rx);
    }

    let sign_ra = ra.get_sign();
    let sign_rx = rx.get_sign();

    if shift_bytes >= 10 {
        let mut result_ra = Word::new(0);
        result_ra.set_sign(sign_ra);

        let mut result_rx = Word::new(0);
        result_rx.set_sign(sign_rx);

        return (result_ra, result_rx);
    }

    let mut tmp_ra = (ra.get() & ABS) as u64;
    tmp_ra <<= 30;
    let tmp_rx = (rx.get() & ABS) as u64;

    let mut tmp = tmp_ra | tmp_rx;
    tmp = op(tmp, shift_bytes);

    let mut tmp_ra = (tmp >> 30) as u32;
    tmp_ra &= ABS;

    let mut result_ra = Word::new(tmp_ra);
    result_ra.set_sign(sign_ra);

    let mut tmp_rx = tmp as u32;
    tmp_rx &= ABS;

    let mut result_rx = Word::new(tmp_rx);
    result_rx.set_sign(sign_rx);

    return (result_ra, result_rx);
}

fn rotate_ax(
    ra: Word,
    rx: Word,
    shift_bytes: u32,
    op: &mut dyn Fn(u64, u32) -> u64,
) -> (Word, Word) {
    let mut shift_bytes = shift_bytes % 10;

    if shift_bytes == 0 {
        return (ra, rx);
    }

    let sign_ra = ra.get_sign();
    let sign_rx = rx.get_sign();

    let mut result_ra = ra;
    let mut result_rx = rx;

    if shift_bytes >= 5 {
        result_ra = Word::new(rx.get());
        result_ra.set_sign(sign_ra);

        result_rx = Word::new(ra.get());
        result_rx.set_sign(sign_rx);

        if shift_bytes == 5 {
            return (result_ra, result_rx);
        }
        shift_bytes -= 5;
    }

    let mut tmp_ra = (result_ra.get() & ABS) as u64;
    tmp_ra <<= 30;
    let tmp_rx = (result_rx.get() & ABS) as u64;

    let mut tmp = tmp_ra | tmp_rx;
    tmp = op(tmp, shift_bytes); // rotate

    let mut tmp_ra = (tmp >> 30) as u32;
    tmp_ra &= ABS;

    let mut result_ra = Word::new(tmp_ra);
    result_ra.set_sign(sign_ra);

    let mut tmp_rx = tmp as u32;
    tmp_rx &= ABS;

    let mut result_rx = Word::new(tmp_rx);
    result_rx.set_sign(sign_rx);

    return (result_ra, result_rx);
}

pub struct SLA {
    code: u32,
    execution_time: u32,
    f: u8,
}
impl SLA {
    pub fn new() -> SLA {
        SLA {
            code: 6,
            execution_time: 2,
            f: 0,
        }
    }
}
impl Operation for SLA {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let mut shift_left = |value: u32, times: u32| value << 6 * times;
        let result = shift(
            args.reg.get_a(),
            args.instruction.get_address().abs() as u32,
            &mut shift_left,
        );

        args.reg.set_a(result);

        OperationResult::from_args(self.execution_time, args)
    }
}

pub struct SRA {
    code: u32,
    execution_time: u32,
    f: u8,
}
impl SRA {
    pub fn new() -> SRA {
        SRA {
            code: 6,
            execution_time: 2,
            f: 1,
        }
    }
}
impl Operation for SRA {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let mut shift_left = |value: u32, times: u32| value >> (6 * times);
        let result = shift(
            args.reg.get_a(),
            args.instruction.get_address().abs() as u32,
            &mut shift_left,
        );

        args.reg.set_a(result);

        OperationResult::from_args(self.execution_time, args)
    }
}

pub struct SLAX {
    code: u32,
    execution_time: u32,
    f: u8,
}
impl SLAX {
    pub fn new() -> SLAX {
        SLAX {
            code: 6,
            execution_time: 2,
            f: 2,
        }
    }
}
impl Operation for SLAX {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let mut shift_left = |value: u64, times: u32| value << 6 * times;

        let (ra, rx) = shift_ax(
            args.reg.get_a(),
            args.reg.get_x(),
            args.instruction.get_address().abs() as u32,
            &mut shift_left,
        );

        args.reg.set_a(ra);
        args.reg.set_x(rx);

        OperationResult::from_args(self.execution_time, args)
    }
}

pub struct SRAX {
    code: u32,
    execution_time: u32,
    f: u8,
}
impl SRAX {
    pub fn new() -> SRAX {
        SRAX {
            code: 6,
            execution_time: 2,
            f: 3,
        }
    }
}
impl Operation for SRAX {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let mut shift_left = |value: u64, times: u32| value >> 6 * times;

        let (ra, rx) = shift_ax(
            args.reg.get_a(),
            args.reg.get_x(),
            args.instruction.get_address().abs() as u32,
            &mut shift_left,
        );

        args.reg.set_a(ra);
        args.reg.set_x(rx);

        OperationResult::from_args(self.execution_time, args)
    }
}

pub struct SLC {
    code: u32,
    execution_time: u32,
    f: u8,
}
impl SLC {
    pub fn new() -> SLC {
        SLC {
            code: 6,
            execution_time: 2,
            f: 4,
        }
    }
}
impl Operation for SLC {
    fn execute(&self, args: OperationArgs) -> OperationResult {

        let mut circularly_left = |value: u64, shift_bytes: u32| {
            let mut tmp = value;
            for i in 0..shift_bytes {
                let mut byte_1 = tmp & BYTE_1_FROM_10;
                byte_1 >>= 6 * 9;
                tmp <<= 6;
                tmp |= byte_1;
            }
            tmp
        };

        let (ra, rx) = rotate_ax(
            args.reg.get_a(),
            args.reg.get_x(),
            args.instruction.get_address().abs() as u32,
            &mut circularly_left 
        );

        args.reg.set_a(ra);
        args.reg.set_x(rx);

        OperationResult::from_args(self.execution_time, args)
    }
}

pub struct SRC {
    code: u32,
    execution_time: u32,
    f: u8,
}
impl SRC {
    pub fn new() -> SRC {
        SRC {
            code: 6,
            execution_time: 2,
            f: 5,
        }
    }
}
impl Operation for SRC {
    fn execute(&self, args: OperationArgs) -> OperationResult {

        let mut circularly_right = |value: u64, shift_bytes: u32| {
            let mut tmp = value;
            for i in 0..shift_bytes {
                let mut byte_10 = tmp & BYTE_10_FROM_10;
                byte_10 <<= 6 * 9;
                tmp >>= 6;
                tmp |= byte_10;
            }
            tmp
        };

        let (ra, rx) = rotate_ax(
            args.reg.get_a(),
            args.reg.get_x(),
            args.instruction.get_address().abs() as u32,
            &mut circularly_right 
        );

        args.reg.set_a(ra);
        args.reg.set_x(rx);

        OperationResult::from_args(self.execution_time, args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // println!("{:#034b}", result);
    #[test]
    fn sla() {
        let op = SLA::new();
        let mut r = Registers::new();
        let mut m = Memory::new();

        let word = Word::new(0b10_101111_110111_111011_111101_111110);
        r.set_a(word);

        let args = OperationArgs::new(
            1,
            Word::new_instruction(4, 0, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(0b10_111110_000000_000000_000000_000000, r.get_a().get());

        r.set_a(word);
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2, 0, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(0b10_111011_111101_111110_000000_000000, r.get_a().get());

        let word = Word::new(0b00_101111_110111_111011_111101_111110);
        r.set_a(word);
        let args = OperationArgs::new(
            1,
            Word::new_instruction(1, 0, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(0b00_110111_111011_111101_111110_000000, r.get_a().get());

        let word = Word::new(0b00_101111_110111_111011_111101_111110);
        r.set_a(word);
        let args = OperationArgs::new(
            1,
            Word::new_instruction(-1, 0, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(0b00_110111_111011_111101_111110_000000, r.get_a().get());

        r.set_a(word);
        let args = OperationArgs::new(
            1,
            Word::new_instruction(0, 0, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(0b00_101111_110111_111011_111101_111110, r.get_a().get());
    }

    #[test]
    fn sra() {
        let op = SRA::new();
        let mut r = Registers::new();
        let mut m = Memory::new();

        let word = Word::new(0b10_101111_110111_111011_111101_111110);
        r.set_a(word);

        let args = OperationArgs::new(
            1,
            Word::new_instruction(4, 0, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(0b10_000000_000000_000000_000000_101111, r.get_a().get());
        r.set_a(word);
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2, 0, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(0b10_000000_000000_101111_110111_111011, r.get_a().get());

        let word = Word::new(0b00_101111_110111_111011_111101_111110);
        r.set_a(word);
        let args = OperationArgs::new(
            1,
            Word::new_instruction(1, 0, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(0b00_000000_101111_110111_111011_111101, r.get_a().get());

        let word = Word::new(0b00_101111_110111_111011_111101_111110);
        r.set_a(word);
        let args = OperationArgs::new(
            1,
            Word::new_instruction(-1, 0, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(0b00_000000_101111_110111_111011_111101, r.get_a().get());

        r.set_a(word);
        let args = OperationArgs::new(
            1,
            Word::new_instruction(0, 0, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(0b00_101111_110111_111011_111101_111110, r.get_a().get());
    }

    #[test]
    fn slax() {
        let op = SLAX::new();
        let mut r = Registers::new();
        let mut m = Memory::new();

        let ra = Word::new(0b10_101111_110111_111011_111101_111110);
        let rx = Word::new(0b10_101111_110111_111011_111101_111110);

        r.set_a(ra);
        r.set_x(rx);
        let args = OperationArgs::new(
            1,
            Word::new_instruction(4, 0, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(0b10_111110_101111_110111_111011_111101, r.get_a().get());
        assert_eq!(0b10_111110_000000_000000_000000_000000, r.get_x().get());

        r.set_a(ra);
        r.set_x(rx);
        let args = OperationArgs::new(
            1,
            Word::new_instruction(9, 0, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(0b10_111110_000000_000000_000000_000000, r.get_a().get());
        assert_eq!(0b10_000000_000000_000000_000000_000000, r.get_x().get());
    }

    #[test]
    fn srax() {
        let op = SRAX::new();
        let mut r = Registers::new();
        let mut m = Memory::new();

        let ra = Word::new(0b10_101111_110111_111011_111101_111110);
        let rx = Word::new(0b00_101111_110111_111011_111101_111110);

        r.set_a(ra);
        r.set_x(rx);
        let args = OperationArgs::new(
            1,
            Word::new_instruction(4, 0, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(0b10_000000_000000_000000_000000_101111, r.get_a().get());
        assert_eq!(0b00_110111_111011_111101_111110_101111, r.get_x().get());

        r.set_a(ra);
        r.set_x(rx);
        let args = OperationArgs::new(
            1,
            Word::new_instruction(9, 0, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(0b10_000000_000000_000000_000000_000000, r.get_a().get());
        assert_eq!(0b00_000000_000000_000000_000000_101111, r.get_x().get());
    }

    #[test]
    fn slc() {
        let op = SLC::new();
        let mut r = Registers::new();
        let mut m = Memory::new();

        let ra = Word::new(0b10_101111_110111_111011_111101_111110);
        let rx = Word::new(0b00_111101_111011_110111_101111_011111);

        r.set_a(ra);
        r.set_x(rx);
        let args = OperationArgs::new(
            1,
            Word::new_instruction(1, 0, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        op.execute(args);
        // println!("{:#034b}", r.get_a().get());
        // println!("{:#034b}", r.get_x().get());
        assert_eq!(0b10_110111_111011_111101_111110_111101, r.get_a().get());
        assert_eq!(0b00_111011_110111_101111_011111_101111, r.get_x().get());

        r.set_a(ra);
        r.set_x(rx);
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2, 0, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(0b10_111011_111101_111110_111101_111011, r.get_a().get());
        assert_eq!(0b00_110111_101111_011111_101111_110111, r.get_x().get());

        r.set_a(ra);
        r.set_x(rx);
        let args = OperationArgs::new(
            1,
            Word::new_instruction(9, 0, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(0b10_011111_101111_110111_111011_111101, r.get_a().get());
        assert_eq!(0b00_111110_111101_111011_110111_101111, r.get_x().get());
    }

    #[test]
    fn src() {
        let op = SRC::new();
        let mut r = Registers::new();
        let mut m = Memory::new();

        let ra = Word::new(0b10_101111_110111_111011_111101_111110);
        let rx = Word::new(0b00_111101_111011_110111_101111_011111);

        r.set_a(ra);
        r.set_x(rx);
        let args = OperationArgs::new(
            1,
            Word::new_instruction(1, 0, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(0b10_011111_101111_110111_111011_111101, r.get_a().get());
        assert_eq!(0b00_111110_111101_111011_110111_101111, r.get_x().get());

        r.set_a(ra);
        r.set_x(rx);
        let args = OperationArgs::new(
            1,
            Word::new_instruction(2, 0, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(0b10_101111_011111_101111_110111_111011, r.get_a().get());
        assert_eq!(0b00_111101_111110_111101_111011_110111, r.get_x().get());

        r.set_a(ra);
        r.set_x(rx);
        let args = OperationArgs::new(
            1,
            Word::new_instruction(9, 0, WordAccess::new(0, 5), 56),
            &mut m,
            &mut r,
        );
        op.execute(args);
        // println!("{:#034b}", r.get_a().get());
        // println!("{:#034b}", r.get_x().get());
        assert_eq!(0b10_110111_111011_111101_111110_111101, r.get_a().get());
        assert_eq!(0b00_111011_110111_101111_011111_101111, r.get_x().get());
    }
}
