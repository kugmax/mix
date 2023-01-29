use crate::memory::word::Word;
use crate::memory::word::MAX_5_BYTES;
use crate::memory::Bytes;
use crate::memory::Instruction;
use crate::memory::Memory;
use crate::operations::get_memory_cell;
use crate::operations::*;
use crate::registers::Registers;

// pub const TAPE:IoUnit = IoUnit::UNIT(0, 100, "Tape unit number 0");
//
// pub enum IoUnit<'a> {
  // UNIT(u8, u8, &'a str)
// }

//TODO: create virual device which would write/read to the file
//the name of the file containts unit type, info
pub struct IO_UNIT {
}

impl IO_UNIT {
}

pub struct IN {
    code: u32,
    execution_time: u32,
}
impl IN {
    pub fn new() -> IN {
        IN {
            code: 36,
            execution_time: 1, // + T
        }
    }
}
impl Operation for IN {
    fn execute(&self, args: OperationArgs) -> OperationResult {

        OperationResult::from_args(self.execution_time, args)
    }
}

pub struct OUT {
    code: u32,
    execution_time: u32,
}
impl OUT {
    pub fn new() -> OUT {
        OUT {
            code: 37,
            execution_time: 1, // + T
        }
    }
}
impl Operation for OUT {
    fn execute(&self, args: OperationArgs) -> OperationResult {

        OperationResult::from_args(self.execution_time, args)
    }
}

pub struct IOC {
    code: u32,
    execution_time: u32,
}
impl IOC {
    pub fn new() -> IOC {
        IOC {
            code: 35,
            execution_time: 1, // + T
        }
    }
}
impl Operation for IOC {
    fn execute(&self, args: OperationArgs) -> OperationResult {

        OperationResult::from_args(self.execution_time, args)
    }
}
