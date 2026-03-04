use std::rc::Rc;
use num_enum::TryFromPrimitive;

use crate::{RuntimeError, Value, chunk::Chunk, op::OpCode, value::new_object};
use crate::chunk::{InstructionOffset, ConstantOffset, LocalOffset};
use crate::library::Library;
use crate::runtime::{CallFrame, Runtime};
use crate::value::FunctionType;

#[derive(Debug)]
pub struct Vm {
    running: bool,
    runtime: Runtime,
}

impl Vm {
    pub fn new(chunk: Rc<Chunk>) -> Self {
        let main_frame = CallFrame {
            chunk,
            stack_offset: 0,
            local_count: 0,
            pc: 0,
        };

        Self {
            runtime: Runtime::new(main_frame),
            running: true,
        }
    }

    pub fn open_lib<T: Library>(&mut self) {
        let lib = new_object();
        T::create(lib.borrow_mut());
        self.add_global(T::name(), lib);
    }

    pub fn run(&mut self) -> Result<(), RuntimeError> {
        #[cfg(feature = "debug_tracing")]
        log::debug!("Starting execution");

        while self.is_running() {
            match self.execute() {
                Err(err) => {
                    log::error!("Runtime error occurred: {}", err);
                    return Err(err);
                }
                _ => (),
            }
        }

        log::debug!("Execution finished without errors");

        Ok(())
    }

    pub fn call(&mut self, func: FunctionType) -> Result<(), RuntimeError> {
        log::debug!("Executing function '{}'", func.identifier);
        self.runtime.call(func.chunk.clone(), func.arity);
        self.run()?;

        Ok(())
    }

    pub fn is_running(&self) -> bool {
        self.running && !self.runtime.call_stack_empty() && self.runtime.pc().unwrap() < self.runtime.chunk().unwrap().byte_len()
    }

    pub fn execute(&mut self) -> Result<(), RuntimeError> {
        let op = self.fetch_op()?;

        #[cfg(feature = "debug_tracing")]
        {
            self.runtime.log_stack();
            log::debug!("Executing: {op:?}");
        }
        
        match op {
            OpCode::Invalid => return Err(RuntimeError::InvalidOpCode { pc: self.runtime.pc()? }),

            OpCode::Constant => {
                let constant = self.fetch_constant()?
                    .to_owned();
                self.runtime.push_value(constant);
            },

            OpCode::GetLocal => {
                let offset = self.fetch_local_offset()?;
                let value = self.runtime.get_local(offset)?.clone();

                self.runtime.push_value(value);
            },

            OpCode::SetLocal => {
                let top = self.runtime.pop_expecting()?;
                let offset = self.fetch_local_offset()?;
                self.runtime.set_local(offset, top)?;
            },

            OpCode::GetGlobal => {
                let constant = self.fetch_constant()?
                    .to_owned()
                    .take_string()?;

                let global = self.get_global(&constant).ok_or(RuntimeError::GlobalNotFound { name: constant })?.clone();
                self.runtime.push_value(global);
            },

            OpCode::SetGlobal => {
                let constant = self.fetch_constant()?
                    .to_owned()
                    .take_string()?;

                let top = self.runtime.pop_expecting()?;
                self.add_global(constant, top);
            },

            OpCode::Pop => {
                self.runtime.pop_expecting()?;
            },

            OpCode::NewObject => {
                self.runtime.push_value(new_object());
            }

            OpCode::GetField => {
                let name = self.fetch_constant()?.to_string();

                let obj = self.runtime.pop_expecting()?.take_object()?;
                let obj = obj.borrow();

                let field = obj.get_field(&name)
                    .ok_or(RuntimeError::NoFieldMatchingName { name })?;

                self.runtime.push_value(field.to_owned());
            },

            OpCode::SetField => {
                let value = self.runtime.pop_expecting()?;
                let obj = self.runtime.pop_expecting()?.take_object()?;
                let name = self.fetch_constant()?.to_string();

                obj.borrow_mut().set_field(name, value.to_owned());
            },

            OpCode::Add => {
                let rhs = self.runtime.pop_expecting()?;
                let lhs = self.runtime.pop_expecting()?;

                self.runtime.push_value(lhs.try_add(&rhs)?);
            },
            OpCode::Sub => {
                let rhs = self.runtime.pop_expecting()?;
                let lhs = self.runtime.pop_expecting()?;

                self.runtime.push_value(lhs.try_sub(&rhs)?);
            },
            OpCode::Mul => {
                let rhs = self.runtime.pop_expecting()?;
                let lhs = self.runtime.pop_expecting()?;

                self.runtime.push_value(lhs.try_mul(&rhs)?);
            },
            OpCode::Div => {
                let rhs = self.runtime.pop_expecting()?;
                let lhs = self.runtime.pop_expecting()?;

                self.runtime.push_value(lhs.try_div(&rhs)?);
            },
            OpCode::Mod => {
                let rhs = self.runtime.pop_expecting()?;
                let lhs = self.runtime.pop_expecting()?;

                self.runtime.push_value(lhs.try_mod(&rhs)?);
            },
            OpCode::Neg => {
                let rhs = self.runtime.pop_expecting()?;

                self.runtime.push_value(rhs.try_negate()?);
            },
            OpCode::Not => {
                let rhs = self.runtime.pop_expecting()?;

                self.runtime.push_value(rhs.not());
            },
            
            OpCode::Eq => {
                let rhs = self.runtime.pop_expecting()?;
                let lhs = self.runtime.pop_expecting()?;
                
                self.runtime.push_value(lhs.is_equal(&rhs));
            },
            OpCode::Neq => {
                let rhs = self.runtime.pop_expecting()?;
                let lhs = self.runtime.pop_expecting()?;

                self.runtime.push_value(lhs.not_equal(&rhs));
            },
            
            OpCode::Lt => {
                let rhs = self.runtime.pop_expecting()?;
                let lhs = self.runtime.pop_expecting()?;

                self.runtime.push_value(lhs.lesser(&rhs));
            },
            OpCode::Le => {
                let rhs = self.runtime.pop_expecting()?;
                let lhs = self.runtime.pop_expecting()?;

                self.runtime.push_value(lhs.lesser_equal(&rhs));
            },
            
            OpCode::Gt => {
                let rhs = self.runtime.pop_expecting()?;
                let lhs = self.runtime.pop_expecting()?;

                self.runtime.push_value(lhs.greater(&rhs));
            },
            OpCode::Ge => {
                let rhs = self.runtime.pop_expecting()?;
                let lhs = self.runtime.pop_expecting()?;

                self.runtime.push_value(lhs.greater_equal(&rhs));
            },

            OpCode::Jump => {
                let addr = self.fetch_instruction_offset()?;

                self.runtime.set_pc(addr as usize);
            },

            OpCode::JumpIf => {
                let addr = self.fetch_instruction_offset()?;
                let val = self.runtime.pop_expecting()?;

                if val.truthy() {
                    #[cfg(feature = "debug_tracing")]
                    log::debug!("Value '{}' truthy, jumping from 0x{:x} -> 0x{:x}", val, self.runtime.pc()?, addr);
                    self.runtime.set_pc(addr as usize);
                }
                else {
                    #[cfg(feature = "debug_tracing")]
                    log::debug!("Skipping jump at 0x{:x}", self.runtime.pc()?);
                }
            },

            OpCode::Call => {
                let func = self.runtime.pop_expecting()?;

                match func {
                    Value::NativeFunction(func) => {
                        let callable = &func.fun;
                        let local_count = self.runtime.local_count()? - func.arity;
                        let value = callable(&mut self.runtime)?;
                        self.runtime.pop_until(local_count)?;
                        self.runtime.push_value(value);
                    },
                    Value::Function(func) => {
                        self.runtime.call(func.chunk.clone(), func.arity);
                    },
                    v => return Err(RuntimeError::IncorrectType { ty: v.type_name().to_string(), expected: "function".to_string() }),
                }
            },

            OpCode::Return => {
                self.runtime.ret()?;
            },

            OpCode::Exit => {
                self.running = false;
            }

            OpCode::Poll => {
                self.runtime.poll()?;
            },
            
            OpCode::Render => {
                let value = self.runtime.pop_expecting()?;
                self.runtime.render(value)?;
            }
        }

        Ok(())
    }

    pub fn add_global(&mut self, name: impl AsRef<str>, value: impl Into<Value>) {
        self.runtime.add_global(name, value);
    }

    pub fn get_global(&mut self, name: impl AsRef<str>) -> Option<&Value> {
        self.runtime.get_global(name)
    }

    pub fn remove_global(&mut self, name: impl AsRef<str>) {
        self.runtime.remove_global(name);
    }

    pub fn fetch_op(&mut self) -> Result<OpCode, RuntimeError> {
        let byte = self.runtime.chunk()?
            .read_u8(self.runtime.pc()?)
            .ok_or(RuntimeError::AccessOutOfBounds { at: self.runtime.pc()?, pc: self.runtime.pc()? })?;

        self.runtime.move_pc(1);

        OpCode::try_from_primitive(byte)
            .map_err(|_| RuntimeError::UnrecognizedOpCode { op: byte, pc: self.runtime.pc().unwrap() })
    }

    pub fn fetch_u8(&mut self) -> Result<u8, RuntimeError> {
        let byte =  self.runtime.chunk()?
            .read_u8(self.runtime.pc()?)
            .ok_or(RuntimeError::AccessOutOfBounds { at: self.runtime.pc()?, pc: self.runtime.pc()? })?;
        self.runtime.move_pc(1);
        Ok(byte)
    }

    pub fn fetch_u16(&mut self) -> Result<u16, RuntimeError> {
        let val =  self.runtime.chunk()?.read_u16(self.runtime.pc()?).ok_or(RuntimeError::AccessOutOfBounds { at: self.runtime.pc()?, pc: self.runtime.pc()? })?;
        self.runtime.move_pc(2);
        Ok(val)
    }

    pub fn fetch_u32(&mut self) -> Result<u32, RuntimeError> {
        let val =  self.runtime.chunk()?.read_u32(self.runtime.pc()?).ok_or(RuntimeError::AccessOutOfBounds { at: self.runtime.pc()?, pc: self.runtime.pc()? })?;
        self.runtime.move_pc(4);
        Ok(val)
    }

    pub fn fetch_u64(&mut self) -> Result<u64, RuntimeError> {
        let val =  self.runtime.chunk()?.read_u64(self.runtime.pc()?).ok_or(RuntimeError::AccessOutOfBounds { at: self.runtime.pc()?, pc: self.runtime.pc()? })?;
        self.runtime.move_pc(8);
        Ok(val)
    }
    
    pub fn fetch_constant_offset(&mut self) -> Result<ConstantOffset, RuntimeError> {
        let val =  self.runtime.chunk()?.read_constant_offset(self.runtime.pc()?).ok_or(RuntimeError::AccessOutOfBounds { at: self.runtime.pc()?, pc: self.runtime.pc()? })?;
        self.runtime.move_pc(size_of::<ConstantOffset>());
        Ok(val)
    }

    pub fn fetch_local_offset(&mut self) -> Result<LocalOffset, RuntimeError> {
        let val =  self.runtime.chunk()?.read_local_offset(self.runtime.pc()?).ok_or(RuntimeError::AccessOutOfBounds { at: self.runtime.pc()?, pc: self.runtime.pc()? })?;
        self.runtime.move_pc(size_of::<LocalOffset>());
        Ok(val)
    }
    
    pub fn fetch_instruction_offset(&mut self) -> Result<InstructionOffset, RuntimeError> {
        let val =  self.runtime.chunk()?.read_instruction_offset(self.runtime.pc()?).ok_or(RuntimeError::AccessOutOfBounds { at: self.runtime.pc()?, pc: self.runtime.pc()? })?;
        self.runtime.move_pc(size_of::<InstructionOffset>());
        Ok(val)
    }

    pub fn fetch_constant(&mut self) -> Result<&Value, RuntimeError> {
        let offset = self.fetch_constant_offset()? as usize;
        let value = self.runtime.chunk()?.get_constant(offset)
            .ok_or(RuntimeError::InvalidConstantAccess { at: offset, pc: self.runtime.pc()? })?;

        Ok(value)
    }
}

impl Drop for Vm {
    fn drop(&mut self) {
        ratatui::restore();
    }
}
