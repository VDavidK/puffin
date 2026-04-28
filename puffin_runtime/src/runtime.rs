use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use ratatui::DefaultTerminal;
use crate::chunk::LocalOffset;
use crate::vm::Vm;
use crate::{Chunk, RuntimeError, value::Value};
use crate::value::{new_instance, FunctionType, InstanceType, Module, LayoutDirection, LayoutNode, Node};

#[derive(Debug, Clone)]
pub struct CallFrame {
    pub chunk: Rc<Chunk>,
    pub stack_offset: usize,
    pub local_count: usize,
    pub pc: usize,
}

#[derive(Debug)]
pub struct Runtime {
    stack: Vec<Value>,
    call_stack: Vec<CallFrame>,
    term: DefaultTerminal,
    globals: HashMap<String, Value>,
    node_stack: Vec<Node>,
}

impl Default for Runtime {
    fn default() -> Self {
        let term = ratatui::init();

        #[allow(clippy::new_without_default)]
        let runtime = Runtime {
            stack: vec![],
            call_stack: vec![],
            globals: HashMap::new(),
            term,
            node_stack: vec![],
        };

        runtime
    }
}

impl Runtime {
    pub fn execute(&mut self, chunk: Rc<Chunk>) -> Result<Value, RuntimeError> {
        log::debug!("Executing chunk '{}'", chunk.get_name());

        self.push_call_frame(chunk.clone(), 0);

        let ret_value = Vm::new(self)
            .run()?;

        Ok(ret_value)
    }

    pub fn run_component(&mut self, name: impl AsRef<str>) -> Result<(), RuntimeError> {
        let name = name.as_ref();
        let component = self.get_global(name)
            .ok_or(RuntimeError::GlobalNotFound { name: name.to_owned() })?;

        let component = component.clone().take_class()?;
        let instance = new_instance(component.clone());

        if let Some(constructor) = component.borrow().get_constructor() {
            self.push_value(instance.clone());
            self.call_fn(constructor.clone().take_function()?)?;
        }

        loop {
            let main = instance.borrow()
                .get_field("<main>")
                .ok_or(RuntimeError::GlobalNotFound { name: "<main>".to_string() })?
                .clone();
            self.push_value(component.clone());
            let ret = self.call_val(main, 1)?;
            self.push_value(ret);

            self.render()?;
            self.poll()?;
        }
    }

    pub fn call(&mut self, value: Value, args: &[Value]) -> Result<Value, RuntimeError> {
        for arg in args {
            self.push_value(arg.to_owned());
        }

        self.call_val(value, args.len())
    }

    pub fn call_val(&mut self, value: Value, num_args: usize) -> Result<Value, RuntimeError> {
        match value {
            Value::NativeFunction(func) => {
                //self.match_function_param_count(func.arity, num_args);
                let callable = &func.fun;
                let local_count = self.local_count()? - num_args;
                let value = callable(self, num_args)?;
                self.pop_until(local_count)?;
                Ok(value)
            },
            Value::Function(func) => {
                self.match_function_param_count(func.arity, num_args);
                let ret_value = self.call_fn(func)?;
                Ok(ret_value)
            },
            Value::Class(cls) => {
                let instance = new_instance(cls.clone());

                if let Some(constructor) = cls.borrow().get_constructor() {
                    let func = constructor.clone();
                    self.push_value(instance.clone());
                    self.call_val(func, num_args)?;
                    self.pop_value();
                }

                Ok(Value::Instance(instance))
            },
            v => Err(RuntimeError::IncorrectType { ty: v.type_name().to_owned(), expected: "function".to_owned() }),
        }
    }



    pub fn call_fn(&mut self, func: FunctionType) -> Result<Value, RuntimeError> {
        log::debug!("Executing function '{}'", func.identifier);

        self.push_call_frame(func.chunk.clone(), func.arity);

        let ret_value = Vm::new(self)
            .run()?;

        Ok(ret_value)
    }

    fn match_function_param_count(&mut self, arity: usize, passed_in: usize) {
        let arg_diff = passed_in as i32 - arity as i32;

        if arg_diff < 0 {
            for _ in 0..-arg_diff {
                self.push_value(Value::Null);
            }
        } else {
            for _ in 0..arg_diff {
                self.pop_value();
            }
        }

    }

    pub fn include_module(&mut self, module: Module) {
        let name = module.get_name().to_owned();
        let module = Rc::new(RefCell::new(module));
        self.add_global(name, module);
    }

    pub(crate) fn push_call_frame(&mut self, chunk: Rc<Chunk>, arity: usize) {
        if let Some(frame) = self.call_stack.last_mut() {
            frame.local_count -= arity;
        }

        let frame = CallFrame {
            chunk,
            stack_offset: self.stack.len() - arity,
            local_count: arity,
            pc: 0,
        };

        self.call_stack.push(frame);
    }


    pub fn call_stack_empty(&self) -> bool {
        self.call_stack.is_empty()
    }

    pub fn get_local(&self, offset: LocalOffset) -> Result<&Value, RuntimeError> {
        let offset = if offset >= 0 {
            offset + self.stack_offset() as LocalOffset
        } else {
            self.stack.len() as LocalOffset + offset
        };

        let value = self.stack.get(offset as usize)
            .ok_or(RuntimeError::StackOutOfBounds { at: offset as usize, pc: self.pc()? })?;

        Ok(value)
    }

    pub fn set_local(&mut self, offset: LocalOffset, value: Value) -> Result<(), RuntimeError> {
        let offset = offset as usize + self.stack_offset();

        if offset >= self.stack.len() {
            return Err(RuntimeError::StackOutOfBounds { at: offset, pc: self.pc()? });
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

    #[cfg(feature = "debug_tracing")]
    pub fn log_stack(&self) {
        let mut values = self.stack.iter().map(|v| v.to_string()).collect::<Vec<_>>().join("> <");
        if !values.is_empty() {
            values = format!("<{values}>");
        }

        log::debug!("stack: {values}");
    }

    pub(crate) fn ret(&mut self) -> Result<Value, RuntimeError> {
        let ret_value = self.pop_expecting()?;

        if let Some(frame) = self.call_stack.pop() {
            // Pop all values pushed in the current call frame
            for _ in 0..frame.local_count {
                self.stack.pop();
            }

            Ok(ret_value)
        } else {
            Err(RuntimeError::CallStackEmpty)
        }
    }

    pub(crate) fn local_count(&self) -> Result<usize, RuntimeError> {
        self.call_stack
            .last()
            .map(|frame| frame.local_count)
            .ok_or(RuntimeError::CallStackEmpty)
    }

    pub(crate) fn pop_until(&mut self, target_local_count: usize) -> Result<(), RuntimeError> {
        let local_count = self.local_count()?;

        for _ in 0..(local_count - target_local_count) {
            self.pop_expecting()?;
        }

        Ok(())
    }

    pub fn stack_offset(&self) -> usize {
        self.call_stack.last()
            .map(|frame| frame.stack_offset)
            .unwrap_or(0)
    }

    pub fn chunk(&self) -> Result<&Chunk, RuntimeError> {
        self.call_stack.last()
            .map(|frame| frame.chunk.as_ref())
            .ok_or(RuntimeError::StackEmpty)
    }

    pub fn chunk_name(&self) -> Result<&str, RuntimeError> {
        Ok(self.chunk()?.get_name())
    }

    pub fn pc(&self) -> Result<usize, RuntimeError> {
        self.call_stack.last()
            .map(|frame| frame.pc)
            .ok_or(RuntimeError::StackEmpty)
    }

    pub fn move_pc(&mut self, amount: usize) {
        if let Some(frame) = self.call_stack.last_mut() {
            frame.pc += amount
        }
    }

    pub fn set_pc(&mut self, pc: usize) {
        if let Some(frame) = self.call_stack.last_mut() {
            frame.pc = pc
        }
    }

    pub fn render(&mut self) -> Result<(), RuntimeError> {
        //let stack = std::mem::take(&mut self.node_stack);
        let node = self.pop_expecting()?
            .take_list()?
            .borrow()
            .iter()
            .cloned()
            .map(|v| v.take_node())
            .collect::<Result<Vec<_>, _>>()?;

        let layout = LayoutNode {
            direction: LayoutDirection::Vertical,
            nodes: node,
        };

        self.term.draw(move |frame| {
            frame.render_widget(&layout, frame.area());
        })?;

        Ok(())
    }

    pub fn poll(&self) -> Result<(), RuntimeError> {
        ratatui::crossterm::event::read()?;
        Ok(())
    }

    pub fn add_global(&mut self, name: impl Into<String>, value: impl Into<Value>) {
        self.globals.insert(name.into(), value.into());
    }

    pub fn remove_global(&mut self, name: impl AsRef<str>) {
        self.globals.remove(name.as_ref());
    }

    pub fn get_global(&self, name: impl AsRef<str>) -> Option<&Value> {
        self.globals.get(name.as_ref())
    }
}


impl Drop for Runtime {
    fn drop(&mut self) {
        ratatui::restore();
    }
}