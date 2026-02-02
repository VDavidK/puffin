
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

    #[error("Unrecognized op code (0x{0:x}) at: 0x{1:x}")]
    UnrecognizedOpCode(u8, usize),

    #[error("Use of invalid op code at: 0x{0:x}")]
    InvalidOpCode(usize),

    #[error("Trying to access out of bounds memory (0x{0:x}) at: 0x{1:x}")]
    AccessOutOfBounds(usize, usize),

    #[error("Trying to access out of bounds literal (0x{0:x}) at: 0x{1:x}")]
    InvalidLiteralAccess(usize, usize),

    #[error("Unable to perform binary {0} operation on {1} and {2}")]
    InvalidBinaryOperation(String, String, String),

    #[error("Unable to perform unary {0} operation on {1}")]
    InvalidUnaryOperation(String, String),

    #[error("Attempt to divide by zero")]
    DivideByZero,

    #[error("Expected value on the stack but the stack was empty")]
    StackEmpty,
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
