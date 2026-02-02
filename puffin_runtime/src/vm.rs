use num_enum::TryFromPrimitive;

use crate::{RuntimeError, Value, op::OpCode, chunk::Chunk};


#[derive(Debug, Clone)]
pub struct Vm<'a> {
    chunk: &'a Chunk,
    stack: Vec<Value>,
    pc: usize,
}

impl<'a> Vm<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        Self {
            chunk,
            stack: vec![],
            pc: 0,
        }
    }

    pub fn is_running(&self) -> bool {
        self.pc < self.chunk.byte_len()
    }

    pub fn execute(&mut self) -> Result<(), RuntimeError> {
        match self.fetch_op()? {
            OpCode::Invalid => return Err(RuntimeError::InvalidOpCode(self.pc)),

            OpCode::Literal => {
                let offset = self.fetch_u64()? as usize;
                let value = self.chunk.get_literal(offset)
                    .ok_or(RuntimeError::InvalidLiteralAccess(offset, self.pc))?;

                self.push_value(value.clone());
            },

            OpCode::Print => {
                if let Some(value )= self.pop_value() {
                    println!("{value}");
                }
            },

            OpCode::Add => todo!(),
            OpCode::Sub => todo!(),
            OpCode::Mul => todo!(),
            OpCode::Div => todo!(),
            OpCode::Mod => todo!(),
        }

        Ok(())
    }

    pub fn fetch_op(&mut self) -> Result<OpCode, RuntimeError> {
        let byte = self.chunk
            .read_u8(self.pc)
            .ok_or(RuntimeError::AccessOutOfBounds(self.pc, self.pc))?;

        self.pc += 1;

        OpCode::try_from_primitive(byte)
            .map_err(|_| RuntimeError::UnrecognizedOpCode(byte, self.pc))
    }

    pub fn fetch_u8(&mut self) -> Result<u8, RuntimeError> {
        let byte =  self.chunk.read_u8(self.pc).ok_or(RuntimeError::AccessOutOfBounds(self.pc, self.pc))?;
        self.pc += 1;
        Ok(byte)
    }

    pub fn fetch_u16(&mut self) -> Result<u16, RuntimeError> {
        let val =  self.chunk.read_u16(self.pc).ok_or(RuntimeError::AccessOutOfBounds(self.pc, self.pc))?;
        self.pc += 2;
        Ok(val)
    }

    pub fn fetch_u32(&mut self) -> Result<u32, RuntimeError> {
        let val =  self.chunk.read_u32(self.pc).ok_or(RuntimeError::AccessOutOfBounds(self.pc, self.pc))?;
        self.pc += 4;
        Ok(val)
    }

    pub fn fetch_u64(&mut self) -> Result<u64, RuntimeError> {
        let val =  self.chunk.read_u64(self.pc).ok_or(RuntimeError::AccessOutOfBounds(self.pc, self.pc))?;
        self.pc += 8;
        Ok(val)
    }

    pub fn push_value<T: Into<Value>>(&mut self, value: T) {
        self.stack.push(value.into());
    }

    pub fn pop_value(&mut self) -> Option<Value> {
        self.stack.pop()
    }
}

