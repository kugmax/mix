use crate::lexer::split_whitespace_once;
use crate::parser::Printable;
use std::collections::HashMap;

const SYMBOLS: [char; 56] = [
    ' ', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', '\u{0394}', 'J', 'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', '\u{2211}', '\u{03A0}', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2',
    '3', '4', '5', '6', '7', '8', '9', '.', ',', '(', ')', '+', '-', '*', '/', '=', '$', '<', '>',
    '@', ';', ':', '\'',
];

struct AlfSymbols {
    symbol_ids: HashMap<char, u8>,
}
impl AlfSymbols {
    pub fn new() -> AlfSymbols {
        let mut symbols = HashMap::new();
        let mut i = 0;
        for c in SYMBOLS {
            symbols.insert(c, i);
            i += 1;
        }

        AlfSymbols {
            symbol_ids: symbols,
        }
    }

    pub fn symbol_id(&self, c: char) -> u8 {
        *self.symbol_ids.get(&c).expect("symbol not found")
    }
}

pub fn new_if_presudo_op(op: &str, line: &str) -> Option<MixalOp> {
    return match op {
        "EQU" => Some(MixalOp::new(op.to_string(), line.to_string())),
        "ORIG" => Some(MixalOp::new(op.to_string(), line.to_string())),
        "CON" => Some(MixalOp::new(op.to_string(), line.to_string())),
        "ALF" => Some(MixalOp::new(op.to_string(), line.to_string())),
        "END" => Some(MixalOp::new(op.to_string(), line.to_string())),
        _ => None,
    };
}
pub struct MixalOp {
    name: String,
    value: String,
}
impl MixalOp {
    pub fn new(name: String, value: String) -> MixalOp {
        MixalOp { name, value }
    }
    pub fn parse_address(&self) -> String {
        return match &self.name[..] {
            "ALF" => self.value[0..5].to_string(),
            _ => {
                let (op, _) = split_whitespace_once(&self.value[..]);
                op.to_string()
            }
        };
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn alf_to_num(&self) -> [u8; 5] {
        if self.name != "ALF" {
            panic!("can't convert not ALF");
        }
        let mut chars = self.value.chars();
        let alf = AlfSymbols::new();
        [
          alf.symbol_id(chars.next().expect("error")),
          alf.symbol_id(chars.next().expect("error")),
          alf.symbol_id(chars.next().expect("error")),
          alf.symbol_id(chars.next().expect("error")),
          alf.symbol_id(chars.next().expect("error")),
        ]
    }
}
impl Clone for MixalOp {
    fn clone(&self) -> MixalOp {
        MixalOp {
            name: self.name.clone(),
            value: self.value.clone(),
        }
    }
}
