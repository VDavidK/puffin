use std::fmt::Display;

use crate::{Value, op::OpCode};


#[derive(Debug, Clone)]
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
        let string = format!("{}", self.name);
        f.write_str(&string)
    }
}
