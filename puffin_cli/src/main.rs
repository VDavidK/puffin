use puffin_runtime::{Value, op::OpCode, run};

fn main() {
    let mut chunk = puffin_runtime::Chunk::new("Test Program");

    chunk.push_literal(Value::Int(32));
    chunk.push_literal(Value::Int(64));
    // chunk.push_op(OpCode::Add);
    chunk.push_op(OpCode::Print);
    chunk.push_op(OpCode::Print);

    run(&chunk).unwrap();

    println!("{chunk}");
}
