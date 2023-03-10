use crate::memory::word::Word;
use crate::memory::word::MAX_5_BYTES;
use crate::memory::Bytes;
use crate::memory::Instruction;
use crate::memory::Memory;
use crate::operations::conversion::*;
use crate::operations::get_memory_cell;
use crate::operations::*;
use crate::registers::Registers;

use std::fs::File;
use std::io;
use std::io::prelude::*;

pub const IO_FILE_PREFIX: &str = "io_unit_";

pub struct IO_UNIT {}

impl IO_UNIT {}

pub struct IN {
    code: u32,
    execution_time: u32,
    instruction: Word,
}
impl IN {
    pub fn new(instruction: Word) -> IN {
        IN {
            code: 36,
            execution_time: 1, // + T
            instruction: instruction,
        }
    }
}
impl Operation for IN {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let io_unit = self.instruction.get_byte(4);
        if io_unit != 18 {
            panic!("unsupported io unit {io_unit}");
        }
        panic!("not implemented");

        // OperationResult::from_args(self.execution_time, args)
    }
    fn get_name(&self) -> String {
        String::from("IN")
    }
}

pub struct OUT {
    code: u32,
    execution_time: u32,
    instruction: Word,
}
impl OUT {
    pub fn new(instruction: Word) -> OUT {
        OUT {
            code: 37,
            execution_time: 1, // + T
            instruction: instruction,
        }
    }

    fn write(&self, io_unit: u8, words: Vec<Word>) -> io::Result<()> {
        let path = IO_FILE_PREFIX.to_string() + &io_unit.to_string();
        let mut file = File::options().create(true).append(true).open(path)?;

        let mut line = String::new();
        for w in words {
            for b in 1..6 {
                line += &SYMBOLS[w.get_byte(b) as usize].to_string();
            }
        }
        line += &"\n";

        file.write_all(&line.as_bytes())?;

        Ok(())
    }
}
impl Operation for OUT {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let io_unit = self.instruction.get_byte(4);
        if io_unit != 18 {
            panic!("unsupported io unit {io_unit}");
        }
        let unit_block = 24;
        let start_from = get_indexed_addr(self.instruction, args.reg);

        let mut out_buffer = Vec::new();
        for i in 0..unit_block {
            let addr = (start_from + i) as usize;
            out_buffer.push(args.mem.get(addr));
        }

        self.write(io_unit, out_buffer);

        OperationResult::from_args(self.execution_time, args)
    }
    fn get_name(&self) -> String {
        String::from("OUT")
    }
}

pub struct IOC {
    code: u32,
    execution_time: u32,
    instruction: Word,
}
impl IOC {
    pub fn new(instruction: Word) -> IOC {
        IOC {
            code: 35,
            execution_time: 1, // + T
            instruction: instruction,
        }
    }

    fn write(&self, io_unit: u8) -> io::Result<()> {
        // let path = "/home/max/Documents/Projects.git/mix/mix/target/";
        // let mut file = File::create(path.to_string() + &IO_FILE_PREFIX.to_string() + &io_unit.to_string())?;

        let path = IO_FILE_PREFIX.to_string() + &io_unit.to_string();
        let mut file = File::options().create(true).append(true).open(path)?;

        file.write_all(b"\n")?;

        Ok(())
    }
}
impl Operation for IOC {
    fn execute(&self, args: OperationArgs) -> OperationResult {
        let io_unit = self.instruction.get_byte(4);
        if io_unit != 18 {
            panic!("unsupported io unit {io_unit}");
        }
        self.write(io_unit);

        OperationResult::from_args(self.execution_time, args)
    }
    fn get_name(&self) -> String {
        String::from("IOC")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn out() {
        let mut m = Memory::new();
        let mut r = Registers::new();

        // HELLO
        //  WORL
        //  D
        m.set_bytes(0, 0, 8, 5, 13, 13, 16);
        m.set_bytes(1, 0, 0, 26, 16, 19, 13);
        m.set_bytes(2, 0, 4, 0, 0, 0, 0);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let op = OUT::new(Word::new_by_bytes(0, &[0, 0, 0, 18, 37]));
        op.execute(args);
    }

    // #[test]
    fn ioc_out() {
        let mut m = Memory::new();
        let mut r = Registers::new();

        // HELLO
        //  WORL
        //  D
        m.set_bytes(0, 0, 8, 5, 13, 13, 16);
        m.set_bytes(1, 0, 0, 26, 16, 19, 13);
        m.set_bytes(2, 0, 4, 0, 0, 0, 0);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let op = OUT::new(Word::new_by_bytes(0, &[0, 0, 0, 18, 37]));
        op.execute(args);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let ioc = IOC::new(Word::new_by_bytes(0, &[0, 0, 0, 18, 37]));
        ioc.execute(args);

        let args = OperationArgs::new(1, &mut m, &mut r);
        let op = OUT::new(Word::new_by_bytes(0, &[0, 0, 0, 18, 37]));
        op.execute(args);
    }
}
