use clap::{Parser, Subcommand};
use color_eyre::eyre::OptionExt;
use puffin_runtime::Chunk;
use std::io::Read;
use std::rc::Rc;
use std::{fs::File, path::PathBuf};
use std::path::Path;
use puffin_runtime::runtime::Runtime;
#[cfg(feature = "logging")]
use simplelog::{Config, LevelFilter, WriteLogger};
use puffin_runtime::dom::Dom;

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
            let mut chunks = vec![];
            compile_file(&input, &mut chunks)?;

            let mut runtime = Runtime::new()?;
            runtime.include_module(puffin_stdlib::core::fs::module())?;
            runtime.include_module(puffin_stdlib::core::math::module())?;
            runtime.include_module(puffin_stdlib::core::list::module())?;
            runtime.include_module(puffin_stdlib::core::dict::module())?;

            puffin_stdlib::base::define(&mut runtime)?;

            let main_component_name = input.file_stem()
                .ok_or_eyre("File path must have a valid stem")?
                .to_str()
                .ok_or_eyre("File path must be a valid string")?
                .to_owned();

            for chunk in chunks {
                runtime.execute(Rc::new(chunk))?;
            }

            let main_component = runtime.get_global(main_component_name)
                .ok_or_eyre("Main component missing")?
                .to_owned();

            let mut dom = Dom::new(main_component, &mut runtime)?;
            dom.run(&mut runtime)?;
        }
    }

    Ok(())
}

fn compile_file(path: impl AsRef<Path>, chunks: &mut Vec<Chunk>) -> color_eyre::Result<()> {
    let mut file = File::open(&path)?;
    let input_path_str = path
        .as_ref()
        .to_str()
        .ok_or_eyre("File name must be valid UTF-8 name")?;

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    let ast = puffin_parser::run_parser(file_contents, input_path_str)?;
    let (chunk, deps) = puffin_compiler::compile_ast(input_path_str, &ast)?;

    #[cfg(feature = "logging")]
    log::debug!("Compiling chunk: {}\n{}", chunk.get_name(), chunk);

    chunks.push(chunk);
    for dep in deps {
        let base_dir = path.as_ref()
            .parent()
            .ok_or_eyre(format!("Could not find dependency {}", dep.to_str().ok_or_eyre("Malformed file path")?))?;
        let file_path = base_dir.join(dep);
        compile_file(file_path, chunks)?;
    }
    Ok(())
}
