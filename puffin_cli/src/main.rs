use puffin_runtime::run_program;

fn main() {
    let program = puffin_runtime::Program::new();
    run_program(&program).unwrap();
}
