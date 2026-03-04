use std::collections::HashMap;
use std::rc::Rc;
use ratatui::DefaultTerminal;
use crate::chunk::LocalOffset;
use crate::{Chunk, RuntimeError, Value};

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
}

impl Runtime {
    pub fn new(starting_frame: CallFrame) -> Self {
        let term = ratatui::init();

        Runtime {
            stack: vec![],
            call_stack: vec![starting_frame],
            globals: HashMap::new(),
            term,
        }
    }

    pub fn call_stack_empty(&self) -> bool {
        self.call_stack.is_empty()
    }

    pub fn get_local(&self, offset: LocalOffset) -> Result<&Value, RuntimeError> {
        let offset = if offset >= 0 {
            offset
        } else {
            self.stack.len() as LocalOffset + offset
        };

        let value = self.stack.get(offset as usize + self.stack_offset())
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
        let mut values = self.stack.iter().map(|v| v.to_string()).collect::<Vec<_>>().join("] [");
        if !values.is_empty() {
            values = format!("[{values}]");
        }

        log::debug!("stack> {values}");
    }

    pub(crate) fn call(&mut self, chunk: Rc<Chunk>, arity: usize) {
        if let Some(frame) = self.call_stack.last_mut() {
            frame.local_count -= arity as usize;
        }

        let frame = CallFrame {
            chunk,
            stack_offset: self.stack.len() - arity,
            local_count: 0,
            pc: 0,
        };

        self.call_stack.push(frame);
    }

    pub(crate) fn ret(&mut self) -> Result<(), RuntimeError> {
        let ret_value = self.pop_expecting()?;

        if let Some(frame) = self.call_stack.pop() {
            // Pop all values pushed in the current call frame
            for _ in 0..frame.local_count {
                self.stack.pop();
            }

            self.push_value(ret_value);
            Ok(())
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

    pub fn render(&mut self, value: Value) -> Result<(), RuntimeError> {
        self.term.draw(|frame| frame.render_widget(value.to_string(), frame.area()))?;
        Ok(())
    }

    pub fn poll(&self) -> Result<(), RuntimeError> {
        ratatui::crossterm::event::read()?;
        Ok(())
    }

    pub fn add_global(&mut self, name: impl AsRef<str>, value: impl Into<Value>) {
        self.globals.insert(name.as_ref().to_owned(), value.into());
    }

    pub fn remove_global(&mut self, name: impl AsRef<str>) {
        self.globals.remove(name.as_ref());
    }

    pub fn get_global(&self, name: impl AsRef<str>) -> Option<&Value> {
        self.globals.get(name.as_ref())
    }
}
