use crate::memory::short_word::ShortWord;
use crate::memory::word::Word;
use crate::memory::Bytes;
use crate::memory::Instruction;
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
        let mut count = 0;
        while true {
            if self.addr > 3_999 {
                break;
            }

            let instruction = mem.get(self.addr as usize);

            let result = op.execute(self.addr, instruction, mem, reg);

            // println!("      {:#?}", reg);
            // for i in 0..10 {
                // println!("      {i}={}", mem.get(i as usize).get_signed_value());
            // }

            self.addr = result.next_addr_instruction;

            count += 1;
            if count > 50 {
                break;
            }
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
        r.set_j(ShortWord::new(4_000));

        // elements
        m.set(1_000, 1);
        m.set(1_001, 2);
        m.set(1_002, 2);
        m.set(1_003, 3);
        m.set(1_004, 6);
        m.set(1_005, 20);
        m.set(1_006, 2);
        m.set(1_007, 33);
        // m.set(1_007, Word::new_from_signed(-33).get());
        m.set(1_008, 2);
        m.set(1_009, 10);

        p.execute(&mut m, &mut r);

        let i = r.get_i(2).get() as usize;
        let max = m.get(1_000 + i).get();

        assert_eq!(i, 7);
        assert_eq!(max, 33);
    }

    #[test]
    fn program_p() {
        let mut m = Memory::new();
        let mut r = Registers::new();

        let mut p = Processor::start_from(3_000);

        m.set_instr_as_bytes(3_000, 0, 0, 18, 35);
        m.set_instr_as_bytes(3_001, 2051, 0, 5, 9);
        m.set_instr_as_bytes(3_002, 2050, 0, 5, 10);
        m.set_instr_as_bytes(3_003, 1, 0, 0, 49);
        m.set_instr_as_bytes(3_004, 499, 1, 5, 26);
        m.set_instr_as_bytes(3_005, 3016, 0, 1, 41);
        m.set_instr_as_bytes(3_006, 2, 0, 0, 50);
        m.set_instr_as_bytes(3_007, 2, 0, 2, 51);
        m.set_instr_as_bytes(3_008, 0, 0, 2, 48);
        m.set_instr_as_bytes(3_009, 0, 2, 2, 55);
        m.set_instr_as_bytes(3_010, -1, 3, 5, 4);
        m.set_instr_as_bytes(3_011, 3006, 0, 1, 47);
        m.set_instr_as_bytes(3_012, -1, 3, 5, 56);
        m.set_instr_as_bytes(3_013, 1, 0, 0, 51);
        m.set_instr_as_bytes(3_014, 3008, 0, 6, 39);
        m.set_instr_as_bytes(3_015, 3003, 0, 0, 39);
        m.set_instr_as_bytes(3_016, 1995, 0, 18, 37);
        m.set_instr_as_bytes(3_017, 2035, 0, 2, 52);
        m.set_instr_as_bytes(3_018, -50, 0, 2, 53);
        m.set_instr_as_bytes(3_019, 501, 0, 0, 53);
        m.set_instr_as_bytes(3_020, -1, 5, 5, 8);
        m.set_instr_as_bytes(3_021, 0, 0, 1, 5);
        m.set_instr_as_bytes(3_022, 0, 4, 12, 31);
        m.set_instr_as_bytes(3_023, 1, 0, 1, 52);
        m.set_instr_as_bytes(3_024, 50, 0, 1, 53);
        m.set_instr_as_bytes(3_025, 3020, 0, 2, 45);
        m.set_instr_as_bytes(3_026, 0, 4, 18, 37);
        m.set_instr_as_bytes(3_027, 24, 4, 5, 12);
        m.set_instr_as_bytes(3_028, 3019, 0, 0, 45);
        m.set_instr_as_bytes(3_029, 0, 0, 2, 5); //HLT

        m.set_instr_as_bytes(3_030, 4_000, 0, 0, 39); //exit

        m.set_word(0_000, Word::new_from_signed(2));

        m.set_bytes(1_995, 0, 6, 9, 19, 22, 23);
        m.set_bytes(1_996, 0, 0, 6, 9, 25, 5);
        m.set_bytes(1_997, 0, 0, 8, 24, 15, 4);
        m.set_bytes(1_998, 0, 19, 5, 4, 0, 17);
        m.set_bytes(1_999, 0, 19, 9, 14, 5, 22);

        m.set_word(2_024, Word::new_from_signed(2035));
        m.set_word(2_049, Word::new_from_signed(2010));
        m.set_word(2_050, Word::new_from_signed(3));
        m.set_word(2_051, Word::new_from_signed(-499));
        // m.set_word(2_051, Word::new_from_signed(-10));

        // r.set_i(1, ShortWord::new(10));
        // r.set_j(ShortWord::new(4_000));
        // r.set_i(1, ShortWord::new(1));
        // r.set_i(2, ShortWord::new(5));

        p.execute(&mut m, &mut r);
    }
}
