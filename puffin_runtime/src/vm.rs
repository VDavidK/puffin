use num_enum::TryFromPrimitive;

use crate::{RuntimeError, op::OpCode, value::{Value, new_instance}};
use crate::chunk::{InstructionOffset, ConstantOffset, LocalOffset};
use crate::runtime::Runtime;
use crate::value::new_class;

#[derive(Debug)]
pub(crate) struct Vm<'a> {
    running: bool,
    runtime: &'a mut Runtime,
}

impl<'a> Vm<'a> {
    pub fn new(runtime: &'a mut Runtime) -> Self {
        Self {
            runtime,
            running: true,
        }
    }

    pub fn run(&mut self) -> Result<Value, RuntimeError> {
        #[cfg(feature = "debug_tracing")]
        log::debug!("Starting execution");

        let ret_value = loop {
            if !self.is_running() {
                break Value::Null;
            }

            match self.execute() {
                Err(err) => {
                    log::error!("Runtime error occurred: {}", err);
                    return Err(err);
                }

                Ok(Some(value)) => break value,

                _ => continue,
            }
        };

        log::debug!("Execution finished without errors");

        Ok(ret_value)
    }

    pub fn is_running(&self) -> bool {
        let not_at_end = match (self.runtime.pc(), self.runtime.chunk()) {
            (Ok(pc), Ok(chunk)) => pc < chunk.byte_len(),
            _ => false,
        };

        self.running
            && !self.runtime.call_stack_empty()
            && not_at_end
    }

    pub fn execute(&mut self) -> Result<Option<Value>, RuntimeError> {
        let op = self.fetch_op()?;

        #[cfg(feature = "debug_tracing")]
        {
            self.runtime.log_stack();
            log::debug!("exec [{}] 0x{:X} | {op:?}", self.runtime.chunk_name()?, self.runtime.pc()? - 1);
        }
        
        match op {
            OpCode::Invalid => return Err(RuntimeError::InvalidOpCode { pc: self.runtime.pc()? - 1 }),

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

                let global = self.runtime.get_global(&constant).ok_or(RuntimeError::GlobalNotFound { name: constant })?.clone();
                self.runtime.push_value(global);
            },

            OpCode::SetGlobal => {
                let constant = self.fetch_constant()?
                    .to_owned()
                    .take_string()?;

                let top = self.runtime.pop_expecting()?;
                self.runtime.add_global(constant, top);
            },

            OpCode::Pop => {
                self.runtime.pop_expecting()?;
            },

            OpCode::NewInstance => {
                let class = self.runtime.pop_expecting()?
                    .take_class()?;

                let instance = new_instance(class.clone());

                if let Some(constructor) = class.borrow().get_constructor() {
                    self.runtime.push_value(instance.clone());
                    self.runtime.call_fn(constructor.clone().take_function()?)?;
                }

                self.runtime.push_value(instance);
            }

            OpCode::NewClass => {
                let name = self.fetch_constant()?
                    .to_owned()
                    .take_string()?;

                self.runtime.push_value(new_class(name));
            }

            OpCode::SetConstructor => {
                let constr = self.runtime.pop_expecting()?
                    .take_function()?;

                let class = self.runtime.pop_expecting()?
                    .take_class()?;

                class.borrow_mut()
                    .set_constructor(constr);
            }

            OpCode::GetField => {
                let name = self.fetch_constant()?.to_string();

                let value = match self.runtime.pop_expecting()? {
                    Value::Class(_class) => {
                        todo!()
                    }
                    Value::Module(module) => {
                        module.borrow().get_item(&name)
                            .ok_or(RuntimeError::NoFieldMatchingName { name })?.to_owned()
                    }
                    Value::Instance(inst) => {
                        inst.borrow()
                            .get_field(&name)
                            .cloned()
                            .or_else(|| inst.borrow()
                                .get_class()
                                .borrow()
                                .get_method(&name)
                                .cloned()
                            )
                            .ok_or(RuntimeError::NoFieldMatchingName { name })?
                    }
                    v => {
                        return Err(RuntimeError::IncorrectType { ty: v.type_name().into(), expected: "instance, module, class, dictionary or array".into() })
                    }
                };

                self.runtime.push_value(value);
            },

            OpCode::SetField => {
                let value = self.runtime.pop_expecting()?;
                let obj = self.runtime.pop_expecting()?;
                let name = self.fetch_constant()?.to_string();

                match obj {
                    Value::Instance(instance) => {
                        instance
                            .borrow_mut()
                            .set_field(name, value.to_owned());
                    }
                    Value::Class(class) => {
                        class
                            .borrow_mut()
                            .set_field(name, value.to_owned());
                    }
                    _ => panic!("Invalid assignment target"),
                }

            },

            OpCode::NewList => {
                self.runtime.push_value(Vec::<Value>::new());
            }

            OpCode::PushList => {
                let value = self.runtime.pop_expecting()?;
                let list = self.runtime.pop_expecting()?
                    .take_list()?;

                list.borrow_mut().push(value);
            }

            OpCode::PopList => {
                let list = self.runtime.pop_expecting()?
                    .take_list()?;

                let value = list.borrow_mut().pop().unwrap_or(Value::Null);
                self.runtime.push_value(value);
            }

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
                let num_args = self.fetch_u8()? as usize;
                let func = self.runtime.pop_expecting()?;
                let ret_value = self.runtime.call_val(func, num_args)?;
                self.runtime.push_value(ret_value);
            },

            OpCode::Return => {
                return Ok(Some(self.runtime.ret()?));
            },

            OpCode::Exit => {
                self.running = false;
            },

            OpCode::Poll => {
                self.runtime.poll()?;
            },
            
            OpCode::Render => {
                self.runtime.render()?;
            },

            OpCode::SetRoot => {
                // let element = self.runtime
                //     .pop_expecting()?
                //     .take_instance()?;

                // self.root = Some(element);
            },
        }

        Ok(None)
    }

    pub fn fetch_op(&mut self) -> Result<OpCode, RuntimeError> {
        let byte = self.read_u8()?;
        self.runtime.move_pc(1);

        let pc = self.runtime.pc()?;

        OpCode::try_from_primitive(byte)
            .map_err(|_| RuntimeError::UnrecognizedOpCode { op: byte, pc })
    }

    pub fn fetch_u8(&mut self) -> Result<u8, RuntimeError> {
        let byte = self.read_u8()?;
        self.runtime.move_pc(size_of::<u8>());
        Ok(byte)
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

    pub fn read_u8(&mut self) -> Result<u8, RuntimeError> {
        self.runtime.chunk()?
            .read_u8(self.runtime.pc()?)
            .ok_or(RuntimeError::AccessOutOfBounds { at: self.runtime.pc()?, pc: self.runtime.pc()? })
    }
}
