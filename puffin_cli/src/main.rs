use std::{fs::File, path::PathBuf};

use clap::{Parser, Subcommand};
use puffin_runtime::{Chunk, Value, op::OpCode, run};

#[cfg(feature = "logging")]
use simplelog::{Config, LevelFilter, WriteLogger};

#[derive(Subcommand, Debug)]
enum Operation {
    Run {
        #[arg(long, action, default_value_t = false)]
        ir: bool,
        input: PathBuf,
    },
    Compile {
        #[arg(long, action, default_value_t = false)]
        ir: bool,
        input: PathBuf,
        output: Option<PathBuf>,
    },
    Test,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    op: Operation
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    
    #[cfg(feature = "logging")]
    WriteLogger::init(LevelFilter::Debug, Config::default(), File::create("latest.log")?)?;

    let args = Args::parse();

    match args.op {
        Operation::Compile { input, output, ir } => {
            let chunk = if ir {
                let file = File::open(&input)?;
                puffin_compiler::ir::compile(input.file_name().unwrap().to_str().unwrap(), file)?
            } else {
                test_chunk()
            };
            let file = File::create(output.unwrap_or("out.pfb".into()))?;
            ciborium::into_writer(&chunk, file)?;
        },
        Operation::Run { input, ir } => {
            let file = File::open(&input)?;

            let chunk = if ir {
                puffin_compiler::ir::compile(input.file_name().unwrap().to_str().unwrap(), file)?
            } else {
                ciborium::from_reader::<puffin_runtime::Chunk, File>(file)?
            };

            #[cfg(feature = "logging")]
            log::debug!("-- Running chunk --\n{chunk}");

            run(&chunk)?;
        },
        Operation::Test => {
            let chunk = test_chunk_2();
            
            #[cfg(feature = "logging")]
            log::info!("-- Running chunk --\n{chunk}");
            
            run(&chunk)?;
        },
    }

    Ok(())
}

fn test_chunk_2() -> Chunk {
    let mut chunk = puffin_runtime::Chunk::new("Test Program 2");
    
    // poll
    chunk.push_op(OpCode::Poll);
    
    // for i in 0:10 {
    chunk.push_constant(0);
    let i_offset = 0;
    let start_loop = chunk.addr();
    chunk.push_op(OpCode::GetLocal);
    chunk.push_local_offset(i_offset);
    chunk.push_constant(10);
    chunk.push_op(OpCode::Ge);
    let end_jump = chunk.push_jump(OpCode::JumpIf);
    
    // render i
    chunk.push_op(OpCode::GetLocal);
    chunk.push_local_offset(i_offset);
    chunk.push_op(OpCode::Render);
    
    // poll
    chunk.push_op(OpCode::Poll);
    
    // }
    chunk.push_op(OpCode::GetLocal);
    chunk.push_local_offset(i_offset);
    chunk.push_constant(1);
    chunk.push_op(OpCode::Add);
    chunk.push_op(OpCode::SetLocal);
    chunk.push_local_offset(i_offset);
    chunk.push_jump_im(OpCode::Jump, start_loop);
    chunk.patch_jump(end_jump, chunk.addr());
    
    chunk
}

fn test_chunk() -> Chunk {
    let mut chunk = puffin_runtime::Chunk::new("Test Program");

    // var foo = 10;
    chunk.push_constant(Value::Int(10));
    let foo_offset = 0;

    // static bar = 4;
    chunk.push_constant(Value::Int(4));
    let bar_name = chunk.new_constant(Value::String("bar".into()));
    chunk.push_op(OpCode::SetGlobal);
    chunk.push_constant_offset(bar_name);

    // foo = foo + 8 / bar;
    chunk.push_op(OpCode::GetLocal);
    chunk.push_local_offset(foo_offset);
    chunk.push_constant(Value::Int(8));
    chunk.push_op(OpCode::GetGlobal);
    chunk.push_constant_offset(bar_name);
    chunk.push_op(OpCode::Div);
    chunk.push_op(OpCode::Add);
    chunk.push_op(OpCode::SetLocal);
    chunk.push_local_offset(foo_offset);

    // render foo;
    chunk.push_op(OpCode::GetLocal);
    chunk.push_local_offset(foo_offset);
    chunk.push_op(OpCode::Render);

    // poll;
    chunk.push_op(OpCode::Poll);

    // render foo * foo
    chunk.push_op(OpCode::GetLocal);
    chunk.push_local_offset(foo_offset);
    chunk.push_op(OpCode::GetLocal);
    chunk.push_local_offset(foo_offset);
    chunk.push_op(OpCode::Mul);
    chunk.push_op(OpCode::Render);

    // poll;
    chunk.push_op(OpCode::Poll);

    // render "Hello";
    chunk.push_constant("Hello");
    chunk.push_op(OpCode::Render);

    // poll;
    chunk.push_op(OpCode::Poll);

    // render "World";
    chunk.push_constant("World");
    chunk.push_op(OpCode::Render);

    // poll;
    chunk.push_op(OpCode::Poll);

    // var baz = {};
    chunk.push_op(OpCode::NewObject);
    let baz_offset = 1;

    // baz.bar = "Hello!";
    chunk.push_op(OpCode::Constant);
    chunk.push_constant_offset(bar_name);
    chunk.push_constant("Hello!");
    chunk.push_op(OpCode::SetField);
    chunk.push_local_offset(baz_offset);

    // render baz.bar;
    chunk.push_op(OpCode::Constant);
    chunk.push_constant_offset(bar_name);
    chunk.push_op(OpCode::GetField);
    chunk.push_local_offset(baz_offset);
    chunk.push_op(OpCode::Render);


    // poll;
    chunk.push_op(OpCode::Poll);

    chunk
}
