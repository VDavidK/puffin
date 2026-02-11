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

pub type LiteralOffset = u32;
pub type LocalOffset = u32;
pub type InstructionOffset = u32;

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

    pub fn push_literal(&mut self, literal: impl Into<Value>) -> LiteralOffset {
        let offset = self.new_literal(literal.into()) as LiteralOffset;
        self.push_op(OpCode::Literal);
        self.push_literal_offset(offset);
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
    
    pub fn push_literal_offset(&mut self, val: LiteralOffset) {
        self.bytes.extend_from_slice(&val.to_le_bytes());
    }
    
    pub fn push_local_offset(&mut self, val: LocalOffset) {
        self.bytes.extend_from_slice(&val.to_le_bytes());
    }
    
    pub fn push_instruction_offset(&mut self, val: InstructionOffset) {
        self.bytes.extend_from_slice(&val.to_le_bytes());
    }
    
    pub fn push_jump(&mut self, op: OpCode) -> InstructionOffset {
        self.push_op(op);
        self.push_instruction_offset(0xFFFFFFFF);
        self.addr() - 4
    }
    
    pub fn push_jump_im(&mut self, op: OpCode, to: InstructionOffset) -> InstructionOffset {
        self.push_op(op);
        self.push_instruction_offset(to);
        self.addr() - 4
    }
    
    pub fn patch_jump(&mut self, jump: InstructionOffset, offset: InstructionOffset) {
        let bytes = offset.to_le_bytes();
        for (i, byte) in bytes.iter().enumerate() {
            self.bytes[i + jump as usize] = *byte;
        }
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
    
    pub fn read_literal_offset(&self, idx: usize) -> Option<LiteralOffset> {
        Some(LiteralOffset::from_le_bytes(<[u8;size_of::<LiteralOffset>()]>::try_from(&self.bytes[idx..idx+size_of::<LiteralOffset>()]).ok()?))
    }
    
    pub fn read_local_offset(&self, idx: usize) -> Option<LocalOffset> {
        Some(LocalOffset::from_le_bytes(<[u8;size_of::<LocalOffset>()]>::try_from(&self.bytes[idx..idx+size_of::<LocalOffset>()]).ok()?))
    }
    
    pub fn read_instruction_offset(&self, idx: usize) -> Option<InstructionOffset> {
        Some(InstructionOffset::from_le_bytes(<[u8;size_of::<InstructionOffset>()]>::try_from(&self.bytes[idx..idx+size_of::<InstructionOffset>()]).ok()?))
    }

    pub fn addr(&self) -> InstructionOffset {
        self.bytes.len() as InstructionOffset
    }

    pub fn new_literal(&mut self, literal: impl Into<Value>) -> LiteralOffset {
        let offset = self.literals.len();
        self.literals.push(literal.into());
        offset as LiteralOffset
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
                    OpCode::Invalid => inst.push(format!("{idx:<4x} | invalid")),

                    // Stack
                    OpCode::Literal => {
                        if let Some(offset) = self.read_literal_offset(idx) {
                            if let Some(value) = self.get_literal(offset as usize) {
                                inst.push(format!("{idx:<4x} | literal [0x{offset:x}] ({value})"));
                                idx += size_of::<LiteralOffset>();
                            } else {
                                inst.push(format!("{idx:<4x} | literal [0x{offset:x}] (UNKNOWN)"));
                            }
                        } else {
                            inst.push(format!("{idx:<4x} | literal [MALFORMED]"));
                        }
                    },

                    OpCode::GetLocal => {
                        if let Some(offset) = self.read_local_offset(idx) {
                            inst.push(format!("{idx:<4x} | getl [0x{offset:x}]"));
                            idx += size_of::<LocalOffset>();
                        } else {
                            inst.push(format!("{idx:<4x} | getl [MALFORMED]"));
                        }
                    },

                    OpCode::SetLocal => {
                        if let Some(offset) = self.read_local_offset(idx) {
                            inst.push(format!("{idx:<4x} | setl [0x{offset:x}]"));
                            idx += size_of::<LocalOffset>();
                        } else {
                            inst.push(format!("{idx:<4x} | setl [MALFORMED]"));
                        }
                    },

                    OpCode::GetGlobal => {
                        if let Some(offset) = self.read_literal_offset(idx) {
                            if let Some(value) = self.get_literal(offset as usize) {
                                inst.push(format!("{idx:<4x} | getg [0x{offset:x}] ({value})"));
                                idx += size_of::<LiteralOffset>();
                            } else {
                                inst.push(format!("{idx:<4x} | getg [0x{offset:x}] (UNKNOWN)"));
                            }
                        } else {
                            inst.push(format!("{idx:<4x} | getg [MALFORMED]"));
                        }
                    },

                    OpCode::SetGlobal => {
                        if let Some(offset) = self.read_literal_offset(idx) {
                            if let Some(value) = self.get_literal(offset as usize) {
                                inst.push(format!("{idx:<4x} | setg [0x{offset:x}] ({value})"));
                                idx += size_of::<LiteralOffset>();
                            } else {
                                inst.push(format!("{idx:<4x} | setg [0x{offset:x}] (UNKNOWN)"));
                            }
                        } else {
                            inst.push(format!("{idx:<4x} | setg [MALFORMED]"));
                        }
                    },

                    OpCode::Pop => inst.push(format!("{idx:<4x} | pop")),
                    
                    // Object Manipulation

                    OpCode::NewObject => inst.push(format!("{idx:<4x} | newobj")),

                    OpCode::GetField => {
                        if let Some(offset) = self.read_local_offset(idx) {
                            inst.push(format!("{idx:<4x} | getf (s:0x{offset:x})"));
                            idx += size_of::<LocalOffset>();
                        } else {
                            inst.push(format!("{idx:<4x} | getf [MALFORMED]"));
                        }
                    },

                    OpCode::SetField => {
                        if let Some(offset) = self.read_local_offset(idx) {
                            inst.push(format!("{idx:<4x} | setf [s:0x{offset:x}]"));
                            idx += size_of::<LocalOffset>();
                        } else {
                            inst.push(format!("{idx:<4x} | setf [MALFORMED]"));
                        }
                    },

                    // Arithmetic
                    OpCode::Add => inst.push(format!("{idx:<4x} | add")),
                    OpCode::Sub => inst.push(format!("{idx:<4x} | sub")),
                    OpCode::Mul => inst.push(format!("{idx:<4x} | mul")),
                    OpCode::Div => inst.push(format!("{idx:<4x} | div")),
                    OpCode::Mod => inst.push(format!("{idx:<4x} | mod")),
                    OpCode::Neg => inst.push(format!("{idx:<4x} | neg")),
                    OpCode::Not => inst.push(format!("{idx:<4x} | not")),
                    OpCode::Eq => inst.push(format!("{idx:<4x} | eq")),
                    OpCode::Neq => inst.push(format!("{idx:<4x} | neq")),
                    OpCode::Lt => inst.push(format!("{idx:<4x} | lt")),
                    OpCode::Le => inst.push(format!("{idx:<4x} | le")),
                    OpCode::Gt => inst.push(format!("{idx:<4x} | gt")),
                    OpCode::Ge => inst.push(format!("{idx:<4x} | ge")),

                    // Branching
                    OpCode::Jump => {
                        if let Some(addr) = self.read_instruction_offset(idx) {
                            inst.push(format!("{idx:<4x} | jmp [0x{addr:x}]"));
                            idx += size_of::<InstructionOffset>();
                        } else {
                            inst.push(format!("{idx:<4x} | jmp [MALFORMED]"));
                        }
                    },
                    OpCode::JumpIf => {
                        if let Some(addr) = self.read_instruction_offset(idx) {
                            inst.push(format!("{idx:<4x} | jmpi [0x{addr:x}]"));
                            idx += size_of::<InstructionOffset>();
                        } else {
                            inst.push(format!("{idx:<4x} | jmpi [MALFORMED]"));
                        }
                    },

                    // Terminal
                    OpCode::Poll => inst.push(format!("{idx:<4x} | poll")),
                    OpCode::Render => inst.push(format!("{idx:<4x} | render")),
                },
                Err(_) => inst.push(format!("{idx:<4x}| unknown [0x{:x}]", byte)),
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
