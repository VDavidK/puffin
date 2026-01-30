
pub mod layout;
pub mod vm;
pub mod op;
pub mod program;
pub mod value;

pub use program::Program;
pub use value::Value;

use vm::Vm;
use ratatui::prelude::*;

use crate::value::{UserValue, UserValueHandle};

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

impl UserValue for RuntimeError {}

pub fn run_program(program: &Program) -> Result<(), RuntimeError> {
    ratatui::run(move |terminal| -> std::io::Result<()> {
        let mut vm = Vm::new(program);

        loop {
            terminal.draw(render)?;
            if ratatui::crossterm::event::read()?.is_key_press() {
                break Ok(());
            }
        }
    })?;

    Ok(())
}

fn render(frame: &mut Frame) {
    frame.render_widget("Hello World", frame.area());
}

