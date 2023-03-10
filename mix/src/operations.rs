use crate::memory::word::Word;
use crate::memory::word_access::WordAccess;
use crate::memory::Bytes;
use crate::memory::Instruction;
use crate::memory::Memory;
use crate::operations::address_arithmetic::*;
use crate::operations::address_transfer::*;
use crate::operations::arithmetic::*;
use crate::operations::compare::*;
use crate::operations::conversion::*;
use crate::operations::io::*;
use crate::operations::jump::*;
use crate::operations::load::*;
use crate::operations::miscellaneous::*;
use crate::operations::store::*;
use crate::registers::Registers;

pub mod address_arithmetic;
pub mod address_transfer;
pub mod arithmetic;
pub mod compare;
pub mod conversion;
pub mod io;
pub mod jump;
pub mod load;
pub mod miscellaneous;
pub mod store;

// pub struct OperationDescription {
// code: u32,
// execution_time: u32,
// f: u8,
// }

trait Operation {
    fn execute(&self, args: OperationArgs) -> OperationResult;
    fn get_name(&self) -> String;
}

pub struct OperationArgs<'a> {
    addr: u32,
    // instruction: Word,
    mem: &'a mut Memory,
    reg: &'a mut Registers,
}

impl<'a> OperationArgs<'a> {
    pub fn new(
        addr: u32,
        // instruction: Word,
        mem: &'a mut Memory,
        reg: &'a mut Registers,
    ) -> OperationArgs<'a> {
        OperationArgs {
            addr,
            // instruction,
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
        let op = self.get_operation(instruction);

        // println!(
        // "{}| {}: {} {} {} {}",
        // addr,
        // op.get_name(),
        // instruction.get_address(),
        // instruction.get_i(),
        // instruction.get_byte(4),
        // instruction.get_c()
        // );

        let args = OperationArgs::new(addr, mem, reg);
        op.execute(args)
    }

    fn get_operation(&self, instruction: Word) -> Box<dyn Operation> {
        let code = instruction.get_c();
        let f = instruction.get_byte(4);
        return match code {
            0 => Box::new(NOP::new(instruction)),

            // arithmetic
            1 => Box::new(ADD::new(instruction)),
            2 => Box::new(SUB::new(instruction)),
            3 => Box::new(MUL::new(instruction)),
            4 => Box::new(DIV::new(instruction)),

            5 if f == 0 => Box::new(NUM::new(instruction)),
            5 if f == 1 => Box::new(CHAR::new(instruction)),
            5 if f == 2 => Box::new(HLT::new(instruction)),

            // shift
            6 if f == 0 => Box::new(SLA::new(instruction)),
            6 if f == 1 => Box::new(SRA::new(instruction)),
            6 if f == 2 => Box::new(SLAX::new(instruction)),
            6 if f == 3 => Box::new(SRAX::new(instruction)),
            6 if f == 4 => Box::new(SLC::new(instruction)),
            6 if f == 5 => Box::new(SRC::new(instruction)),

            7 => Box::new(MOVE::new(instruction)),

            // load
            8 => Box::new(LDA::new(instruction)),
            9..=14 => Box::new(LDi::new(instruction)),
            15 => Box::new(LDX::new(instruction)),
            16 => Box::new(LDAN::new(instruction)),
            17..=22 => Box::new(LDiN::new(instruction)),
            23 => Box::new(LDXN::new(instruction)),

            // store
            24 => Box::new(STA::new(instruction)),
            25..=30 => Box::new(STi::new(instruction)),
            31 => Box::new(STX::new(instruction)),
            32 => Box::new(STJ::new(instruction)),
            33 => Box::new(STZ::new(instruction)),

            //IO
            35 => Box::new(IOC::new(instruction)),
            36 => Box::new(IN::new(instruction)),
            37 => Box::new(OUT::new(instruction)),

            // jump
            39 if f == 0 => Box::new(JMP::new(instruction)),
            39 if f == 1 => Box::new(JSJ::new(instruction)),
            39 if f == 2 => Box::new(JOV::new(instruction)),
            39 if f == 3 => Box::new(JNOV::new(instruction)),
            39 if f == 4 => Box::new(JL::new(instruction)),
            39 if f == 5 => Box::new(JE::new(instruction)),
            39 if f == 6 => Box::new(JG::new(instruction)),
            39 if f == 7 => Box::new(JGE::new(instruction)),
            39 if f == 8 => Box::new(JNE::new(instruction)),
            39 if f == 9 => Box::new(JLE::new(instruction)),

            40 if f == 0 => Box::new(JAN::new(instruction)),
            40 if f == 1 => Box::new(JAZ::new(instruction)),
            40 if f == 2 => Box::new(JAP::new(instruction)),
            40 if f == 3 => Box::new(JANN::new(instruction)),
            40 if f == 4 => Box::new(JANZ::new(instruction)),
            40 if f == 5 => Box::new(JANP::new(instruction)),

            41..=46 if f == 0 => Box::new(JiN::new(instruction)),
            41..=46 if f == 1 => Box::new(JiZ::new(instruction)),
            41..=46 if f == 2 => Box::new(JiP::new(instruction)),
            41..=46 if f == 3 => Box::new(JiNN::new(instruction)),
            41..=46 if f == 4 => Box::new(JiNZ::new(instruction)),
            41..=46 if f == 5 => Box::new(JiNP::new(instruction)),

            47 if f == 0 => Box::new(JXN::new(instruction)),
            47 if f == 1 => Box::new(JXZ::new(instruction)),
            47 if f == 2 => Box::new(JXP::new(instruction)),
            47 if f == 3 => Box::new(JXNN::new(instruction)),
            47 if f == 4 => Box::new(JXNZ::new(instruction)),
            47 if f == 5 => Box::new(JXNP::new(instruction)),

            // address_transfer
            48 if f == 0 => Box::new(INCA::new(instruction)),
            48 if f == 1 => Box::new(DECA::new(instruction)),
            48 if f == 2 => Box::new(ENTA::new(instruction)),
            48 if f == 3 => Box::new(ENNA::new(instruction)),

            49..=54 if f == 0 => Box::new(INCi::new(instruction)),
            49..=54 if f == 1 => Box::new(DECi::new(instruction)),
            49..=54 if f == 2 => Box::new(ENTi::new(instruction)),
            49..=54 if f == 3 => Box::new(ENNi::new(instruction)),

            55 if f == 0 => Box::new(INCX::new(instruction)),
            55 if f == 1 => Box::new(DECX::new(instruction)),
            55 if f == 2 => Box::new(ENTX::new(instruction)),
            55 if f == 3 => Box::new(ENNX::new(instruction)),

            // compare
            56 => Box::new(CMPA::new(instruction)),
            57..=62 => Box::new(CMPi::new(instruction)),
            63 => Box::new(CMPX::new(instruction)),

            _ => panic!("unsupported operation code {code}"),
        };
    }
}

pub fn get_memory_cell(instruction: impl Instruction, mem: &Memory, reg: &Registers) -> Word {
    let mut addr = instruction.get_address();

    let i = instruction.get_i();
    if i != 0 {
        addr += reg.get_i(i as usize).get_signed_value();
    }

    mem.get(addr as usize)
}

pub fn get_indexed_addr(instruction: impl Instruction, reg: &Registers) -> i32 {
    let mut addr = instruction.get_address();

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
