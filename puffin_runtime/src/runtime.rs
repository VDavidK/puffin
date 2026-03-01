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
}

impl Runtime {
    pub fn new(starting_frame: CallFrame) -> Self {
        let term = ratatui::init();

        Runtime {
            stack: vec![],
            call_stack: vec![starting_frame],
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
            stack_offset: self.stack.len(),
            local_count: 0,
            pc: 0,
        };

        self.call_stack.push(frame);
    }

    pub(crate) fn ret(&mut self) -> Result<(), RuntimeError> {
        if let Some(frame) = self.call_stack.pop() {
            let ret_value = self.pop_expecting()?;

            // Pop all values pushed in the current call frame
            for _ in 0..frame.local_count - 1 {
                self.pop_expecting()?;
            }

            self.push_value(ret_value);
            Ok(())
        } else {
            Err(RuntimeError::CallStackEmpty)
        }
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
}
