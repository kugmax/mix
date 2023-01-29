use crate::memory::word::Word;
use crate::memory::word_access::WordAccess;
use crate::memory::Instruction;
use crate::memory::Bytes;
use crate::memory::Memory;
use crate::operations::address_arithmetic::*;
use crate::operations::address_transfer::*;
use crate::operations::arithmetic::*;
use crate::operations::compare::*;
use crate::operations::jump::*;
use crate::operations::load::*;
use crate::operations::miscellaneous::*;
use crate::operations::store::*;
use crate::operations::io::*;
use crate::registers::Registers;

pub mod address_arithmetic;
pub mod address_transfer;
pub mod arithmetic;
pub mod compare;
pub mod jump;
pub mod load;
pub mod miscellaneous;
pub mod store;
pub mod io;
// pub mod conversion;

// pub struct OperationDescription {
// code: u32,
// execution_time: u32,
// f: u8,
// }

trait Operation {
    fn execute(&self, args: OperationArgs) -> OperationResult;
}

pub struct OperationArgs<'a> {
    addr: u32,
    instruction: Word,
    mem: &'a mut Memory,
    reg: &'a mut Registers,
}

impl<'a> OperationArgs<'a> {
    pub fn new(
        addr: u32,
        instruction: Word,
        mem: &'a mut Memory,
        reg: &'a mut Registers,
    ) -> OperationArgs<'a> {
        OperationArgs {
            addr,
            instruction,
            mem,
            reg,
        }
    }
}

pub struct OperationResult {
    pub execution_time: u32,
    pub next_addr_instruction: u32,
}

impl OperationResult {
    pub fn from_args(execution_time: u32, args: OperationArgs) -> OperationResult {
        OperationResult {
            execution_time: execution_time,
            next_addr_instruction: args.addr + 1,
        }
    }

    pub fn new(execution_time: u32, next_addr_instruction: u32) -> OperationResult {
        OperationResult {
            execution_time,
            next_addr_instruction,
        }
    }
}

pub struct Operations {}

impl Operations {
    pub fn new() -> Operations {
        Operations {}
    }
    pub fn execute(
        &self,
        addr: u32,
        instruction: Word,
        mem: &mut Memory,
        reg: &mut Registers,
    ) -> OperationResult {
        let op = self.get_operation(instruction.get_c(), instruction.get_byte(4));

        let args = OperationArgs::new(addr, instruction, mem, reg);
        op.execute(args)
    }

    fn get_operation(&self, code: u8, f: u8) -> Box<dyn Operation> {
        // println!("op: {code} {f}");
        return match code {
            0 => Box::new(NOP::new()),

            // arithmetic
            1 => Box::new(ADD::new()),
            2 => Box::new(SUB::new()),
            3 => Box::new(MUL::new()),
            4 => Box::new(DIV::new()),

            5 if f == 2 => Box::new(HLT::new()),

            // shift
            6 if f == 0 => Box::new(SLA::new()),
            6 if f == 1 => Box::new(SRA::new()),
            6 if f == 2 => Box::new(SLAX::new()),
            6 if f == 3 => Box::new(SRAX::new()),
            6 if f == 4 => Box::new(SLC::new()),
            6 if f == 5 => Box::new(SRC::new()),

            7 => Box::new(MOVE::new()),

            // load
            8 => Box::new(LDA::new()),
            9..=14 => Box::new(LDi::new()),
            15 => Box::new(LDX::new()),
            16 => Box::new(LDAN::new()),
            17..=22 => Box::new(LDiN::new()),
            23 => Box::new(LDXN::new()),

            // store
            24 => Box::new(STA::new()),
            25..=30 => Box::new(STi::new()),
            31 => Box::new(STX::new()),
            32 => Box::new(STJ::new()),
            33 => Box::new(STZ::new()),

            // jump
            39 if f == 0 => Box::new(JMP::new()),
            39 if f == 1 => Box::new(JSJ::new()),
            39 if f == 2 => Box::new(JOV::new()),
            39 if f == 3 => Box::new(JNOV::new()),
            39 if f == 4 => Box::new(JL::new()),
            39 if f == 5 => Box::new(JE::new()),
            39 if f == 6 => Box::new(JG::new()),
            39 if f == 7 => Box::new(JGE::new()),
            39 if f == 8 => Box::new(JNE::new()),
            39 if f == 9 => Box::new(JLE::new()),

            40 if f == 0 => Box::new(JAN::new()),
            40 if f == 1 => Box::new(JAZ::new()),
            40 if f == 2 => Box::new(JAP::new()),
            40 if f == 3 => Box::new(JANN::new()),
            40 if f == 4 => Box::new(JANZ::new()),
            40 if f == 5 => Box::new(JANP::new()),

            41..=46 if f == 0 => Box::new(JiN::new()),
            41..=46 if f == 1 => Box::new(JiZ::new()),
            41..=46 if f == 2 => Box::new(JiP::new()),
            41..=46 if f == 3 => Box::new(JiNN::new()),
            41..=46 if f == 4 => Box::new(JiNZ::new()),
            41..=46 if f == 5 => Box::new(JiNP::new()),

            47 if f == 0 => Box::new(JXN::new()),
            47 if f == 1 => Box::new(JXZ::new()),
            47 if f == 2 => Box::new(JXP::new()),
            47 if f == 3 => Box::new(JXNN::new()),
            47 if f == 4 => Box::new(JXNZ::new()),
            47 if f == 5 => Box::new(JXNP::new()),

            // address_transfer
            48 if f == 0 => Box::new(INCA::new()),
            48 if f == 1 => Box::new(DECA::new()),
            48 if f == 2 => Box::new(ENTA::new()),
            48 if f == 3 => Box::new(ENNA::new()),

            47..=54 if f == 0 => Box::new(INCi::new()),
            47..=54 if f == 1 => Box::new(DECi::new()),
            47..=54 if f == 2 => Box::new(ENTi::new()),
            47..=54 if f == 3 => Box::new(ENNi::new()),

            55 if f == 0 => Box::new(INCX::new()),
            55 if f == 1 => Box::new(DECX::new()),
            55 if f == 2 => Box::new(ENTX::new()),
            55 if f == 3 => Box::new(ENNX::new()),

            // compare
            56 => Box::new(CMPA::new()),
            57..=62 => Box::new(CMPi::new()),
            63 => Box::new(CMPX::new()),

            _ => panic!("unsupported operation code {code}"),
        };
    }
}

pub fn get_memory_cell(instruction: impl Instruction, mem: &Memory, reg: &Registers) -> Word {
    let mut addr = instruction.get_address();
    addr = addr.abs();

    let i = instruction.get_i();
    if i != 0 {
        addr += reg.get_i(i as usize).get_signed_value();
    }

    mem.get(addr as usize)
}

pub fn get_indexed_addr(instruction: impl Instruction, reg: &Registers) -> i32 {
    let mut addr = instruction.get_address();
    addr = addr.abs();

    let i = instruction.get_i();
    if i != 0 {
        addr += reg.get_i(i as usize).get_signed_value();
    }
    addr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operations() {
        let mut mem = Memory::new();
        mem.set(2_000, 77);

        let mut reg = Registers::new();

        let operations = Operations::new();

        let result = operations.execute(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 8),
            &mut mem,
            &mut reg,
        );
        assert_eq!(reg.get_a().get_signed_value(), 77);

        let result = operations.execute(
            1,
            Word::new_instruction(2_000, 0, WordAccess::new(0, 5), 15),
            &mut mem,
            &mut reg,
        );
        assert_eq!(reg.get_x().get_signed_value(), 77);

        let result = operations.execute(
            1,
            Word::new_instruction(2_000, 1, WordAccess::new(0, 5), 9),
            &mut mem,
            &mut reg,
        );
        assert_eq!(reg.get_i(1).get_signed_value(), 77);

        let result = operations.execute(
            1,
            Word::new_instruction(2_000, 6, WordAccess::new(0, 5), 14),
            &mut mem,
            &mut reg,
        );
        assert_eq!(reg.get_i(6).get_signed_value(), 77);
    }
}
