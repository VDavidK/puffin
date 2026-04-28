use std::fmt::Display;

use num_enum::TryFromPrimitive;
use serde_derive::{Deserialize, Serialize};

use crate::{value::Value, op::OpCode};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    name: String,
    bytes: Vec<u8>,
    constants: Vec<Value>,
}

pub type ConstantOffset = u32;
pub type LocalOffset = i32;
pub type InstructionOffset = u32;

impl Chunk {
    pub fn new(name: impl AsRef<str>) -> Self {
        Self {
            name: name.as_ref().to_owned(),
            bytes: vec![],
            constants: vec![],
        }
    }
    
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn byte_len(&self) -> usize {
        self.bytes.len()
    }

    pub fn push_op(&mut self, op: OpCode) {
        self.bytes.push(op.into())
    }

    pub fn push_constant(&mut self, constant: impl Into<Value>) -> ConstantOffset {
        let offset = self.new_constant(constant.into()) as ConstantOffset;
        self.push_op(OpCode::Constant);
        self.push_constant_offset(offset);
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
    
    pub fn push_constant_offset(&mut self, val: ConstantOffset) {
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

    pub fn push_call_im(&mut self, arg_count: u8, to: InstructionOffset) -> InstructionOffset {
        self.push_op(OpCode::Call);
        self.push_u8(arg_count);
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
    
    pub fn read_constant_offset(&self, idx: usize) -> Option<ConstantOffset> {
        Some(ConstantOffset::from_le_bytes(<[u8;size_of::<ConstantOffset>()]>::try_from(&self.bytes[idx..idx+size_of::<ConstantOffset>()]).ok()?))
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

    pub fn new_constant(&mut self, constant: impl Into<Value>) -> ConstantOffset {
        let offset = self.constants.len();
        self.constants.push(constant.into());
        offset as ConstantOffset
    }

    pub fn get_constant(&self, offset: usize) -> Option<&Value> {
        self.constants.get(offset)
    }

    pub fn last_op_is(&self, matches: OpCode) -> bool {
        matches!(self.bytes.last(), Some(&op) if op == matches.into())
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
    indent: usize,
}

impl<'a> From<&'a Chunk> for ChunkFormatter<'a> {
    fn from(value: &'a Chunk) -> Self {
        ChunkFormatter {
            chunk: value,
            inst: vec![],
            idx: 0,
            indent: 0,
        }
    }
}

impl<'a> ChunkFormatter<'a> {
    fn format_string(mut self) -> String {
        while self.idx < self.chunk.byte_len() {
            let byte = self.chunk.bytes[self.idx];

            match OpCode::try_from_primitive(byte) {
                Ok(code) => match code {
                    OpCode::Invalid => self.push("invalid"),

                    // Stack
                    OpCode::Constant => self.push_with_constant("const"),
                    OpCode::Pop => self.push("pop"),

                    OpCode::GetLocal => self.push_with_local_offset("getl"),
                    OpCode::SetLocal => self.push_with_local_offset("setl"),
                    OpCode::GetGlobal => self.push_with_constant("getg"),
                    OpCode::SetGlobal => self.push_with_constant("setg"),

                    // Object Manipulation

                    OpCode::NewInstance => self.push_with_local_offset("newobj"),
                    OpCode::NewClass => self.push_with_constant("class"),
                    OpCode::SetConstructor => self.push("scons"),
                    OpCode::GetField => self.push_with_local_offset("getf"),
                    OpCode::SetField => self.push_with_local_offset("setf"),
                    OpCode::NewList => self.push("newlist"),
                    OpCode::PushList => self.push("pushlist"),
                    OpCode::PopList => self.push("poplist"),

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
                    OpCode::Call => self.push_with_u8("call"),
                    OpCode::Return  => self.push("return"),

                    // Terminal
                    OpCode::Exit => self.push("exit"),
                    OpCode::Poll => self.push("poll"),
                    OpCode::Render => self.push("render"),

                    // Layout
                    OpCode::SetRoot => self.push("setroot"),
                },
                Err(_) => self.inst.push(format!("{:<4x}| unknown [0x{:x}]", byte, self.idx)),
            }
            self.idx += 1;
        }

        let indent = self.get_indent();

        let mut string = format!("{indent}Chunk '{}'", self.chunk.name);
        let mut inner_functions = vec![];

        let constants = self.chunk.constants
            .iter()
            .enumerate()
            .map(|(i, constant)| {
                if let Value::Function(func) = &constant {
                    inner_functions.push(func.clone());
                }

                format!("{indent}0x{i:x} {constant}")
            })
            .collect::<Vec<_>>();

        if !constants.is_empty() {
            string.push_str(&format!("\n{indent}== CONSTANTS ==\n"));
            string.push_str(&constants.join("\n"));
        }

        if !self.inst.is_empty() {
            string.push_str(&format!("\n{indent}== INSTRUCTIONS ==\n"));
            string.push_str(&self.inst.join("\n"));
        }

        if !inner_functions.is_empty() {
            string.push_str(&format!("\n{indent}== FUNCTION CONSTANTS ==\n"));

            for func in inner_functions {
                let formatter = ChunkFormatter {
                    chunk: &func.chunk,
                    inst: vec![],
                    idx: 0,
                    indent: self.indent + 1,
                };
                string.push_str(&format!("{}\n", formatter.format_string()));
            }
        }

        string
    }
    
    fn push(&mut self, name: &'static str) {
        self.push_line(name);
    }
    
    fn push_with_constant(&mut self, name: &'static str) {
        if let Some(offset) = self.chunk.read_constant_offset(self.idx + 1) {
            if let Some(value) = self.chunk.get_constant(offset as usize) {
                self.push_line(format!("{name} [0x{offset:x}] ({value})"));
                self.idx += size_of::<ConstantOffset>();
            } else {
                self.push_line(format!("{name} [0x{offset:x}] (CONSTANT NOT FOUND)"));
            }
        } else {
            self.push_line(format!("{name} [MALFORMED]"));
        }
    }
    
    fn push_with_local_offset(&mut self, name: &'static str) {
        if let Some(offset) = self.chunk.read_local_offset(self.idx + 1) {
            self.push_line(format!("{name} [0x{offset:x}]"));
            self.idx += size_of::<LocalOffset>();
        } else {
            self.push_line(format!("{name} [MALFORMED]"));
        }
    }

    fn push_with_instruction_offset(&mut self, name: &'static str) {
        if let Some(offset) = self.chunk.read_instruction_offset(self.idx + 1) {
            self.push_line(format!("{name} [0x{offset:x}]"));
            self.idx += size_of::<InstructionOffset>();
        } else {
            self.push_line(format!("{name} [MALFORMED]"));
        }
    }

    fn push_with_u8(&mut self, name: &'static str) {
        if let Some(val) = self.chunk.read_u8(self.idx + 1) {
            self.push_line(format!("{name} [{val:x}]"));
            self.idx += size_of::<u8>();
        } else {
            self.push_line(format!("{name} [MALFORMED]"));
        }
    }

    fn push_line(&mut self, line: impl AsRef<str>) {
        self.inst.push(format!("{}0x{:<6x} | {}", self.get_indent(), self.idx, line.as_ref()));
    }

    fn get_indent(&self) -> String {
        ">\t".repeat(self.indent)
    }
}
