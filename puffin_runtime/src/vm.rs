use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use num_enum::TryFromPrimitive;

use crate::{RuntimeError, op::OpCode, value::{Value, new_instance}};
use crate::chunk::{InstructionOffset, ConstantOffset, LocalOffset};
use crate::runtime::Runtime;
use crate::value::{new_class, BlockNode, ComponentNode, ConditionalNode, Node, NodeType, Reactive};

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
                    #[cfg(feature = "debug_tracing")]
                    log::error!("Runtime error occurred: {}", err);
                    return Err(err);
                }

                Ok(Some(value)) => break value,

                _ => continue,
            }
        };

        #[cfg(feature = "debug_tracing")]
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

                let global = self.runtime.get_global(constant.borrow().as_str()).ok_or(RuntimeError::GlobalNotFound { name: constant.borrow().as_str().to_owned() })?.clone();
                self.runtime.push_value(global);
            },

            OpCode::SetGlobal => {
                let constant = self.fetch_constant()?
                    .to_owned()
                    .take_string()?;

                let top = self.runtime.pop_expecting()?;
                self.runtime.add_global(constant.borrow().as_str(), top)?;
            },

            OpCode::Pop => {
                self.runtime.pop_expecting()?;
            },

            OpCode::ReservePush => {
                let value = self.runtime.pop_expecting()?;
                self.runtime.push_intermediate(value);
            },

            OpCode::ReservePop => {
                let value = self.runtime.pop_intermediate().ok_or(RuntimeError::IntermediateStackEmpty)?;
                self.runtime.push_value(value);
            },

            OpCode::NewClass => {
                let name = self.fetch_constant()?
                    .to_owned()
                    .take_string()?;

                self.runtime.push_value(new_class(name.borrow().as_str()));
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
                        todo!("static")
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
                    Value::Dictionary(dict) => {
                        dict.borrow()
                            .get(&name.into())
                            .cloned()
                            .unwrap_or(Value::Null)
                    }
                    v => {
                        return Err(RuntimeError::IncorrectType { ty: v.type_name().into(), expected: "instance, module, class or dictionary".into() })
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
                            .set_field(name, value.to_owned())?;
                    }
                    Value::Class(class) => {
                        class
                            .borrow_mut()
                            .set_field(name, value.to_owned())?;
                    }
                    Value::Dictionary(dict) => {
                        dict
                            .borrow_mut()
                            .insert(Value::String(Rc::new(RefCell::new(name))), value.to_owned());
                    }
                    obj => Err(RuntimeError::InvalidAssignmentTarget{ ty: obj.type_name().to_owned() })?,
                }

            },

            OpCode::GetIndex => {
                let idx = self.runtime.pop_expecting()?;

                let value = match self.runtime.pop_expecting()?.eval()? {
                    Value::List(li) => {
                        let idx = idx.take_int()?;
                        li.borrow().get(idx as usize).cloned().unwrap_or(Value::Null)
                    }
                    Value::Dictionary(dict) => {
                        dict.borrow().get(&idx).cloned().unwrap_or(Value::Null)
                    }
                    Value::String(string) => {
                        let char = string.borrow().chars().nth(idx.take_int()? as usize);
                        if let Some(c) = char {
                            Value::String(Rc::new(RefCell::new(c.to_string())))
                        } else {
                            Value::Null
                        }
                    }
                    v => {
                        return Err(RuntimeError::IncorrectType { ty: v.type_name().into(), expected: "list or dictionary".into() })
                    }
                };

                self.runtime.push_value(value);
            }

            OpCode::SetClassMethod => {
                let name = self.fetch_constant()?
                    .to_owned()
                    .take_string()?;
                let method = self.runtime.pop_expecting()?;
                let class = self.runtime.pop_expecting()?
                    .take_class()?;

                class.borrow_mut().set_method(name.borrow().as_str(), method);
            }

            OpCode::NewList => {
                self.runtime.push_value(Vec::<Value>::new());
            }

            OpCode::NewDictionary => {
                self.runtime.push_value(HashMap::<Value, Value>::new());
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

            OpCode::SetHandler => {
                let name = self.fetch_constant()?
                    .to_owned()
                    .take_string()?;
                let method = self.runtime.pop_expecting()?;
                let class = self.runtime.pop_expecting()?
                    .take_class()?;

                class.borrow_mut().set_handler(name.borrow().as_str(), method);
            }

            OpCode::MakeReactive => {
                let value = self.runtime.pop_expecting()?;

                let reactive_value = match value {
                    Value::Reactive(inner) => Value::Reactive(inner),
                    Value::Derived(inner) => Value::Derived(inner),

                    val => Reactive::new(val).into(),
                };

                self.runtime.push_value(reactive_value);
            }

            OpCode::Add => {
                let rhs = self.runtime.pop_expecting()?;
                let lhs = self.runtime.pop_expecting()?;

                self.runtime.push_value(lhs.try_add_derivable(&rhs)?);
            },
            OpCode::Sub => {
                let rhs = self.runtime.pop_expecting()?;
                let lhs = self.runtime.pop_expecting()?;

                self.runtime.push_value(lhs.try_sub_derivable(&rhs)?);
            },
            OpCode::Mul => {
                let rhs = self.runtime.pop_expecting()?;
                let lhs = self.runtime.pop_expecting()?;

                self.runtime.push_value(lhs.try_mul_derivable(&rhs)?);
            },
            OpCode::Div => {
                let rhs = self.runtime.pop_expecting()?;
                let lhs = self.runtime.pop_expecting()?;

                self.runtime.push_value(lhs.try_div_derivable(&rhs)?);
            },
            OpCode::Mod => {
                let rhs = self.runtime.pop_expecting()?;
                let lhs = self.runtime.pop_expecting()?;

                self.runtime.push_value(lhs.try_mod_derivable(&rhs)?);
            },
            OpCode::Neg => {
                let rhs = self.runtime.pop_expecting()?;

                self.runtime.push_value(rhs.try_negate_derivable()?);
            },
            OpCode::Not => {
                let rhs = self.runtime.pop_expecting()?;

                self.runtime.push_value(rhs.not_derivable()?);
            },
            
            OpCode::Eq => {
                let rhs = self.runtime.pop_expecting()?;
                let lhs = self.runtime.pop_expecting()?;
                
                self.runtime.push_value(lhs.is_equal_derivable(&rhs)?);
            },
            OpCode::Neq => {
                let rhs = self.runtime.pop_expecting()?;
                let lhs = self.runtime.pop_expecting()?;

                self.runtime.push_value(lhs.not_equal_derivable(&rhs)?);
            },
            
            OpCode::Lt => {
                let rhs = self.runtime.pop_expecting()?;
                let lhs = self.runtime.pop_expecting()?;

                self.runtime.push_value(lhs.lesser_derivable(&rhs)?);
            },
            OpCode::Le => {
                let rhs = self.runtime.pop_expecting()?;
                let lhs = self.runtime.pop_expecting()?;

                self.runtime.push_value(lhs.lesser_equal_derivable(&rhs)?);
            },
            
            OpCode::Gt => {
                let rhs = self.runtime.pop_expecting()?;
                let lhs = self.runtime.pop_expecting()?;

                self.runtime.push_value(lhs.greater_derivable(&rhs)?);
            },
            OpCode::Ge => {
                let rhs = self.runtime.pop_expecting()?;
                let lhs = self.runtime.pop_expecting()?;

                self.runtime.push_value(lhs.greater_equal_derivable(&rhs)?);
            },

            OpCode::Jump => {
                let addr = self.fetch_instruction_offset()?;

                self.runtime.set_pc(addr as usize);
            },

            OpCode::JumpIf => {
                let addr = self.fetch_instruction_offset()?;
                let val = self.runtime.pop_expecting()?;

                if val.truthy()? {
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

            OpCode::Bind => {
                let func = self.runtime
                    .pop_expecting()?;
                let value = self.runtime
                    .pop_expecting()?
                    .take_instance()?;

                match func.to_owned() {
                    Value::Function(func) => {
                        func.borrow_mut().bound_value = Some(value);
                    }
                    Value::NativeFunction(func) => {
                        func.borrow_mut().bound_value = Some(value);
                    }

                    v => Err(RuntimeError::IncorrectType { expected: "function or native_function".to_owned(), ty: v.type_name().to_owned() })?
                }

                self.runtime.push_value(func);
            }

            OpCode::NewNodeComponent => {
                let instance = self.runtime.pop_expecting()?
                    .take_instance()?;
                let node = ComponentNode::try_from(instance).map(Into::<NodeType>::into)?;
                self.runtime.push_value(node);
            }

            OpCode::NewNodeBlock => {
                let list = self.runtime
                    .pop_expecting()?
                    .take_list()?;

                let nodes = list
                    .borrow()
                    .to_owned()
                    .into_iter()
                    .map(Value::take_node)
                    .collect::<Result<Vec<_>, _>>()?;

                let node = BlockNode::from(nodes);

                self.runtime.push_value(Node::Block(node));
            }

            OpCode::NewNodeIf => {
                let condition = self.runtime.pop_expecting()?;

                let then_branch = self.runtime
                    .pop_expecting()?
                    .take_node()?;

                let else_branch = match self.runtime.pop_expecting()? {
                    Value::Null => BlockNode::from(vec![]).into(),
                    Value::Node(node) => node,
                    v => Err(RuntimeError::IncorrectType { expected: "node or null".to_owned(), ty: v.type_name().to_owned() })?,
                };

                self.runtime.push_value(ConditionalNode {
                    condition,
                    if_case: then_branch,
                    else_case: else_branch,
                })
            }
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
