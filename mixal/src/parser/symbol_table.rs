use crate::lexer::token::*;
use crate::lexer::*;
use crate::parser::addr_parser::*;
use crate::pseudo_op::*;
use crate::tags::*;
use crate::word::*;

use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

pub struct LocalSymbolTabel {
    local_symbols: HashMap<u8, Vec<u32>>,
    // local_symbols: HashMap<u8, Vec<(u32, u32)>>,
}
impl LocalSymbolTabel {
    pub fn new() -> LocalSymbolTabel {
        LocalSymbolTabel {
            local_symbols: HashMap::new(),
        }
    }

    pub fn get(&self, name: String, current_addrs: u32) -> u32 {
        let digit = self.get_digit(name.clone());
        let mut addresses = self
            .local_symbols
            .get(&digit)
            .expect("local symbol not found")
            .to_vec();

        let is_up = self.is_up_direction(name.clone());
        if is_up {
            addresses.reverse();
        }

        for addr in addresses {
            if is_up {
                if addr < current_addrs {
                    return addr;
                }
            } else {
                if addr > current_addrs {
                    return addr;
                }
            }
        }
        panic!("local symbol not found {name}:{current_addrs}");
    }
    fn put(&mut self, name: String, address: u32) {
        let digit = self.get_digit(name);
        let mut local_symbols_addrs = match self.local_symbols.get(&digit) {
            None => Vec::new(),
            Some(v) => v.to_vec(),
        };
        local_symbols_addrs.push(address);
        self.local_symbols.insert(digit, local_symbols_addrs);
    }

    pub fn is_local_symbol(&self, name: &String) -> bool {
        if name.len() == 2 {
            let mut chars = name.chars();
            let digit = chars.next().expect("error");
            let direction = chars.next().expect("error");

            return digit.is_numeric()
                && (direction == 'H' || direction == 'B' || direction == 'F');
        }

        return false;
    }

    fn get_digit(&self, name: String) -> u8 {
        let mut chars = name.chars();
        let digit = chars.next().expect("error");
        digit.to_digit(10).expect("error") as u8
    }

    fn is_up_direction(&self, name: String) -> bool {
        let mut chars = name.chars();
        chars.next();
        chars.next().expect("error") == 'B'
    }
}

pub struct SymbolTable {
    equ_values: HashMap<String, Word>,
    references: HashMap<String, u32>,
    local_symbols: LocalSymbolTabel,
}
impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            equ_values: HashMap::new(),
            references: HashMap::new(),
            local_symbols: LocalSymbolTabel::new(),
        }
    }
    pub fn get(&self, name: String, current_addrs: u32) -> i32 {
        if self.local_symbols.is_local_symbol(&name) {
            return self.local_symbols.get(name, current_addrs) as i32;
        }

        return match self.equ_values.get(&name) {
            Some(v) => v.get_signed_value(),
            None => match self.references.get(&name) {
                Some(v) => *v as i32,
                None => panic!("symbol {} not found", name),
            },
        };
    }

    pub fn put_equ(&mut self, name: String, value: Word) {
        self.equ_values.insert(name, value);
    }

    pub fn put_reference(&mut self, name: String, address: u32) {
        if self.local_symbols.is_local_symbol(&name) {
            self.local_symbols.put(name, address);
            return;
        }
        self.references.insert(name, address);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_symbol_table() {
        let mut table = LocalSymbolTabel::new();
        table.put("2H".to_string(), 2);
        table.put("2H".to_string(), 4);
        table.put("2H".to_string(), 10);

        table.put("3H".to_string(), 20);
        table.put("3H".to_string(), 30);
        table.put("3H".to_string(), 100);

        // assert_eq!(4, table.get("2B".to_string(), 0));

        assert_eq!(4, table.get("2B".to_string(), 7));
        assert_eq!(2, table.get("2B".to_string(), 3));
        assert_eq!(10, table.get("2B".to_string(), 200));
        
        assert_eq!(20, table.get("3F".to_string(), 0));
        assert_eq!(30, table.get("3F".to_string(), 22));
        assert_eq!(2, table.get("2F".to_string(), 0));
    }
}
