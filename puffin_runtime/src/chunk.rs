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
        f.write_str(&ChunkFormatter::from(self).format_string())
    }
}

struct ChunkFormatter<'a> {
    chunk: &'a Chunk,
    inst: Vec<String>,
    idx: usize,
}

impl<'a> From<&'a Chunk> for ChunkFormatter<'a> {
    fn from(value: &'a Chunk) -> Self {
        ChunkFormatter {
            chunk: value,
            inst: vec![],
            idx: 0,
        }
    }
}

impl<'a> ChunkFormatter<'a> {
    fn format_string(mut self) -> String {
        while self.idx < self.chunk.byte_len() {
            let byte = self.chunk.bytes[self.idx];
            self.idx += 1;

            match OpCode::try_from_primitive(byte) {
                Ok(code) => match code {
                    OpCode::Invalid => self.push("invalid"),

                    // Stack
                    OpCode::Literal => self.push_with_literal("literal"),
                    OpCode::Pop => self.push("pop"),

                    OpCode::GetLocal => self.push_with_local_offset("getl"),
                    OpCode::SetLocal => self.push_with_local_offset("setl"),
                    OpCode::GetGlobal => self.push_with_literal("getg"),
                    OpCode::SetGlobal => self.push_with_literal("setg"),

                    // Object Manipulation

                    OpCode::NewObject => self.push("newobj"),
                    OpCode::GetField => self.push_with_local_offset("getf"),
                    OpCode::SetField => self.push_with_local_offset("setf"),

                    // Arithmetic
                    OpCode::Add => self.push("add"),
                    OpCode::Sub => self.push("sub"),
                    OpCode::Mul => self.push("mul"),
                    OpCode::Div => self.push("div"),
                    OpCode::Mod => self.push("mod"),
                    OpCode::Neg => self.push("neg"),
                    OpCode::Not => self.push("not"),
                    OpCode::Eq  => self.push("eq"),
                    OpCode::Neq => self.push("neq"),
                    OpCode::Lt  => self.push("lt"),
                    OpCode::Le  => self.push("le"),
                    OpCode::Gt  => self.push("gt"),
                    OpCode::Ge  => self.push("ge"),

                    // Branching
                    OpCode::Jump => self.push_with_instruction_offset("jmp"),
                    OpCode::JumpIf => self.push_with_instruction_offset("jmpi"),

                    // Terminal
                    OpCode::Exit => self.push("exit"),
                    OpCode::Poll => self.push("poll"),
                    OpCode::Render => self.push("render"),
                },
                Err(_) => self.inst.push(format!("{:<4x}| unknown [0x{:x}]", byte, self.idx)),
            }
        }

        let mut string = format!("Chunk '{}'", self.chunk.name);

        let literals = self.chunk.literals
            .iter()
            .enumerate()
            .map(|(i, literal)| format!("0x{i:x} {literal}"))
            .collect::<Vec<_>>();

        if !literals.is_empty() {
            string.push_str("\n== LITERALS ==\n");
            string.push_str(&literals.join("\n"));
        }

        if !self.inst.is_empty() {
            string.push_str("\n== INSTRUCTIONS ==\n");
            string.push_str(&self.inst.join("\n"));
        }

        string
    }
    
    fn push(&mut self, name: &'static str) {
        self.push_line(format!("{name}"));
    }
    
    fn push_with_literal(&mut self, name: &'static str) {
        if let Some(offset) = self.chunk.read_literal_offset(self.idx) {
            if let Some(value) = self.chunk.get_literal(offset as usize) {
                self.push_line(format!("{name} [0x{offset:x}] ({value})"));
                self.idx += size_of::<LiteralOffset>();
            } else {
                self.push_line(format!("{name} [0x{offset:x}]"));
            }
        } else {
            self.push_line(format!("{name} [MALFORMED]"));
        }
    }
    
    fn push_with_local_offset(&mut self, name: &'static str) {
        if let Some(offset) = self.chunk.read_local_offset(self.idx) {
            self.push_line(format!("{name} [0x{offset:x}]"));
            self.idx += size_of::<LocalOffset>();
        } else {
            self.push_line(format!("{name} [MALFORMED]"));
        }
    }

    fn push_with_instruction_offset(&mut self, name: &'static str) {
        if let Some(offset) = self.chunk.read_instruction_offset(self.idx) {
            self.push_line(format!("{name} [0x{offset:x}]"));
            self.idx += size_of::<InstructionOffset>();
        } else {
            self.push_line(format!("{name} [MALFORMED]"));
        }
    }

    fn push_line(&mut self, line: impl AsRef<str>) {
        self.inst.push(format!("0x{:<6x} | {}", self.idx, line.as_ref()));
    }
}
