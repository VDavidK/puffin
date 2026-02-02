use std::fmt::Display;

use num_enum::TryFromPrimitive;
use serde_derive::{Deserialize, Serialize};

use crate::{Value, op::OpCode};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    name: String,
    bytes: Vec<u8>,
    literals: Vec<Value>,
}

impl Chunk {
    pub fn new(name: impl AsRef<str>) -> Self {
        Self {
            name: name.as_ref().to_owned(),
            bytes: vec![],
            literals: vec![],
        }
    }

    pub fn byte_len(&self) -> usize {
        self.bytes.len()
    }

    pub fn push_op(&mut self, op: OpCode) {
        self.bytes.push(op.into())
    }

    pub fn push_literal(&mut self, literal: Value) {
        let offset = self.literals.len();
        self.literals.push(literal);
        self.push_op(OpCode::Literal);
        self.push_u64(offset as u64);
    }

    pub fn push_u8(&mut self, val: u8) {
        self.bytes.push(val);
    }

    pub fn push_u16(&mut self, val: u16) {
        self.bytes.extend_from_slice(&val.to_le_bytes());
    }

    pub fn push_u32(&mut self, val: u32) {
        self.bytes.extend_from_slice(&val.to_le_bytes());
    }

    pub fn push_u64(&mut self, val: u64) {
        self.bytes.extend_from_slice(&val.to_le_bytes());
    }

    pub fn read_u8(&self, idx: usize) -> Option<u8> {
        self.bytes.get(idx).copied()
    }

    pub fn read_u16(&self, idx: usize) -> Option<u16> {
        Some(u16::from_le_bytes(<[u8;2]>::try_from(&self.bytes[idx..idx+2]).ok()?))
    }

    pub fn read_u32(&self, idx: usize) -> Option<u32> {
        Some(u32::from_le_bytes(<[u8;4]>::try_from(&self.bytes[idx..idx+4]).ok()?))
    }

    pub fn read_u64(&self, idx: usize) -> Option<u64> {
        Some(u64::from_le_bytes(<[u8;8]>::try_from(&self.bytes[idx..idx+8]).ok()?))
    }

    pub fn get_literal(&self, offset: usize) -> Option<&Value> {
        self.literals.get(offset)
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut idx = 0;

        let mut string = format!("Chunk '{}'", self.name);
        let mut lines = vec![];

        while idx < self.byte_len() {
            let byte = self.read_u8(idx).unwrap();
            idx += 1;

            match OpCode::try_from_primitive(byte) {
                Ok(code) => match code {
                    OpCode::Invalid => lines.push("\tinvalid".to_owned()),
                    OpCode::Literal => {
                        if let Some(offset) = self.read_u64(idx) {
                            if let Some(value) = self.get_literal(offset as usize) {
                                lines.push(format!("\tliteral [{value}]"));
                                idx += 8;
                            } else {
                                lines.push(format!("\tliteral [UNKNOWN:0x{:x}]", offset));
                            }
                        } else {
                            lines.push("\tliteral [MALFORMED]".to_owned());
                        }
                    },
                    OpCode::Print => lines.push("\tprint".to_owned()),
                    OpCode::Add => lines.push("\tadd".to_owned()),
                    OpCode::Sub => lines.push("\tsub".to_owned()),
                    OpCode::Mul => lines.push("\tmul".to_owned()),
                    OpCode::Div => lines.push("\tdiv".to_owned()),
                    OpCode::Mod => lines.push("\tmod".to_owned()),
                },
                Err(_) => lines.push(format!("\tunknown [0x{:x}]", byte)),
            }
        }

        if lines.len() > 0 {
            string.push('\n');
            string.push_str(&lines.join("\n"));
        }

        f.write_str(&string)
    }
}
