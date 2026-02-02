
pub mod layout;
pub mod vm;
pub mod op;
pub mod chunk;
pub mod value;

pub use chunk::Chunk;
pub use value::Value;

use vm::Vm;
use ratatui::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("Unrecognized op code ({0:x}) at: {1:x}")]
    UnrecognizedOpCode(u8, usize),

    #[error("Use of invalid op code at: {0:x}")]
    InvalidOpCode(usize),

    #[error("Trying to access out of bounds memory (0:x) at: {1:x}")]
    AccessOutOfBounds(usize, usize),

    #[error("Trying to access out of bounds literal (0:x) at: {1:x}")]
    InvalidLiteralAccess(usize, usize),
}


pub fn run(program: &Chunk) -> Result<(), RuntimeError> {
    let mut vm = Vm::new(program);

    while vm.is_running() {
        vm.execute()?;
    }

    // let mut terminal = ratatui::init();

    // loop {
    //     terminal.draw(render)?;

    //     if ratatui::crossterm::event::read()?.is_key_press() {
    //         break;
    //     }
    // }

    // ratatui::restore();
    Ok(())
}

fn render(frame: &mut Frame) {
    frame.render_widget("Hello World", frame.area());
}
