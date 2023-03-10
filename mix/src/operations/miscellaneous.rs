use crate::memory::short_word::ShortWord;
use crate::memory::word::Word;
use crate::memory::word::ABS;
use crate::memory::word::BYTE_10_FROM_10;
use crate::memory::word::BYTE_1_FROM_10;
use crate::memory::Bytes;
use crate::memory::Instruction;
use crate::operations::*;

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
    instruction: Word,
}
impl SLA {
    pub fn new(instruction: Word) -> SLA {
        SLA {
            code: 6,
            execution_time: 2,
            f: 0,
            instruction: instruction,
        }
    }
}
impl Operation for SLA {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let mut shift_left = |value: u32, times: u32| value << 6 * times;
        let result = shift(
            args.reg.get_a(),
            self.instruction.get_address().abs() as u32,
            &mut shift_left,
        );

        args.reg.set_a(result);

        OperationResult::from_args(self.execution_time, args)
    }
    fn get_name(&self) -> String {
        String::from("SLA")
    }
}

pub struct SRA {
    code: u32,
    execution_time: u32,
    f: u8,
    instruction: Word,
}
impl SRA {
    pub fn new(instruction: Word) -> SRA {
        SRA {
            code: 6,
            execution_time: 2,
            f: 1,
            instruction: instruction,
        }
    }
}
impl Operation for SRA {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let mut shift_left = |value: u32, times: u32| value >> (6 * times);
        let result = shift(
            args.reg.get_a(),
            self.instruction.get_address().abs() as u32,
            &mut shift_left,
        );

        args.reg.set_a(result);

        OperationResult::from_args(self.execution_time, args)
    }
    fn get_name(&self) -> String {
        String::from("SRA")
    }
}

pub struct SLAX {
    code: u32,
    execution_time: u32,
    f: u8,
    instruction: Word,
}
impl SLAX {
    pub fn new(instruction: Word) -> SLAX {
        SLAX {
            code: 6,
            execution_time: 2,
            f: 2,
            instruction: instruction,
        }
    }
}
impl Operation for SLAX {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let mut shift_left = |value: u64, times: u32| value << 6 * times;

        let (ra, rx) = shift_ax(
            args.reg.get_a(),
            args.reg.get_x(),
            self.instruction.get_address().abs() as u32,
            &mut shift_left,
        );

        args.reg.set_a(ra);
        args.reg.set_x(rx);

        OperationResult::from_args(self.execution_time, args)
    }
    fn get_name(&self) -> String {
        String::from("SLAX")
    }
}

pub struct SRAX {
    code: u32,
    execution_time: u32,
    f: u8,
    instruction: Word,
}
impl SRAX {
    pub fn new(instruction: Word) -> SRAX {
        SRAX {
            code: 6,
            execution_time: 2,
            f: 3,
            instruction: instruction,
        }
    }
}
impl Operation for SRAX {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let mut shift_left = |value: u64, times: u32| value >> 6 * times;

        let (ra, rx) = shift_ax(
            args.reg.get_a(),
            args.reg.get_x(),
            self.instruction.get_address().abs() as u32,
            &mut shift_left,
        );

        args.reg.set_a(ra);
        args.reg.set_x(rx);

        OperationResult::from_args(self.execution_time, args)
    }
    fn get_name(&self) -> String {
        String::from("SRAX")
    }
}

pub struct SLC {
    code: u32,
    execution_time: u32,
    f: u8,
    instruction: Word,
}
impl SLC {
    pub fn new(instruction: Word) -> SLC {
        SLC {
            code: 6,
            execution_time: 2,
            f: 4,
            instruction: instruction,
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
            self.instruction.get_address().abs() as u32,
            &mut circularly_left,
        );

        args.reg.set_a(ra);
        args.reg.set_x(rx);

        OperationResult::from_args(self.execution_time, args)
    }
    fn get_name(&self) -> String {
        String::from("SLC")
    }
}

pub struct SRC {
    code: u32,
    execution_time: u32,
    f: u8,
    instruction: Word,
}
impl SRC {
    pub fn new(instruction: Word) -> SRC {
        SRC {
            code: 6,
            execution_time: 2,
            f: 5,
            instruction: instruction,
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
            self.instruction.get_address().abs() as u32,
            &mut circularly_right,
        );

        args.reg.set_a(ra);
        args.reg.set_x(rx);

        OperationResult::from_args(self.execution_time, args)
    }
    fn get_name(&self) -> String {
        String::from("SRC")
    }
}

pub struct MOVE {
    code: u32,
    execution_time: u32,
    instruction: Word,
}
impl MOVE {
    pub fn new(instruction: Word) -> MOVE {
        MOVE {
            code: 7,
            execution_time: 1, // + 2F
            instruction: instruction,
        }
    }
}
impl Operation for MOVE {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let n_words = self.instruction.get_f().spec as u32;
        if n_words == 0 {
            return OperationResult::from_args(self.execution_time, args);
        }

        for i in 0..n_words {
            let from = (self.instruction.get_address() as u32 + i) as usize;
            let to = (args.reg.get_i(1).get() + i) as usize;

            args.mem.set(to, args.mem.get(from).get());
        }

        OperationResult::from_args(self.execution_time + 2 * n_words, args)
    }
    fn get_name(&self) -> String {
        String::from("MOVE")
    }
}

pub struct NOP {
    code: u32,
    execution_time: u32,
    instruction: Word,
}
impl NOP {
    pub fn new(instruction: Word) -> NOP {
        NOP {
            code: 0,
            execution_time: 1,
            instruction: instruction,
        }
    }
}
impl Operation for NOP {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        OperationResult::from_args(self.execution_time, args)
    }
    fn get_name(&self) -> String {
        String::from("NOP")
    }
}

pub struct HLT {
    code: u32,
    execution_time: u32,
    f: u8,
    instruction: Word,
}
impl HLT {
    pub fn new(instruction: Word) -> HLT {
        HLT {
            code: 5,
            execution_time: 10,
            f: 2,
            instruction: instruction,
        }
    }
}
impl Operation for HLT {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        OperationResult::from_args(self.execution_time, args)
    }
    fn get_name(&self) -> String {
        String::from("HLT")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // println!("{:#034b}", result);
    #[test]
    fn sla() {
        let mut r = Registers::new();
        let mut m = Memory::new();

        let word = Word::new(0b10_101111_110111_111011_111101_111110);
        r.set_a(word);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let op = SLA::new(Word::new_instruction(4, 0, WordAccess::new(0, 5), 56));
        op.execute(args);
        assert_eq!(0b10_111110_000000_000000_000000_000000, r.get_a().get());

        r.set_a(word);
        let args = OperationArgs::new(1, &mut m, &mut r);
        let op = SLA::new(Word::new_instruction(2, 0, WordAccess::new(0, 5), 56));
        op.execute(args);
        assert_eq!(0b10_111011_111101_111110_000000_000000, r.get_a().get());

        let word = Word::new(0b00_101111_110111_111011_111101_111110);
        r.set_a(word);
        let args = OperationArgs::new(1, &mut m, &mut r);
        let op = SLA::new(Word::new_instruction(1, 0, WordAccess::new(0, 5), 56));
        op.execute(args);
        assert_eq!(0b00_110111_111011_111101_111110_000000, r.get_a().get());

        let word = Word::new(0b00_101111_110111_111011_111101_111110);
        r.set_a(word);
        let args = OperationArgs::new(1, &mut m, &mut r);
        let op = SLA::new(Word::new_instruction(-1, 0, WordAccess::new(0, 5), 56));
        op.execute(args);
        assert_eq!(0b00_110111_111011_111101_111110_000000, r.get_a().get());

        r.set_a(word);
        let args = OperationArgs::new(1, &mut m, &mut r);
        let op = SLA::new(Word::new_instruction(0, 0, WordAccess::new(0, 5), 56));
        op.execute(args);
        assert_eq!(0b00_101111_110111_111011_111101_111110, r.get_a().get());
    }

    #[test]
    fn sra() {
        let mut r = Registers::new();
        let mut m = Memory::new();

        let word = Word::new(0b10_101111_110111_111011_111101_111110);
        r.set_a(word);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let op = SRA::new(Word::new_instruction(4, 0, WordAccess::new(0, 5), 56));
        op.execute(args);
        assert_eq!(0b10_000000_000000_000000_000000_101111, r.get_a().get());
        r.set_a(word);
        let args = OperationArgs::new(1, &mut m, &mut r);
        let op = SRA::new(Word::new_instruction(2, 0, WordAccess::new(0, 5), 56));
        op.execute(args);
        assert_eq!(0b10_000000_000000_101111_110111_111011, r.get_a().get());

        let word = Word::new(0b00_101111_110111_111011_111101_111110);
        r.set_a(word);
        let args = OperationArgs::new(1, &mut m, &mut r);
        let op = SRA::new(Word::new_instruction(1, 0, WordAccess::new(0, 5), 56));
        op.execute(args);
        assert_eq!(0b00_000000_101111_110111_111011_111101, r.get_a().get());

        let word = Word::new(0b00_101111_110111_111011_111101_111110);
        r.set_a(word);
        let args = OperationArgs::new(1, &mut m, &mut r);
        let op = SRA::new(Word::new_instruction(-1, 0, WordAccess::new(0, 5), 56));
        op.execute(args);
        assert_eq!(0b00_000000_101111_110111_111011_111101, r.get_a().get());

        r.set_a(word);
        let args = OperationArgs::new(1, &mut m, &mut r);
        let op = SRA::new(Word::new_instruction(0, 0, WordAccess::new(0, 5), 56));
        op.execute(args);
        assert_eq!(0b00_101111_110111_111011_111101_111110, r.get_a().get());
    }

    #[test]
    fn slax() {
        let mut r = Registers::new();
        let mut m = Memory::new();

        let ra = Word::new(0b10_101111_110111_111011_111101_111110);
        let rx = Word::new(0b10_101111_110111_111011_111101_111110);

        r.set_a(ra);
        r.set_x(rx);
        let args = OperationArgs::new(1, &mut m, &mut r);
        let op = SLAX::new(Word::new_instruction(4, 0, WordAccess::new(0, 5), 56));
        op.execute(args);
        assert_eq!(0b10_111110_101111_110111_111011_111101, r.get_a().get());
        assert_eq!(0b10_111110_000000_000000_000000_000000, r.get_x().get());

        r.set_a(ra);
        r.set_x(rx);
        let args = OperationArgs::new(1, &mut m, &mut r);
        let op = SLAX::new(Word::new_instruction(9, 0, WordAccess::new(0, 5), 56));
        op.execute(args);
        assert_eq!(0b10_111110_000000_000000_000000_000000, r.get_a().get());
        assert_eq!(0b10_000000_000000_000000_000000_000000, r.get_x().get());
    }

    #[test]
    fn srax() {
        let mut r = Registers::new();
        let mut m = Memory::new();

        let ra = Word::new(0b10_101111_110111_111011_111101_111110);
        let rx = Word::new(0b00_101111_110111_111011_111101_111110);

        r.set_a(ra);
        r.set_x(rx);
        let args = OperationArgs::new(1, &mut m, &mut r);
        let op = SRAX::new(Word::new_instruction(4, 0, WordAccess::new(0, 5), 56));
        op.execute(args);
        assert_eq!(0b10_000000_000000_000000_000000_101111, r.get_a().get());
        assert_eq!(0b00_110111_111011_111101_111110_101111, r.get_x().get());

        r.set_a(ra);
        r.set_x(rx);
        let args = OperationArgs::new(1, &mut m, &mut r);
        let op = SRAX::new(Word::new_instruction(9, 0, WordAccess::new(0, 5), 56));
        op.execute(args);
        assert_eq!(0b10_000000_000000_000000_000000_000000, r.get_a().get());
        assert_eq!(0b00_000000_000000_000000_000000_101111, r.get_x().get());
    }

    #[test]
    fn slc() {
        let mut r = Registers::new();
        let mut m = Memory::new();

        let ra = Word::new(0b10_101111_110111_111011_111101_111110);
        let rx = Word::new(0b00_111101_111011_110111_101111_011111);

        r.set_a(ra);
        r.set_x(rx);
        let args = OperationArgs::new(1, &mut m, &mut r);
        let op = SLC::new(Word::new_instruction(1, 0, WordAccess::new(0, 5), 56));
        op.execute(args);
        // println!("{:#034b}", r.get_a().get());
        // println!("{:#034b}", r.get_x().get());
        assert_eq!(0b10_110111_111011_111101_111110_111101, r.get_a().get());
        assert_eq!(0b00_111011_110111_101111_011111_101111, r.get_x().get());

        r.set_a(ra);
        r.set_x(rx);
        let args = OperationArgs::new(1, &mut m, &mut r);
        let op = SLC::new(Word::new_instruction(2, 0, WordAccess::new(0, 5), 56));
        op.execute(args);
        assert_eq!(0b10_111011_111101_111110_111101_111011, r.get_a().get());
        assert_eq!(0b00_110111_101111_011111_101111_110111, r.get_x().get());

        r.set_a(ra);
        r.set_x(rx);
        let args = OperationArgs::new(1, &mut m, &mut r);
        let op = SLC::new(Word::new_instruction(9, 0, WordAccess::new(0, 5), 56));
        op.execute(args);
        assert_eq!(0b10_011111_101111_110111_111011_111101, r.get_a().get());
        assert_eq!(0b00_111110_111101_111011_110111_101111, r.get_x().get());
    }

    #[test]
    fn src() {
        let mut r = Registers::new();
        let mut m = Memory::new();

        let ra = Word::new(0b10_101111_110111_111011_111101_111110);
        let rx = Word::new(0b00_111101_111011_110111_101111_011111);

        r.set_a(ra);
        r.set_x(rx);
        let args = OperationArgs::new(1, &mut m, &mut r);
        let op = SRC::new(Word::new_instruction(1, 0, WordAccess::new(0, 5), 56));
        op.execute(args);
        assert_eq!(0b10_011111_101111_110111_111011_111101, r.get_a().get());
        assert_eq!(0b00_111110_111101_111011_110111_101111, r.get_x().get());

        r.set_a(ra);
        r.set_x(rx);
        let args = OperationArgs::new(1, &mut m, &mut r);
        let op = SRC::new(Word::new_instruction(2, 0, WordAccess::new(0, 5), 56));
        op.execute(args);
        assert_eq!(0b10_101111_011111_101111_110111_111011, r.get_a().get());
        assert_eq!(0b00_111101_111110_111101_111011_110111, r.get_x().get());

        r.set_a(ra);
        r.set_x(rx);
        let args = OperationArgs::new(1, &mut m, &mut r);
        let op = SRC::new(Word::new_instruction(9, 0, WordAccess::new(0, 5), 56));
        op.execute(args);
        // println!("{:#034b}", r.get_a().get());
        // println!("{:#034b}", r.get_x().get());
        assert_eq!(0b10_110111_111011_111101_111110_111101, r.get_a().get());
        assert_eq!(0b00_111011_110111_101111_011111_101111, r.get_x().get());
    }

    #[test]
    fn several_shifts() {
        let mut r = Registers::new();
        let mut m = Memory::new();

        let ra = Word::new_by_bytes(0, &[1, 2, 3, 4, 5]);
        let rx = Word::new_by_bytes(-1, &[6, 7, 8, 9, 10]);

        r.set_a(ra);
        r.set_x(rx);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let srax = SRAX::new(Word::new_instruction(1, 0, WordAccess::new(0, 5), 56));
        srax.execute(args);
        assert_by_bytes(r.get_a(), 0, 0, 1, 2, 3, 4);
        assert_by_bytes(r.get_x(), -1, 5, 6, 7, 8, 9);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let sla = SLA::new(Word::new_instruction(2, 0, WordAccess::new(0, 5), 56));
        sla.execute(args);
        assert_by_bytes(r.get_a(), 0, 2, 3, 4, 0, 0);
        assert_by_bytes(r.get_x(), -1, 5, 6, 7, 8, 9);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let src = SRC::new(Word::new_instruction(4, 0, WordAccess::new(0, 5), 56));
        src.execute(args);
        assert_by_bytes(r.get_a(), 0, 6, 7, 8, 9, 2);
        assert_by_bytes(r.get_x(), -1, 3, 4, 0, 0, 5);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let sra = SRA::new(Word::new_instruction(2, 0, WordAccess::new(0, 5), 56));
        sra.execute(args);
        assert_by_bytes(r.get_a(), 0, 0, 0, 6, 7, 8);
        assert_by_bytes(r.get_x(), -1, 3, 4, 0, 0, 5);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let slc = SLC::new(Word::new_instruction(501, 0, WordAccess::new(0, 5), 56));
        slc.execute(args);
        assert_by_bytes(r.get_a(), 0, 0, 6, 7, 8, 3);
        assert_by_bytes(r.get_x(), -1, 4, 0, 0, 5, 0);
    }

    #[test]
    fn move_up() {
        let mut r = Registers::new();
        let mut m = Memory::new();

        m.set(999, 1);
        m.set(1_000, 2);
        m.set(1_001, 3);
        m.set(1_002, 4);

        r.set_i(1, ShortWord::new(999));
        let args = OperationArgs::new(1, &mut m, &mut r);
        let op = MOVE::new(Word::new_instruction(
            1_000,
            0,
            WordAccess::new_by_spec(3),
            56,
        ));
        op.execute(args);
        assert_eq!(m.get(999).get(), 2);
        assert_eq!(m.get(1_000).get(), 3);
        assert_eq!(m.get(1_001).get(), 4);
        assert_eq!(m.get(1_002).get(), 4);
    }

    #[test]
    fn move_down() {
        let mut r = Registers::new();
        let mut m = Memory::new();

        m.set(999, 1);
        m.set(1_000, 2);
        m.set(1_001, 3);
        m.set(1_002, 4);
        m.set(1_003, 5);

        r.set_i(1, ShortWord::new(1_001));
        let args = OperationArgs::new(1, &mut m, &mut r);
        let op = MOVE::new(Word::new_instruction(
            1_000,
            0,
            WordAccess::new_by_spec(3),
            56,
        ));
        op.execute(args);
        assert_eq!(m.get(999).get(), 1);
        assert_eq!(m.get(1_000).get(), 2);
        assert_eq!(m.get(1_001).get(), 2);
        assert_eq!(m.get(1_002).get(), 2);
        assert_eq!(m.get(1_003).get(), 2);
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
        assert_eq!(actual.get_byte(2), byte_2, "byte 1 is wrong");
        assert_eq!(actual.get_byte(3), byte_3, "byte 1 is wrong");
        assert_eq!(actual.get_byte(4), byte_4, "byte 1 is wrong");
        assert_eq!(actual.get_byte(5), byte_5, "byte 1 is wrong");
    }
}
