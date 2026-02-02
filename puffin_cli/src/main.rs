use std::{fs::File, path::PathBuf};

use clap::{Parser, Subcommand};
use puffin_runtime::{Value, op::OpCode, run};

#[derive(Subcommand, Debug)]
enum Operation {
    Run {
        input: PathBuf,
    },
    Compile {
        output: Option<PathBuf>,
    },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    op: Operation
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    match args.op {
        Operation::Compile { output } => {
            let mut chunk = puffin_runtime::Chunk::new("Test Program");

            // var foo = 10;
            chunk.push_literal(Value::Int(10));

            // static bar = 4;
            chunk.push_literal(Value::Int(4));
            let bar_name = chunk.new_literal(Value::String("bar".into()));
            chunk.push_op(OpCode::SetGlobal);
            chunk.push_u64(bar_name as u64);

            // foo = foo + 8 / bar;
            chunk.push_op(OpCode::GetLocal);
            chunk.push_u64(0);
            chunk.push_literal(Value::Int(8));
            chunk.push_op(OpCode::GetGlobal);
            chunk.push_u64(bar_name as u64);
            chunk.push_op(OpCode::Div);
            chunk.push_op(OpCode::Add);
            chunk.push_op(OpCode::SetLocal);
            chunk.push_u64(0);

            // render foo;
            chunk.push_op(OpCode::GetLocal);
            chunk.push_u64(0);
            chunk.push_op(OpCode::Render);

            // poll;
            chunk.push_op(OpCode::Poll);

            // render foo * foo
            chunk.push_op(OpCode::GetLocal);
            chunk.push_u64(0);
            chunk.push_op(OpCode::GetLocal);
            chunk.push_u64(0);
            chunk.push_op(OpCode::Mul);
            chunk.push_op(OpCode::Render);

            // poll;
            chunk.push_op(OpCode::Poll);

            // render "Hello";
            chunk.push_literal("Hello");
            chunk.push_op(OpCode::Render);

            // poll;
            chunk.push_op(OpCode::Poll);

            // render "World";
            chunk.push_literal("World");
            chunk.push_op(OpCode::Render);

            // poll;
            chunk.push_op(OpCode::Poll);

            // var baz = {};
            chunk.push_op(OpCode::NewObject);

            // baz.bar = "Hello!";
            chunk.push_op(OpCode::GetLocal);
            chunk.push_u64(1);
            chunk.push_literal("Hello!");
            chunk.push_op(OpCode::SetField);
            chunk.push_u64(bar_name as u64);

            // render baz.bar;
            chunk.push_op(OpCode::GetField);
            chunk.push_u64(bar_name as u64);
            chunk.push_op(OpCode::Render);


            // poll;
            chunk.push_op(OpCode::Poll);

            let file = File::create(output.unwrap_or("out.pfb".into()))?;
            ciborium::into_writer(&chunk, file)?;
        },
        Operation::Run { input } => {
            let file = File::open(input)?;
            let chunk = ciborium::from_reader::<puffin_runtime::Chunk, File>(file)?;

            println!("-- Running chunk --\n{chunk}");

            run(&chunk)?;
        }
    }

    Ok(())
}
