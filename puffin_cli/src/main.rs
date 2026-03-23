use clap::{Parser, Subcommand};
use color_eyre::eyre::OptionExt;
use puffin_runtime::Chunk;
use std::io::Read;
use std::rc::Rc;
use std::{fs::File, path::PathBuf};

use puffin_runtime::vm::Vm;
#[cfg(feature = "logging")]
use simplelog::{Config, LevelFilter, WriteLogger};

#[derive(Subcommand, Debug)]
enum Operation {
    Run {
        input: PathBuf,
    },
    Compile {
        input: PathBuf,
        output: Option<PathBuf>,
    },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    op: Operation,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    #[cfg(feature = "logging")]
    WriteLogger::init(
        LevelFilter::Debug,
        Config::default(),
        File::create("latest.log")?,
    )?;

    let args = Args::parse();

    match args.op {
        Operation::Compile { input, output } => {
            // Open input file
            let mut input_file = File::open(&input)?;
            let input_path_str = input
                .to_str()
                .ok_or_eyre("File name must be valid UTF-8 name")?;

            // Read source
            let mut source = String::new();
            input_file.read_to_string(&mut source)?;

            // Parse and compile source
            let ast = puffin_parser::run_parser(source, input_path_str)?;
            let chunk = puffin_compiler::compile_ast(input_path_str, &ast)?;

            // Write compiled output to disk
            let file = File::create(output.unwrap_or("out.pfb".into()))?;
            ciborium::into_writer(&chunk, file)?;
        }
        Operation::Run { input } => {
            let mut file = File::open(&input)?;
            let input_path_str = input
                .to_str()
                .ok_or_eyre("File name must be valid UTF-8 name")?;

            let chunk = if let Some(ext) = input.extension()
                && ext == "pfb"
            {
                ciborium::from_reader::<Chunk, File>(file)?
            } else {
                let mut file_contents = String::new();
                file.read_to_string(&mut file_contents)?;
                let ast = puffin_parser::run_parser(file_contents, input_path_str)?;
                puffin_compiler::compile_ast(input_path_str, &ast)?
            };

            #[cfg(feature = "logging")]
            log::debug!("-- Running chunk --\n{chunk}");

            let mut vm = Vm::new(Rc::new(chunk));

            vm.include_module(puffin_stdlib::core::dom::module());

            vm.run()?;

            let func = vm
                .get_global("main")
                .expect("Expected main function")
                .clone()
                .take_function()?;

            vm.call(func)?;
        }
    }

    Ok(())
}
