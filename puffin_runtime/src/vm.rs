use std::collections::HashMap;

use num_enum::TryFromPrimitive;
use ratatui::DefaultTerminal;

use crate::{RuntimeError, Value, chunk::Chunk, op::OpCode, value::new_object};
use crate::chunk::{InstructionOffset, ConstantOffset, LocalOffset};

#[derive(Debug)]
struct CallFrame {
    pub return_addr: InstructionOffset,
    pub stack_offset: usize,
    pub local_count: usize,
}

#[derive(Debug)]
pub struct Vm<'a> {
    chunk: &'a Chunk,
    stack: Vec<Value>,
    pc: usize,
    running: bool,
    globals: HashMap<String, Value>,
    call_stack: Vec<CallFrame>,
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
            call_stack: vec![],
            term,
        }
    }

    pub fn is_running(&self) -> bool {
        self.running && self.pc < self.chunk.byte_len()
    }

    pub fn execute(&mut self) -> Result<(), RuntimeError> {
        let op = self.fetch_op()?;
        
        #[cfg(feature = "debug_tracing")]
        {
            let mut values = self.stack.iter().map(|v| v.to_string()).collect::<Vec<_>>().join("] [");
            if !values.is_empty() {
                values = format!("[{values}]");
            }
            
            log::debug!("| {values}");
            log::debug!("Executing: {op:?}");
        }
        
        match op {
            OpCode::Invalid => return Err(RuntimeError::InvalidOpCode { pc: self.pc }),

            OpCode::Constant => {
                let constant = self.fetch_constant()?
                    .to_owned();
                self.push_value(constant);
            },

            OpCode::GetLocal => {
                let offset = self.fetch_local_offset()?;
                let value = self.get_local(offset)?;

                self.push_value(value.clone());
            },

            OpCode::SetLocal => {
                let top = self.pop_expecting()?;
                let offset = self.fetch_local_offset()?;
                self.set_local(offset, top)?;
            },

            OpCode::GetGlobal => {
                let constant = self.fetch_constant()?
                    .to_owned()
                    .take_string()?;

                let global = self.globals.get(&constant).ok_or(RuntimeError::GlobalNotFound { name: constant })?;
                self.push_value(global.clone());
            },

            OpCode::SetGlobal => {
                let constant = self.fetch_constant()?
                    .to_owned()
                    .take_string()?;

                let top = self.pop_expecting()?;

                self.globals.insert(constant, top);
            },

            OpCode::Pop => {
                self.pop_expecting()?;
            },

            OpCode::NewObject => {
                self.push_value(new_object());
            }

            OpCode::GetField => {
                let name = self.pop_expecting()?.take_string()?;
                
                let obj_offset = self.fetch_local_offset()?;
                let obj = self.get_local(obj_offset)?.clone().take_object()?;
                let obj = obj.borrow();

                let field = obj.get_field(&name)
                    .ok_or(RuntimeError::NoFieldMatchingName { name: name })?;

                self.push_value(field.to_owned());
            },

            OpCode::SetField => {
                let value = self.pop_expecting()?;
                let name = self.pop_expecting()?;

                let obj_offset = self.fetch_local_offset()?;
                let obj = self.get_local(obj_offset)?.clone().take_object()?;

                obj.borrow_mut().set_field(name.take_string()?, value.to_owned());
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
            
            OpCode::Eq => {
                let rhs = self.pop_expecting()?;
                let lhs = self.pop_expecting()?;
                
                self.push_value(lhs.is_equal(&rhs));
            },
            OpCode::Neq => {
                let rhs = self.pop_expecting()?;
                let lhs = self.pop_expecting()?;

                self.push_value(lhs.not_equal(&rhs));
            },
            
            OpCode::Lt => {
                let rhs = self.pop_expecting()?;
                let lhs = self.pop_expecting()?;

                self.push_value(lhs.lesser(&rhs));
            },
            OpCode::Le => {
                let rhs = self.pop_expecting()?;
                let lhs = self.pop_expecting()?;

                self.push_value(lhs.lesser_equal(&rhs));
            },
            
            OpCode::Gt => {
                let rhs = self.pop_expecting()?;
                let lhs = self.pop_expecting()?;

                self.push_value(lhs.greater(&rhs));
            },
            OpCode::Ge => {
                let rhs = self.pop_expecting()?;
                let lhs = self.pop_expecting()?;

                self.push_value(lhs.greater_equal(&rhs));
            },

            OpCode::Jump => {
                let addr = self.fetch_instruction_offset()?;

                self.pc = addr as usize;
            },

            OpCode::JumpIf => {
                let addr = self.fetch_instruction_offset()?;
                let val = self.pop_expecting()?;

                if val.truthy() {
                    #[cfg(feature = "debug_tracing")]
                    log::debug!("Value '{}' truthy, jumping from 0x{:x} -> 0x{:x}", val, self.pc, addr);
                    self.pc = addr as usize;
                }
                else {
                    #[cfg(feature = "debug_tracing")]
                    log::debug!("Skipping jump at 0x{:x}", self.pc);
                }
            },

            OpCode::Call => {
                let arg_count = self.fetch_u8()?;
                let addr = self.fetch_instruction_offset()?;

                if let Some(frame) = self.call_stack.last_mut() {
                    frame.local_count -= arg_count as usize;
                }

                self.call_stack.push(self.new_call_frame(arg_count as usize));
                self.pc = addr as usize;
            },

            OpCode::Return => {
                if let Some(frame) = self.call_stack.pop() {
                    self.pc = frame.return_addr as usize;
                    let ret_value = self.pop_expecting()?;

                    // Pop all values pushed in the current call frame
                    for _ in 0..frame.local_count - 1 {
                        self.pop_expecting()?;
                    }

                    self.push_value(ret_value);
                } else {
                    self.running = false;
                }
            },

            OpCode::Exit => {
                self.running = false;
            }

            OpCode::Poll => {
                ratatui::crossterm::event::read()?;
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
    
    pub fn fetch_constant_offset(&mut self) -> Result<ConstantOffset, RuntimeError> {
        let val =  self.chunk.read_constant_offset(self.pc).ok_or(RuntimeError::AccessOutOfBounds { at: self.pc, pc: self.pc })?;
        self.pc += size_of::<ConstantOffset>();
        Ok(val)
    }

    pub fn fetch_local_offset(&mut self) -> Result<LocalOffset, RuntimeError> {
        let val =  self.chunk.read_local_offset(self.pc).ok_or(RuntimeError::AccessOutOfBounds { at: self.pc, pc: self.pc })?;
        self.pc += size_of::<LocalOffset>();
        Ok(val)
    }
    
    pub fn fetch_instruction_offset(&mut self) -> Result<InstructionOffset, RuntimeError> {
        let val =  self.chunk.read_instruction_offset(self.pc).ok_or(RuntimeError::AccessOutOfBounds { at: self.pc, pc: self.pc })?;
        self.pc += size_of::<InstructionOffset>();
        Ok(val)
    }

    pub fn fetch_constant(&mut self) -> Result<&Value, RuntimeError> {
        let offset = self.fetch_constant_offset()? as usize;
        let value = self.chunk.get_constant(offset)
            .ok_or(RuntimeError::InvalidConstantAccess { at: offset, pc: self.pc })?;

        Ok(value)
    }
    
    pub fn get_local(&self, offset: LocalOffset) -> Result<&Value, RuntimeError> {
        let value = self.stack.get(offset as usize + self.stack_offset())
            .ok_or(RuntimeError::StackOutOfBounds { at: offset as usize, pc: self.pc })?;
        
        Ok(value)
    }

    pub fn set_local(&mut self, offset: LocalOffset, value: Value) -> Result<(), RuntimeError> {
        let offset = offset as usize + self.stack_offset();

        if offset >= self.stack.len() {
            return Err(RuntimeError::StackOutOfBounds { at: offset, pc: self.pc });
        }

        self.stack[offset] = value;
        Ok(())
    }

    pub fn push_value<T: Into<Value>>(&mut self, value: T) {
        self.stack.push(value.into());

        if let Some(frame) = self.call_stack.last_mut() {
            frame.local_count += 1;
        }
    }

    pub fn pop_value(&mut self) -> Option<Value> {
        let val = self.stack.pop();

        if let Some(frame) = self.call_stack.last_mut() && val.is_some() {
            frame.local_count -= 1;
        }

        val
    }

    pub fn peek_value(&mut self) -> Option<&Value> {
        self.stack.last()
    }

    pub fn pop_expecting(&mut self) -> Result<Value, RuntimeError> {
        let val = self.stack.pop().ok_or(RuntimeError::StackEmpty)?;

        if let Some(frame) = self.call_stack.last_mut() {
            frame.local_count -= 1;
        }

        Ok(val)
    }

    fn stack_offset(&self) -> usize {
        self.call_stack.last()
            .map(|frame| frame.stack_offset)
            .unwrap_or(0)
    }

    fn new_call_frame(&self, arg_count: usize) -> CallFrame {
        CallFrame {
            return_addr: self.pc as InstructionOffset,
            stack_offset: self.stack.len() - arg_count,
            local_count: arg_count,
        }
    }
}

impl<'a> Drop for Vm<'a> {
    fn drop(&mut self) {
        ratatui::restore();
    }
}
