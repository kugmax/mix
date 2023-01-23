use crate::memory::short_word::ShortWord;
use crate::memory::word::Word;
use crate::memory::word::MAX_5_BYTES;
use crate::memory::word_access::WordAccess;
use crate::memory::Bytes;
use crate::memory::Instruction;
use crate::memory::Memory;
use crate::operations::get_memory_cell;
use crate::operations::*;
use crate::registers::Registers;

fn sum(
    instruction: impl Instruction,
    sum: &mut dyn Fn(i32, i32) -> i32,
    mem: &Memory,
    reg: &mut Registers,
) {
    let f = instruction.get_f();
    let mem_cell = get_memory_cell(instruction, mem, reg);

    let value: i32 = Word::new(mem_cell.get_by_access(f)).get_signed_value();
    let result: i32 = sum(reg.get_a().get_signed_value(), value);

    if result == 0 {
        let mut result = Word::new(0);
        result.set_sign(reg.get_a().get_sign());
        reg.set_a(result);
        return;
    }

    if result >= -MAX_5_BYTES && result <= MAX_5_BYTES {
        reg.set_a(Word::new_from_signed(result));
        return;
    }

    reg.set_overflow(true);
    reg.set_a(Word::new(0)); //TODO: the behaviour have to be different
}

pub struct ADD {
    code: u32,
    execution_time: u32,
}

impl ADD {
    pub fn new() -> ADD {
        ADD {
            code: 1,
            execution_time: 2,
        }
    }
}

impl Operation for ADD {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let mut add = |v1, v2| v1 + v2;
        sum(args.instruction, &mut add, args.mem, args.reg);

        OperationResult::from_args(self.execution_time, args)
    }
}

pub struct SUB {
    code: u32,
    execution_time: u32,
}

impl SUB {
    pub fn new() -> SUB {
        SUB {
            code: 2,
            execution_time: 2,
        }
    }
}

impl Operation for SUB {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let mut sub = |v1, v2| v1 - v2;
        sum(args.instruction, &mut sub, args.mem, args.reg);

        OperationResult::from_args(self.execution_time, args)
    }
}

pub struct MUL {
    code: u32,
    execution_time: u32,
}

impl MUL {
    pub fn new() -> MUL {
        MUL {
            code: 3,
            execution_time: 10,
        }
    }
}

impl Operation for MUL {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let f = args.instruction.get_f();
        let mem_cell = get_memory_cell(args.instruction, args.mem, args.reg);

        let value: i64 = Word::new(mem_cell.get_by_access(f)).get_signed_value() as i64;
        let result: i64 = args.reg.get_a().get_signed_value() as i64 * value;

        let (a, x) = Word::split(result);
        args.reg.set_a(a);
        args.reg.set_x(x);

        OperationResult::from_args(self.execution_time, args)
    }
}

pub struct DIV {
    code: u32,
    execution_time: u32,
}

impl DIV {
    pub fn new() -> DIV {
        DIV {
            code: 4,
            execution_time: 12,
        }
    }
}

impl Operation for DIV {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let f = args.instruction.get_f();
        let mem_cell = get_memory_cell(args.instruction, args.mem, args.reg);

        let value = Word::new(mem_cell.get_by_access(f)).get_signed_value();

        if value == 0 || args.reg.get_a().get_signed_value().abs() >= value.abs() {
            args.reg.set_overflow(true);
            args.reg.set_a(Word::new(0));
            args.reg.set_x(Word::new(0));

            return OperationResult::from_args(self.execution_time, args);
        }

        let value: i64 = value as i64;

        let old_ra_sign = args.reg.get_a().get_sign();
        let r_ax: i64 = Word::unite(args.reg.get_a().get(), args.reg.get_x().get());

        let quotient: i64 = r_ax / value;
        let reminder: i64 = r_ax % value;

        let quotient = Word::new_from_signed(quotient as i32);
        let mut reminder = Word::new_from_signed(reminder as i32);
        reminder.set_sign(old_ra_sign);

        args.reg.set_a(quotient);
        args.reg.set_x(reminder);

        OperationResult::from_args(self.execution_time, args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let operation = ADD::new();

        let mut m = Memory::new();

        let mem_value = Word::new(1_001);
        m.set(2_000, mem_value.get());

        let mem_value = Word::new_from_signed(-1_001);
        m.set(2_001, mem_value.get());

        let mem_value = Word::new_from_signed(MAX_5_BYTES);
        m.set(2_002, mem_value.get());

        let mem_value = Word::new_from_signed(-MAX_5_BYTES);
        m.set(2_003, mem_value.get());

        let mut r = Registers::new();

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 1),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.get_a(), Word::new(1_001));
        assert_eq!(r.is_overflow(), false);

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_001, 0, WordAccess::new(0, 5), 1),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.get_a(), Word::new(0));
        assert_eq!(r.is_overflow(), false);

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_002, 0, WordAccess::new(0, 5), 1),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.get_a(), Word::new_from_signed(MAX_5_BYTES));
        assert_eq!(r.is_overflow(), false);

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_003, 0, WordAccess::new(0, 5), 1),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.get_a(), Word::new_from_signed(0));
        assert_eq!(r.is_overflow(), false);
    }

    #[test]
    fn add_result_is_0() {
        let operation = ADD::new();

        let mut m = Memory::new();

        let mem_value = Word::new(1);
        m.set(2_000, mem_value.get());

        let mem_value = Word::new_from_signed(-1);
        m.set(2_001, mem_value.get());

        let mut r = Registers::new();

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_001, 0, WordAccess::new(0, 5), 1),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.is_overflow(), false);
        assert_eq!(r.get_a(), Word::new_from_signed(-1));

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 1),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.is_overflow(), false);
        assert_eq!(r.get_a(), Word::new_by_bytes(-1, &[0, 0, 0, 0, 0]));

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 1),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.is_overflow(), false);
        assert_eq!(r.get_a(), Word::new_from_signed(1));

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_001, 0, WordAccess::new(0, 5), 1),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.is_overflow(), false);
        assert_eq!(r.get_a(), Word::new_by_bytes(0, &[0, 0, 0, 0, 0]));
    }

    #[test]
    fn add_overflow() {
        let operation = ADD::new();

        let mut m = Memory::new();

        let mem_value = Word::new(2);
        m.set(2_000, mem_value.get());

        let mem_value = Word::new_from_signed(-2);
        m.set(2_001, mem_value.get());

        let mem_value = Word::new_from_signed(MAX_5_BYTES);
        m.set(2_002, mem_value.get());

        let mem_value = Word::new_from_signed(-MAX_5_BYTES);
        m.set(2_003, mem_value.get());

        let mut r = Registers::new();

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_002, 0, WordAccess::new(0, 5), 1),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.is_overflow(), false);
        assert_eq!(r.get_a(), Word::new_from_signed(MAX_5_BYTES));

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 1),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.is_overflow(), true);
        assert_eq!(r.get_a(), Word::new(0));

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_003, 0, WordAccess::new(0, 5), 1),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.is_overflow(), true);
        assert_eq!(r.get_a(), Word::new_from_signed(-MAX_5_BYTES));

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_001, 0, WordAccess::new(0, 5), 1),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.is_overflow(), true);
        assert_eq!(r.get_a(), Word::new(0));
    }

    #[test]
    fn sub() {
        let operation = SUB::new();

        let mut m = Memory::new();

        let mem_value = Word::new(1_001);
        m.set(2_000, mem_value.get());

        let mem_value = Word::new_from_signed(-1_001);
        m.set(2_001, mem_value.get());

        let mem_value = Word::new_from_signed(MAX_5_BYTES);
        m.set(2_002, mem_value.get());

        let mem_value = Word::new_from_signed(-MAX_5_BYTES);
        m.set(2_003, mem_value.get());

        let mut r = Registers::new();

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_001, 0, WordAccess::new(0, 5), 1),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.get_a(), Word::new(1_001));
        assert_eq!(r.is_overflow(), false);

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 1),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.get_a(), Word::new(0));
        assert_eq!(r.is_overflow(), false);

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_003, 0, WordAccess::new(0, 5), 1),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.get_a(), Word::new_from_signed(MAX_5_BYTES));
        assert_eq!(r.is_overflow(), false);

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_002, 0, WordAccess::new(0, 5), 1),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(r.get_a(), Word::new_from_signed(0));
        assert_eq!(r.is_overflow(), false);
    }

    #[test]
    fn mul() {
        let operation = MUL::new();

        let mut m = Memory::new();
        let mut r = Registers::new();

        m.set(3_000, Word::new_from_signed(MAX_5_BYTES).get());
        r.set_a(Word::new_from_signed(MAX_5_BYTES));

        let args = OperationArgs::new(
            1,
            Word::new_instruction(3_000, 0, WordAccess::new(0, 5), 3),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(0b00_111111_111111_111111_111111_111110, r.get_a().get());
        assert_eq!(0b00_000000_000000_000000_000000_000001, r.get_x().get());

        r.set_a(Word::new_from_signed(-MAX_5_BYTES));
        let args = OperationArgs::new(
            1,
            Word::new_instruction(3_000, 0, WordAccess::new(0, 5), 3),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(0b10_111111_111111_111111_111111_111110, r.get_a().get());
        assert_eq!(0b10_000000_000000_000000_000000_000001, r.get_x().get());

        // println!("rA {:#034b}", r.get_a().get());
        // println!("rX {:#034b}", r.get_x().get());
    }

    #[test]
    fn div() {
        let operation = DIV::new();

        let mut m = Memory::new();
        let mut r = Registers::new();

        m.set(3_000, Word::new_from_signed(2).get());
        r.set_a(Word::new(0));
        r.set_x(Word::new(10));

        let args = OperationArgs::new(
            1,
            Word::new_instruction(3_000, 0, WordAccess::new(0, 5), 4),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(5, r.get_a().get());
        assert_eq!(0, r.get_x().get());

        m.set(3_000, Word::new_from_signed(-2).get());
        r.set_a(Word::new(0));
        r.set_x(Word::new(10));

        let args = OperationArgs::new(
            1,
            Word::new_instruction(3_000, 0, WordAccess::new(0, 5), 4),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(0b10_000000_000000_000000_000000_000101, r.get_a().get());
        assert_eq!(0b00_000000_000000_000000_000000_000000, r.get_x().get());

        m.set(3_000, Word::new_from_signed(-2).get());
        r.set_a(Word::new_from_signed(-1));
        r.set_x(Word::new(11));

        let args = OperationArgs::new(
            1,
            Word::new_instruction(3_000, 0, WordAccess::new(0, 5), 4),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(0b00_100000_000000_000000_000000_000101, r.get_a().get());
        assert_eq!(0b10_000000_000000_000000_000000_000001, r.get_x().get());

        m.set(3_000, Word::new_from_signed(0).get());
        r.set_a(Word::new_from_signed(-1));
        r.set_x(Word::new(11));

        let args = OperationArgs::new(
            1,
            Word::new_instruction(3_000, 0, WordAccess::new(0, 5), 4),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(true, r.is_overflow());
        assert_eq!(0, r.get_a().get());
        assert_eq!(0, r.get_x().get());

        m.set(3_000, Word::new_from_signed(-1).get());
        r.set_a(Word::new_from_signed(-2));
        r.set_x(Word::new(0));

        let args = OperationArgs::new(
            1,
            Word::new_instruction(3_000, 0, WordAccess::new(0, 5), 4),
            &mut m,
            &mut r,
        );
        operation.execute(args);
        assert_eq!(true, r.is_overflow());
        assert_eq!(0, r.get_a().get());
        assert_eq!(0, r.get_x().get());
    }

    #[test]
    fn arithmetic_instructions_1() {
        let op = ADD::new();

        let mut m = Memory::new();
        let mut r = Registers::new();

        let mut ra = Word::new(0);
        ra.set_bytes(&[1, 2], 1234);
        ra.set_bytes(&[3], 1);
        ra.set_bytes(&[4, 5], 150);

        r.set_a(ra);
        r.set_x(Word::new(0));

        let mut cell = Word::new(0);
        cell.set_bytes(&[1, 2], 100);
        cell.set_bytes(&[3], 5);
        cell.set_bytes(&[4, 5], 50);

        m.set(1_000, cell.get());

        let args = OperationArgs::new(
            1,
            Word::new_instruction(1_000, 0, WordAccess::new(0, 5), 0),
            &mut m,
            &mut r,
        );
        op.execute(args);

        let mut ra = r.get_a();
        assert_eq!(1334, ra.get_bytes(&[1, 2]));
        assert_eq!(6, ra.get_bytes(&[3]));
        assert_eq!(200, ra.get_bytes(&[4, 5]));
    }

    #[test]
    fn arithmetic_instructions_2() {
        let op = SUB::new();
        let mut m = Memory::new();
        let mut r = Registers::new();

        let mut ra = Word::new(0);
        ra.set_bytes(&[1, 2], 1234);
        ra.set_bytes(&[3], 0);
        ra.set_bytes(&[4], 0);
        ra.set_bytes(&[5], 9);
        ra.set_sign(-1);

        r.set_a(ra);
        r.set_x(Word::new(0));

        let mut cell = Word::new(0);
        cell.set_bytes(&[1, 2], 2000);
        cell.set_bytes(&[3, 4], 150);
        cell.set_bytes(&[5], 0);
        cell.set_sign(-1);

        m.set(1_000, cell.get());

        let args = OperationArgs::new(
            1,
            Word::new_instruction(1_000, 0, WordAccess::new(0, 5), 0),
            &mut m,
            &mut r,
        );
        op.execute(args);

        let mut ra = r.get_a();
        assert_eq!(0, ra.get_sign());
        assert_eq!(766, ra.get_bytes(&[1, 2]));
        assert_eq!(149, ra.get_bytes(&[3, 4]));
        // assert_eq!(0, ra.get_bytes(&[5]));
    }

    #[test]
    fn arithmetic_instructions_3() {
        let op = MUL::new();

        let mut m = Memory::new();
        let mut r = Registers::new();

        let ra = Word::new_by_bytes(0, &[1, 1, 1, 1, 1]);

        r.set_a(ra);
        r.set_x(Word::new(0));

        let cell = Word::new_by_bytes(0, &[1, 1, 1, 1, 1]);

        m.set(1_000, cell.get());

        let args = OperationArgs::new(
            1,
            Word::new_instruction(1_000, 0, WordAccess::new(0, 5), 0),
            &mut m,
            &mut r,
        );
        op.execute(args);

        let ra = r.get_a();
        assert_eq!(0, ra.get_sign());
        assert_eq!(0, ra.get_byte(1));
        assert_eq!(1, ra.get_byte(2));
        assert_eq!(2, ra.get_byte(3));
        assert_eq!(3, ra.get_byte(4));
        assert_eq!(4, ra.get_byte(5));

        let rx = r.get_x();
        assert_eq!(0, rx.get_sign());
        assert_eq!(5, rx.get_byte(1));
        assert_eq!(4, rx.get_byte(2));
        assert_eq!(3, rx.get_byte(3));
        assert_eq!(2, rx.get_byte(4));
        assert_eq!(1, rx.get_byte(5));
    }

    #[test]
    fn arithmetic_instructions_4() {
        let op = MUL::new();

        let mut m = Memory::new();
        let mut r = Registers::new();

        let ra = Word::new_from_signed(-112);

        r.set_a(ra);
        r.set_x(Word::new(0));

        let cell = Word::new_by_bytes(0, &[2, 1, 1, 1, 1]);

        m.set(1_000, cell.get());

        let args = OperationArgs::new(
            1,
            Word::new_instruction(1_000, 0, WordAccess::new(1, 1), 0),
            &mut m,
            &mut r,
        );
        op.execute(args);

        let ra = r.get_a();
        assert_eq!(-1, ra.get_sign());
        assert_eq!(0, ra.get_signed_value());

        let rx = r.get_x();
        assert_eq!(-224, rx.get_signed_value());
    }

    #[test]
    fn arithmetic_instructions_5() {
        let op = MUL::new();

        let mut m = Memory::new();
        let mut r = Registers::new();

        let mut ra = Word::new(0);
        ra.set_bytes(&[1], 50);
        ra.set_bytes(&[2], 0);
        ra.set_bytes(&[3, 4], 112);
        ra.set_bytes(&[5], 4);
        ra.set_sign(-1);

        r.set_a(ra);
        r.set_x(Word::new(0));

        let cell = Word::new_by_bytes(-1, &[2, 0, 0, 0, 0]);

        m.set(1_000, cell.get());

        let args = OperationArgs::new(
            1,
            Word::new_instruction(1_000, 0, WordAccess::new(0, 5), 0),
            &mut m,
            &mut r,
        );
        op.execute(args);

        let mut ra = r.get_a();
        assert_eq!(0, ra.get_sign());
        assert_eq!(100, ra.get_bytes(&[1, 2]));
        assert_eq!(0, ra.get_bytes(&[3]));
        assert_eq!(224, ra.get_bytes(&[4, 5]));

        let rx = r.get_x();
        assert_eq!(0, rx.get_sign());
        assert_eq!(8, rx.get_byte(1));
        assert_eq!(0, rx.get_byte(2));
        assert_eq!(0, rx.get_byte(3));
        assert_eq!(0, rx.get_byte(4));
        assert_eq!(0, rx.get_byte(5));
    }

    #[test]
    fn arithmetic_instructions_6() {
        let op = DIV::new();

        let mut m = Memory::new();
        let mut r = Registers::new();

        let mut ra = Word::new(0);

        r.set_a(ra);
        r.set_x(Word::new(17));

        let cell = Word::new(3);

        m.set(1_000, cell.get());

        let args = OperationArgs::new(
            1,
            Word::new_instruction(1_000, 0, WordAccess::new(0, 5), 0),
            &mut m,
            &mut r,
        );
        op.execute(args);

        let ra = r.get_a();
        assert_eq!(0, ra.get_sign());
        assert_eq!(5, ra.get());

        let rx = r.get_x();
        assert_eq!(0, rx.get_sign());
        assert_eq!(2, rx.get());
    }

    #[test]
    fn arithmetic_instructions_7() {
        let op = DIV::new();
        let mut m = Memory::new();
        let mut r = Registers::new();

        let mut ra = Word::new(0);
        ra.set_sign(-1);

        r.set_a(ra);

        let mut rx = Word::new(0);
        rx.set_bytes(&[1, 2], 1235);
        rx.set_bytes(&[3], 0);
        rx.set_bytes(&[4], 3);
        rx.set_bytes(&[5], 1);

        r.set_x(rx);

        let cell = Word::new_by_bytes(-1, &[0, 0, 0, 2, 0]);

        m.set(1_000, cell.get());

        let args = OperationArgs::new(
            1,
            Word::new_instruction(1_000, 0, WordAccess::new(0, 5), 0),
            &mut m,
            &mut r,
        );
        op.execute(args);

        let ra = r.get_a();
        assert_eq!(0, ra.get_sign());
        assert_eq!(0, ra.get_bytes(&[1]));
        assert_eq!(617, ra.get_bytes(&[2, 3]));

        let rx = r.get_x();
        assert_eq!(-1, rx.get_sign());
        assert_eq!(1, rx.get_byte(5));
    }
}
