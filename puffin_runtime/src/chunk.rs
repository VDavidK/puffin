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

    pub fn push_literal(&mut self, literal: Value) -> usize {
        let offset = self.new_literal(literal);
        self.push_op(OpCode::Literal);
        self.push_u64(offset as u64);
        offset
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

    pub fn new_literal(&mut self, literal: Value) -> usize {
        let offset = self.literals.len();
        self.literals.push(literal);
        offset
    }

    pub fn get_literal(&self, offset: usize) -> Option<&Value> {
        self.literals.get(offset)
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut idx = 0;

        let mut string = format!("Chunk '{}'", self.name);
        let mut literals = vec![];
        let mut inst = vec![];

        for (i, literal) in self.literals.iter().enumerate() {
            literals.push(format!("0x{i:x} {literal}"));
        }

        while idx < self.byte_len() {
            let byte = self.bytes[idx];
            idx += 1;

            match OpCode::try_from_primitive(byte) {
                Ok(code) => match code {
                    OpCode::Invalid => inst.push("| invalid".to_owned()),

                    // Stack
                    OpCode::Literal => {
                        if let Some(offset) = self.read_u64(idx) {
                            if let Some(value) = self.get_literal(offset as usize) {
                                inst.push(format!("| literal [0x{offset:x}] ({value})"));
                                idx += 8;
                            } else {
                                inst.push(format!("| literal [0x{offset:x}] (UNKNOWN)"));
                            }
                        } else {
                            inst.push("| literal [MALFORMED]".to_owned());
                        }
                    },

                    OpCode::GetLocal => {
                        if let Some(offset) = self.read_u64(idx) {
                            inst.push(format!("| getl [0x{offset:x}]"));
                            idx += 8;
                        } else {
                            inst.push("| getl [MALFORMED]".to_owned());
                        }
                    },

                    OpCode::SetLocal => {
                        if let Some(offset) = self.read_u64(idx) {
                            inst.push(format!("| setl [0x{offset:x}]"));
                            idx += 8;
                        } else {
                            inst.push("| setl [MALFORMED]".to_owned());
                        }
                    },

                    OpCode::GetGlobal => {
                        if let Some(offset) = self.read_u64(idx) {
                            inst.push(format!("| getg [{offset}]"));
                            idx += 8;
                        } else {
                            inst.push("| getg [MALFORMED]".to_owned());
                        }
                    },

                    OpCode::SetGlobal => {
                        if let Some(offset) = self.read_u64(idx) {
                            inst.push(format!("| setg [{offset}]"));
                            idx += 8;
                        } else {
                            inst.push("| setg [MALFORMED]".to_owned());
                        }
                    },

                    OpCode::Pop => inst.push("| pop".to_owned()),
                    

                    // Arithmetic
                    OpCode::Add => inst.push("| add".to_owned()),
                    OpCode::Sub => inst.push("| sub".to_owned()),
                    OpCode::Mul => inst.push("| mul".to_owned()),
                    OpCode::Div => inst.push("| div".to_owned()),
                    OpCode::Mod => inst.push("| mod".to_owned()),

                    // Terminal
                    OpCode::Poll => inst.push("| poll".to_owned()),
                    OpCode::Render => inst.push("| render".to_owned()),
                },
                Err(_) => inst.push(format!("\tunknown [0x{:x}]", byte)),
            }
        }

        if !literals.is_empty() {
            string.push_str("\n== LITERALS ==\n");
            string.push_str(&literals.join("\n"));
        }

        if !inst.is_empty() {
            string.push_str("\n== INSTRUCTIONS ==\n");
            string.push_str(&inst.join("\n"));
        }

        f.write_str(&string)
    }
}
