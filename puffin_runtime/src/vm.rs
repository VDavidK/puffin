use std::collections::HashMap;

use num_enum::TryFromPrimitive;
use ratatui::DefaultTerminal;

use crate::{RuntimeError, Value, chunk::Chunk, op::OpCode, value::new_object};


#[derive(Debug)]
pub struct Vm<'a> {
    chunk: &'a Chunk,
    stack: Vec<Value>,
    pc: usize,
    running: bool,
    globals: HashMap<String, Value>,
    term: DefaultTerminal,
}

impl<'a> Vm<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        let term = ratatui::init();

        Self {
            chunk,
            stack: vec![],
            pc: 0,
            running: true,
            globals: HashMap::new(),
            term,
        }
    }

    pub fn is_running(&self) -> bool {
        self.running && self.pc < self.chunk.byte_len()
    }

    pub fn execute(&mut self) -> Result<(), RuntimeError> {
        match self.fetch_op()? {
            OpCode::Invalid => return Err(RuntimeError::InvalidOpCode { pc: self.pc }),

            OpCode::Literal => {
                let literal = self.fetch_literal()?
                    .to_owned();
                self.push_value(literal);
            },

            OpCode::GetLocal => {
                let offset = self.fetch_u32()? as usize;
                let value = self.stack.get(offset)
                    .ok_or(RuntimeError::StackOutOfBounds { at: offset, pc: self.pc })?;

                self.stack.push(value.clone());
            },

            OpCode::SetLocal => {
                let top = self.pop_expecting()?;

                let offset = self.fetch_u32()? as usize;
                if offset >= self.stack.len() {
                    return Err(RuntimeError::StackOutOfBounds { at: offset, pc: self.pc });
                }

                self.stack[offset] = top;
            },

            OpCode::GetGlobal => {
                let literal = self.fetch_literal()?
                    .to_owned()
                    .as_string()?;

                let global = self.globals.get(&literal).ok_or(RuntimeError::GlobalNotFound { name: literal })?;
                self.push_value(global.clone());
            },

            OpCode::SetGlobal => {
                let literal = self.fetch_literal()?
                    .to_owned()
                    .as_string()?;

                let top = self.pop_expecting()?;

                self.globals.insert(literal, top);
            },

            OpCode::Pop => {
                self.pop_expecting()?;
            },

            OpCode::NewObject => {
                self.push_value(new_object());
            }

            OpCode::GetField => {
                let literal = self.fetch_literal()?
                    .to_owned()
                    .as_string()?;

                let obj = self.pop_expecting()?
                    .as_object()?;

                let obj = obj.borrow();

                let field = obj.get_field(&literal)
                    .ok_or(RuntimeError::NoFieldMatchingName { name: literal })?;

                self.push_value(field.to_owned());
            },

            OpCode::SetField => {
                let literal = self.fetch_literal()?
                    .to_owned()
                    .as_string()?;

                let value = self.pop_expecting()?;

                let obj = self.pop_expecting()?
                    .as_object()?;

                obj.borrow_mut().set_field(literal, value.to_owned());
            },

            OpCode::Add => {
                let rhs = self.pop_expecting()?;
                let lhs = self.pop_expecting()?;

                self.push_value(lhs.try_add(&rhs)?);
            },
            OpCode::Sub => {
                let rhs = self.pop_expecting()?;
                let lhs = self.pop_expecting()?;

                self.push_value(lhs.try_sub(&rhs)?);
            },
            OpCode::Mul => {
                let rhs = self.pop_expecting()?;
                let lhs = self.pop_expecting()?;

                self.push_value(lhs.try_mul(&rhs)?);
            },
            OpCode::Div => {
                let rhs = self.pop_expecting()?;
                let lhs = self.pop_expecting()?;

                self.push_value(lhs.try_div(&rhs)?);
            },
            OpCode::Mod => {
                let rhs = self.pop_expecting()?;
                let lhs = self.pop_expecting()?;

                self.push_value(lhs.try_mod(&rhs)?);
            },
            OpCode::Neg => {
                let rhs = self.pop_expecting()?;

                self.push_value(rhs.try_negate()?);
            },
            OpCode::Not => {
                let rhs = self.pop_expecting()?;

                self.push_value(rhs.not());
            },

            OpCode::Jump => {
                let addr = self.fetch_u64()?;

                self.pc = addr as usize;
            },

            OpCode::JumpIf => {
                let addr = self.fetch_u64()?;
                let val = self.pop_expecting()?;

                if val.truthy() {
                    self.pc = addr as usize;
                }
            },

            OpCode::Poll => {
                if ratatui::crossterm::event::read()?.is_key_press() {
                    // self.running = false;
                }
            },
            
            OpCode::Render => {
                let value = self.pop_expecting()?;
                self.term.draw(|frame| frame.render_widget(value.to_string(), frame.area()))?;
            }
        }

        Ok(())
    }

    pub fn fetch_op(&mut self) -> Result<OpCode, RuntimeError> {
        let byte = self.chunk
            .read_u8(self.pc)
            .ok_or(RuntimeError::AccessOutOfBounds { at: self.pc, pc: self.pc })?;

        self.pc += 1;

        OpCode::try_from_primitive(byte)
            .map_err(|_| RuntimeError::UnrecognizedOpCode { op: byte, pc: self.pc })
    }

    pub fn fetch_u8(&mut self) -> Result<u8, RuntimeError> {
        let byte =  self.chunk.read_u8(self.pc).ok_or(RuntimeError::AccessOutOfBounds { at: self.pc, pc: self.pc })?;
        self.pc += 1;
        Ok(byte)
    }

    pub fn fetch_u16(&mut self) -> Result<u16, RuntimeError> {
        let val =  self.chunk.read_u16(self.pc).ok_or(RuntimeError::AccessOutOfBounds { at: self.pc, pc: self.pc })?;
        self.pc += 2;
        Ok(val)
    }

    pub fn fetch_u32(&mut self) -> Result<u32, RuntimeError> {
        let val =  self.chunk.read_u32(self.pc).ok_or(RuntimeError::AccessOutOfBounds { at: self.pc, pc: self.pc })?;
        self.pc += 4;
        Ok(val)
    }

    pub fn fetch_u64(&mut self) -> Result<u64, RuntimeError> {
        let val =  self.chunk.read_u64(self.pc).ok_or(RuntimeError::AccessOutOfBounds { at: self.pc, pc: self.pc })?;
        self.pc += 8;
        Ok(val)
    }

    pub fn fetch_literal(&mut self) -> Result<&Value, RuntimeError> {
        let offset = self.fetch_u32()? as usize;
        let value = self.chunk.get_literal(offset)
            .ok_or(RuntimeError::InvalidLiteralAccess { at: offset, pc: self.pc })?;

        Ok(value)
    }

    pub fn push_value<T: Into<Value>>(&mut self, value: T) {
        self.stack.push(value.into());
    }

    pub fn pop_value(&mut self) -> Option<Value> {
        self.stack.pop()
    }

    pub fn peek_value(&mut self) -> Option<&Value> {
        self.stack.last()
    }

    pub fn pop_expecting(&mut self) -> Result<Value, RuntimeError> {
        self.stack.pop().ok_or(RuntimeError::StackEmpty)
    }
}

impl<'a> Drop for Vm<'a> {
    fn drop(&mut self) {
        ratatui::restore();
    }
}
