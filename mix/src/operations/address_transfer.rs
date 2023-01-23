use crate::memory::short_word::ShortWord;
use crate::memory::swap_sign;
use crate::memory::word::Word;
use crate::memory::word_access::WordAccess;
use crate::memory::Bytes;
use crate::memory::Instruction;
use crate::memory::Sign;
use crate::memory::NEGATIVE;
use crate::memory::POSITIVE;
use crate::operations::*;
use crate::registers::RegisterType;
use crate::registers::Registers;

fn enter(instruction: Word, op_sing: Sign, r_type: RegisterType, reg: &mut Registers) {
    let m = instruction.get_address();

    let mut sign = instruction.get_sign();
    if op_sing == NEGATIVE {
        sign = swap_sign(sign);
    }

    let mut result = Word::new_from_signed(m);
    result.set_sign(sign);
    if m != 0 {
        reg.set_reg_by_type(r_type, result);
        return;
    }

    let i = instruction.get_i();
    if i != 0 {
        let ri = reg.get_i(i as usize);
        let mut ra = Word::new(ri.get());
        ra.set_sign(sign);
        reg.set_reg_by_type(r_type, ra);
        return;
    }

    let mut ra = reg.get_reg_by_type(r_type);
    ra.set_sign(sign);
    reg.set_reg_by_type(r_type, ra);
}

pub struct ENTA {
    code: u32,
    execution_time: u32,
    f: u8,
}

impl ENTA {
    pub fn new() -> ENTA {
        ENTA {
            code: 48,
            execution_time: 1,
            f: 2,
        }
    }
}

impl Operation for ENTA {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        enter(args.instruction, POSITIVE, RegisterType::A, args.reg);

        OperationResult::from_args(self.execution_time, args)
    }
}

pub struct ENTX {
    code: u32,
    execution_time: u32,
    f: u8,
}

impl ENTX {
    pub fn new() -> ENTX {
        ENTX {
            code: 55,
            execution_time: 1,
            f: 2,
        }
    }
}

impl Operation for ENTX {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        enter(args.instruction, POSITIVE, RegisterType::X, args.reg);

        OperationResult::from_args(self.execution_time, args)
    }
}

pub struct ENTi {
    code: u32,
    execution_time: u32,
    f: u8,
}

impl ENTi {
    pub fn new() -> ENTi {
        ENTi {
            code: 48,
            execution_time: 1,
            f: 2,
        }
    }
}

impl Operation for ENTi {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let m = args.instruction.get_address();
        let i = args.instruction.get_i();
        if m != 0 {
            args.reg.set_i(i as usize, ShortWord::new_from_signed(m));
            return OperationResult::from_args(self.execution_time, args);
        }

        let sign = args.instruction.get_sign();
        let mut ri = args.reg.get_i(i as usize);
        ri.set_sign(sign);
        args.reg.set_i(i as usize, ri);

        OperationResult::from_args(self.execution_time, args)
    }
}

pub struct ENNA {
    code: u32,
    execution_time: u32,
    f: u8,
}

impl ENNA {
    pub fn new() -> ENNA {
        ENNA {
            code: 48,
            execution_time: 1,
            f: 3,
        }
    }
}

impl Operation for ENNA {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        enter(args.instruction, NEGATIVE, RegisterType::A, args.reg);

        OperationResult::from_args(self.execution_time, args)
    }
}

pub struct ENNX {
    code: u32,
    execution_time: u32,
    f: u8,
}

impl ENNX {
    pub fn new() -> ENNX {
        ENNX {
            code: 55,
            execution_time: 1,
            f: 3,
        }
    }
}

impl Operation for ENNX {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        enter(args.instruction, NEGATIVE, RegisterType::X, args.reg);

        OperationResult::from_args(self.execution_time, args)
    }
}

pub struct ENNi {
    code: u32,
    execution_time: u32,
    f: u8,
}

impl ENNi {
    pub fn new() -> ENNi {
        ENNi {
            code: 48,
            execution_time: 1,
            f: 3,
        }
    }
}

impl Operation for ENNi {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let m = args.instruction.get_address();
        let i = args.instruction.get_i();
        let sign = swap_sign(args.instruction.get_sign());

        if m != 0 {
            let mut result = ShortWord::new_from_signed(m);
            result.set_sign(sign);
            args.reg.set_i(i as usize, result);
            return OperationResult::from_args(self.execution_time, args);
        }

        let mut ri = args.reg.get_i(i as usize);
        ri.set_sign(sign);
        args.reg.set_i(i as usize, ri);

        OperationResult::from_args(self.execution_time, args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enta_zero() {
        let op = ENTA::new();

        let mut r = Registers::new();
        let mut m = Memory::new();

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_001, 0, WordAccess::new_by_spec(2), 48),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(r.get_a(), Word::new_from_signed(2_001));

        let args = OperationArgs::new(
            1,
            Word::new_instruction(-2_001, 0, WordAccess::new_by_spec(2), 48),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(r.get_a(), Word::new_from_signed(-2_001));

        let args = OperationArgs::new(
            1,
            Word::new_instruction(0, 0, WordAccess::new_by_spec(2), 48),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(r.get_a(), Word::new_from_signed(2_001));

        let mut instruction = Word::new_instruction(0, 0, WordAccess::new_by_spec(2), 48);
        instruction.set_sign(-1);
        let args = OperationArgs::new(1, instruction, &mut m, &mut r);
        op.execute(args);
        assert_eq!(r.get_a(), Word::new_from_signed(-2_001));
    }

    #[test]
    fn enta_indexing() {
        let op = ENTA::new();

        let mut m = Memory::new();
        let mut r = Registers::new();
        r.set_i(1, ShortWord::new(2_001));

        let args = OperationArgs::new(
            1,
            Word::new_instruction(0, 1, WordAccess::new_by_spec(2), 48),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(r.get_a(), Word::new_from_signed(2_001));

        r.set_i(1, ShortWord::new(2_002));
        let mut instruction = Word::new_instruction(0, 1, WordAccess::new_by_spec(2), 48);
        instruction.set_sign(-1);
        let args = OperationArgs::new(1, instruction, &mut m, &mut r);
        op.execute(args);
        assert_eq!(r.get_a(), Word::new_from_signed(-2_002));
    }

    #[test]
    fn entx_zero() {
        let op = ENTX::new();

        let mut r = Registers::new();
        let mut m = Memory::new();

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_001, 0, WordAccess::new_by_spec(2), 48),
            &mut m,
            &mut r,
        );

        op.execute(args);
        assert_eq!(r.get_x(), Word::new_from_signed(2_001));

        let args = OperationArgs::new(
            1,
            Word::new_instruction(-2_001, 0, WordAccess::new_by_spec(2), 48),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(r.get_x(), Word::new_from_signed(-2_001));

        let args = OperationArgs::new(
            1,
            Word::new_instruction(0, 0, WordAccess::new_by_spec(2), 48),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(r.get_x(), Word::new_from_signed(2_001));

        let mut instruction = Word::new_instruction(0, 0, WordAccess::new_by_spec(2), 48);
        instruction.set_sign(-1);
        let args = OperationArgs::new(1, instruction, &mut m, &mut r);
        let op = ENTX::new();
        op.execute(args);
        assert_eq!(r.get_x(), Word::new_from_signed(-2_001));
    }

    #[test]
    fn entx_indexing() {
        let op = ENTX::new();

        let mut r = Registers::new();
        let mut m = Memory::new();
        r.set_i(1, ShortWord::new(2_001));

        let args = OperationArgs::new(
            1,
            Word::new_instruction(0, 1, WordAccess::new_by_spec(2), 48),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(r.get_x(), Word::new_from_signed(2_001));

        r.set_i(1, ShortWord::new(2_002));
        let mut instruction = Word::new_instruction(0, 1, WordAccess::new_by_spec(2), 48);
        instruction.set_sign(-1);
        let args = OperationArgs::new(1, instruction, &mut m, &mut r);
        let op = ENTX::new();
        op.execute(args);
        assert_eq!(r.get_x(), Word::new_from_signed(-2_002));
    }

    #[test]
    fn enti() {
        let op = ENTi::new();
        let mut m = Memory::new();
        let mut r = Registers::new();
        r.set_i(1, ShortWord::new(2_001));

        let args = OperationArgs::new(
            1,
            Word::new_instruction(11, 1, WordAccess::new_by_spec(2), 48),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(r.get_i(1), ShortWord::new_from_signed(11));

        let args = OperationArgs::new(
            1,
            Word::new_instruction(-12, 1, WordAccess::new_by_spec(2), 48),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(r.get_i(1), ShortWord::new_from_signed(-12));

        let args = OperationArgs::new(
            1,
            Word::new_instruction(0, 1, WordAccess::new_by_spec(2), 48),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(r.get_i(1), ShortWord::new_from_signed(12));

        let mut instruction = Word::new_instruction(0, 1, WordAccess::new_by_spec(2), 48);
        instruction.set_sign(-1);
        let args = OperationArgs::new(1, instruction, &mut m, &mut r);
        let op = ENTi::new();
        op.execute(args);
        assert_eq!(r.get_i(1), ShortWord::new_from_signed(-12));
    }

    #[test]
    fn enna_zero() {
        let op = ENNA::new();

        let mut m = Memory::new();
        let mut r = Registers::new();

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_001, 0, WordAccess::new_by_spec(2), 48),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(r.get_a(), Word::new_from_signed(-2_001));

        let args = OperationArgs::new(
            1,
            Word::new_instruction(-2_001, 0, WordAccess::new_by_spec(2), 48),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(r.get_a(), Word::new_from_signed(2_001));

        let args = OperationArgs::new(
            1,
            Word::new_instruction(0, 0, WordAccess::new_by_spec(2), 48),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(r.get_a(), Word::new_from_signed(-2_001));

        let mut instruction = Word::new_instruction(0, 0, WordAccess::new_by_spec(2), 48);
        instruction.set_sign(-1);
        let args = OperationArgs::new(1, instruction, &mut m, &mut r);
        op.execute(args);
        assert_eq!(r.get_a(), Word::new_from_signed(2_001));
    }

    #[test]
    fn enna_indexing() {
        let op = ENNA::new();
        let mut m = Memory::new();
        let mut r = Registers::new();
        r.set_i(1, ShortWord::new(2_001));

        let args = OperationArgs::new(
            1,
            Word::new_instruction(0, 1, WordAccess::new_by_spec(2), 48),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(r.get_a(), Word::new_from_signed(-2_001));

        r.set_i(1, ShortWord::new(2_002));
        let mut instruction = Word::new_instruction(0, 1, WordAccess::new_by_spec(2), 48);
        instruction.set_sign(-1);
        let args = OperationArgs::new(1, instruction, &mut m, &mut r);
        op.execute(args);
        assert_eq!(r.get_a(), Word::new_from_signed(2_002));
    }

    #[test]
    fn ennx_zero() {
        let op = ENNX::new();
        let mut m = Memory::new();
        let mut r = Registers::new();

        let args = OperationArgs::new(
            1,
            Word::new_instruction(2_001, 0, WordAccess::new_by_spec(2), 48),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(r.get_x(), Word::new_from_signed(-2_001));

        let args = OperationArgs::new(
            1,
            Word::new_instruction(-2_001, 0, WordAccess::new_by_spec(2), 48),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(r.get_x(), Word::new_from_signed(2_001));

        let args = OperationArgs::new(
            1,
            Word::new_instruction(0, 0, WordAccess::new_by_spec(2), 48),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(r.get_x(), Word::new_from_signed(-2_001));

        let mut instruction = Word::new_instruction(0, 0, WordAccess::new_by_spec(2), 48);
        instruction.set_sign(-1);
        let args = OperationArgs::new(1, instruction, &mut m, &mut r);
        op.execute(args);
        assert_eq!(r.get_x(), Word::new_from_signed(2_001));
    }

    #[test]
    fn ennx_indexing() {
        let op = ENNX::new();
        let mut m = Memory::new();
        let mut r = Registers::new();
        r.set_i(1, ShortWord::new(2_001));

        let args = OperationArgs::new(
            1,
            Word::new_instruction(0, 1, WordAccess::new_by_spec(2), 48),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(r.get_x(), Word::new_from_signed(-2_001));

        r.set_i(1, ShortWord::new(2_002));
        let mut instruction = Word::new_instruction(0, 1, WordAccess::new_by_spec(2), 48);
        instruction.set_sign(-1);
        let args = OperationArgs::new(1, instruction, &mut m, &mut r);
        op.execute(args);
        assert_eq!(r.get_x(), Word::new_from_signed(2_002));
    }

    #[test]
    fn enni() {
        let op = ENNi::new();
        let mut m = Memory::new();
        let mut r = Registers::new();
        r.set_i(1, ShortWord::new(2_001));

        let args = OperationArgs::new(
            1,
            Word::new_instruction(11, 1, WordAccess::new_by_spec(2), 48),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(r.get_i(1), ShortWord::new_from_signed(-11));

        let args = OperationArgs::new(
            1,
            Word::new_instruction(-12, 1, WordAccess::new_by_spec(2), 48),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(r.get_i(1), ShortWord::new_from_signed(12));

        let args = OperationArgs::new(
            1,
            Word::new_instruction(0, 1, WordAccess::new_by_spec(2), 48),
            &mut m,
            &mut r,
        );
        op.execute(args);
        assert_eq!(r.get_i(1), ShortWord::new_from_signed(-12));

        let mut instruction = Word::new_instruction(0, 1, WordAccess::new_by_spec(2), 48);
        instruction.set_sign(-1);
        let args = OperationArgs::new(1, instruction, &mut m, &mut r);
        op.execute(args);
        assert_eq!(r.get_i(1), ShortWord::new_from_signed(12));
    }
}
