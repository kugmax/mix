use crate::memory::short_word::ShortWord;
use crate::memory::word::Word;
use crate::memory::Bytes;
use crate::memory::Instruction;
use crate::memory::Memory;
use crate::processor::Processor;
use crate::registers::Registers;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

pub struct MIX {
    reg: Registers,
    mem: Memory,
    proc: Processor,
}

impl MIX {
    pub fn new() -> MIX {
        MIX {
            reg: Registers::new(),
            mem: Memory::new(),
            proc: Processor::new(),
        }
    }

    pub fn execute(&mut self) {
      self.proc.execute(&mut self.mem, &mut self.reg);
    }

    pub fn load(&mut self, path: &str) {
        let lines: Vec<String> = self.read_programm(path);

        for line in lines {
            let bytes: Vec<&str> = line.split(",").map(|s| s.trim()).collect();
            let len = bytes.len();
            // println!("{:#?}", bytes);
            // let addr = bytes.get(0);

            let addr = u32::from_str(bytes.get(0).expect("cant get addr")).expect("cant parse u32");

            if len == 1 {
                self.proc.set_addr(addr);
            } else if len == 2 {
                let value =
                    i32::from_str(bytes.get(1).expect("cant get 2")).expect("cant parse i32");

                self.mem
                    .set_word(addr as usize, Word::new_from_signed(value));
            } else if len == 5 {
                let AA = self.parse_i32(&bytes, 1);
                let byte_3 = self.parse_u8(&bytes, 2);
                let byte_4 = self.parse_u8(&bytes, 3);
                let byte_5 = self.parse_u8(&bytes, 4);

                self.mem
                    .set_instr_as_bytes(addr as usize, AA, byte_3, byte_4, byte_5);
            } else if len == 7 {
                let sign = self.parse_i8(&bytes, 1);
                let byte_1 = self.parse_u8(&bytes, 2);
                let byte_2 = self.parse_u8(&bytes, 3);
                let byte_3 = self.parse_u8(&bytes, 4);
                let byte_4 = self.parse_u8(&bytes, 5);
                let byte_5 = self.parse_u8(&bytes, 6);

                self.mem
                    .set_bytes(addr as usize, sign, byte_1, byte_2, byte_3, byte_4, byte_5);
            } else {
                panic!("unsupported program format {len}");
            }
        }
    }

    fn parse_u8(&self, bytes: &Vec<&str>, byte: usize) -> u8 {
        u8::from_str(bytes.get(byte).expect("cant get byte")).expect("cant parse u8")
    }

    fn parse_i8(&self, bytes: &Vec<&str>, byte: usize) -> i8 {
        i8::from_str(bytes.get(byte).expect("cant get byte")).expect("cant parse i8")
    }

    fn parse_i32(&self, bytes: &Vec<&str>, byte: usize) -> i32 {
        i32::from_str(bytes.get(byte).expect("cant get byte")).expect("cant parse i32")
    }

    fn read_programm(&self, path: &str) -> Vec<String> {
        let mut file = File::open(path.to_string()).expect(&("file not found ".to_owned() + path));
        let mut reader = BufReader::new(file);
        let mut result = Vec::new();

        // let mut line = String::new();
        // let len = reader.read_line(&mut line)?;
        let lines = reader.lines();
        for line in lines {
            let line = line.expect("some err in lines");

            // let line = line::trim();
            let line = line.trim();
            if line.len() <= 0 {
                continue;
            }
            result.push(line.to_string());

            println!("{line}");
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_program() {
        let mut mix = MIX::new();

        mix.load("../programs/print_500_primes.mix");

        mix.execute();
    }
}
