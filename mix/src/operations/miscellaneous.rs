use crate::memory::short_word::ShortWord;
use crate::memory::word::Word;
use crate::memory::word::ABS;
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
        println!("{:#034b}", r.get_a().get());
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
}
