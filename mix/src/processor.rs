use crate::memory::short_word::ShortWord;
use crate::memory::word::Word;
use crate::memory::word_access::WordAccess;
use crate::memory::Instruction;
use crate::memory::Bytes;
use crate::memory::Memory;
use crate::operations::Operations;
use crate::registers::Registers;

pub struct Processor {
    addr: u32,
}

impl Processor {
    pub fn new() -> Processor {
        Processor { addr: 0 }
    }

    pub fn start_from(addr: u32) -> Processor {
        Processor { addr }
    }

    pub fn execute(&mut self, mem: &mut Memory, reg: &mut Registers) {
        let op = Operations::new();

        while true {
            if self.addr > 3_999 {
                break;
            }

            let instruction = mem.get(self.addr as usize);

            // println!(
                // "{}:{} {} {} {}",
                // self.addr,
                // instruction.get_address(),
                // instruction.get_i(),
                // instruction.get_byte(4),
                // instruction.get_c()
            // );

            let result = op.execute(self.addr, instruction, mem, reg);

            // println!("{:#?}", reg);

            self.addr = result.next_addr_instruction;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_maximum() {
        let mut m = Memory::new();
        let mut r = Registers::new();

        let mut p = Processor::start_from(3_000);

        // programm code
        m.set_instr_as_bytes(3_000, 3009, 0, 2, 32);
        m.set_instr_as_bytes(3_001, 0, 1, 2, 51);
        m.set_instr_as_bytes(3_002, 3005, 0, 0, 39);
        m.set_instr_as_bytes(3_003, 1000, 3, 5, 56);
        m.set_instr_as_bytes(3_004, 3007, 0, 7, 39);
        m.set_instr_as_bytes(3_005, 0, 3, 2, 50);
        m.set_instr_as_bytes(3_006, 1000, 3, 5, 8);
        m.set_instr_as_bytes(3_007, 1, 0, 1, 51);
        m.set_instr_as_bytes(3_008, 3003, 0, 2, 43);
        m.set_instr_as_bytes(3_009, 3009, 0, 0, 39);

        // n
        r.set_i(1, ShortWord::new(10));
        r.set_j(ShortWord::new(3_999));

        // elements
        m.set(1_000, 1);
        m.set(1_001, 2);
        m.set(1_002, 2);
        m.set(1_003, 3);
        m.set(1_004, 6);
        m.set(1_005, 20);
        m.set(1_006, 2);
        m.set(1_007, 33);
        m.set(1_008, 2);
        m.set(1_009, 10);

        p.execute(&mut m, &mut r);

        let i = r.get_i(2).get() as usize;
        let max = m.get(1_000 + i).get();

        println!("Largest element is {max} {i}");
        // println!("{}", r.get_i(3).get());
    }
}
