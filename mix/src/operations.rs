use crate::memory::Instruction;
use crate::memory::word::Word;
use crate::memory::Memory;
use crate::registers::Registers;

pub mod address_transfer;
pub mod arithmetic;
pub mod load;
pub mod store;
// pub mod compare;
// pub mod jump;
// pub mod miscellaneous;
// pub mod io;
// pub mod conversion;

struct Operations {}

pub fn get_memory_cell(instruction: Word, mem: &Memory, reg: &Registers) -> Word {
    let mut addr = instruction.get_address();
    addr = addr.abs();

    let i = instruction.get_i();
    if i != 0 {
        addr += reg.get_i(i as usize).get_signed_value();
    }

    mem.get(addr as usize)
}
