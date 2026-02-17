use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use puffin_runtime::{Chunk, Value};
use puffin_runtime::chunk::{InstructionOffset, LiteralOffset};
use puffin_runtime::op::OpCode;
use puffin_runtime::value::{FloatType, IntType};

#[derive(Debug, thiserror::Error)]
pub enum IrError {
    #[error("Invalid op code received")]
    InvalidOpCode,

    #[error("Invalid literal received")]
    InvalidLiteral,

    #[error("Invalid offset received")]
    InvalidOffset,
}

pub fn compile(name: impl AsRef<str>, reader: impl Read) -> Result<Chunk, IrError> {
    let mut reader = BufReader::new(reader);
    let mut chunk = Chunk::new(name);
    let mut labels = HashMap::<String, InstructionOffset>::new();
    let mut literals = HashMap::<Value, LiteralOffset>::new();
    let mut unpatched_jumps = HashMap::<String, Vec<InstructionOffset>>::new();

    let mut buf = String::new();
    while let Ok(s) = reader.read_line(&mut buf) && s > 0 {
        let line = buf.clone();
        buf.clear();

        let mut content = line.trim();
        if let Some(idx) = content.find(';') {
            content = content[..idx].trim();
        }

        if content.is_empty() {
            continue;
        }

        let line = content.split(' ').collect::<Vec<_>>();
        let cmd = line[0];

        if cmd.ends_with(':') {
            let name = cmd[..cmd.len() - 1].to_string();

            if let Some(v) = unpatched_jumps.get(&name) {
                for jmp in v {
                    chunk.patch_jump(*jmp, chunk.addr());
                }
            }

            labels.insert(name, chunk.addr());
            continue;
        }

        let args = &line[1..];

        match get_op_code(cmd) {
            OpCode::Literal => match literals.entry(parse_literal(args[0])?) {
                Entry::Occupied(entry) => {
                    chunk.push_literal_offset(*entry.get());
                }
                Entry::Vacant(entry) => {
                    let offset = chunk.push_literal(entry.key().clone());
                    entry.insert(offset);
                }
            }

            OpCode::GetLocal => {
                chunk.push_op(OpCode::GetLocal);
                chunk.push_literal_offset(parse_offset(args[0])? as LiteralOffset);
            }
            OpCode::SetLocal => {
                chunk.push_op(OpCode::SetLocal);
                chunk.push_literal_offset(parse_offset(args[0])? as LiteralOffset);
            }
            OpCode::GetGlobal => {
                let literal = parse_literal(args[0])?;
                let offset = get_literal_offset(&mut chunk, &mut literals, literal);

                chunk.push_op(OpCode::GetGlobal);
                chunk.push_literal_offset(offset);
            }
            OpCode::SetGlobal => {
                let literal = parse_literal(args[0])?;
                let offset = get_literal_offset(&mut chunk, &mut literals, literal);

                chunk.push_op(OpCode::SetGlobal);
                chunk.push_literal_offset(offset);
            }
            OpCode::SetField => {
                chunk.push_op(OpCode::SetField);
                chunk.push_literal_offset(parse_offset(args[0])? as LiteralOffset);
            }
            OpCode::GetField => {
                chunk.push_op(OpCode::SetField);
                chunk.push_literal_offset(parse_offset(args[0])? as LiteralOffset);
            }
            OpCode::Jump => add_jump(&mut chunk, &mut labels, &mut unpatched_jumps, OpCode::Jump, args[0]),
            OpCode::JumpIf => add_jump(&mut chunk, &mut labels, &mut unpatched_jumps, OpCode::JumpIf, args[0]),

            OpCode::Invalid => return Err(IrError::InvalidOpCode),

            op => chunk.push_op(op),
        }
    }

    Ok(chunk)
}

fn add_jump(chunk: &mut Chunk, labels: &mut HashMap<String, InstructionOffset>, unpatched_jumps: &mut HashMap<String, Vec<InstructionOffset>>, jump: OpCode, label: impl AsRef<str>) {
    let label = parse_label(label);

    if let Some(offset) = labels.get(&label) {
        chunk.push_jump_im(jump, *offset);
    } else {
        let jump = chunk.push_jump(jump);
        if let Some(v) = unpatched_jumps.get_mut(&label) {
            v.push(jump);
        } else {
            unpatched_jumps.insert(label, vec![jump]);
        }
    }
}

fn get_literal_offset(chunk: &mut Chunk, literals: &mut HashMap<Value, LiteralOffset>, literal: Value) -> LiteralOffset {
    if !literals.contains_key(&literal) {
        let offset = chunk.push_literal(literal.clone());
        literals.insert(literal, offset);
        offset
    } else {
        *literals.get(&literal).unwrap()
    }
}

fn get_op_code(name: impl AsRef<str>) -> OpCode {
    match name.as_ref() {
        // Literal argument instructions
        "literal" => OpCode::Literal,
        "getg" => OpCode::GetGlobal,
        "setg" => OpCode::SetGlobal,

        // Local argument instructions
        "getf" => OpCode::GetField,
        "setf" => OpCode::SetField,
        "getl" => OpCode::GetLocal,
        "setl" => OpCode::SetLocal,

        // Address argument instructions
        "jmp" => OpCode::Jump,
        "jmpi" => OpCode::JumpIf,

        // No argument instructions
        "pop" => OpCode::Pop,
        "newobj" => OpCode::NewObject,
        "add" => OpCode::Add,
        "sub" => OpCode::Sub,
        "mul" => OpCode::Mul,
        "div" => OpCode::Div,
        "mod" => OpCode::Mod,
        "neg" => OpCode::Neg,
        "not" => OpCode::Not,
        "eq" => OpCode::Eq,
        "neq" => OpCode::Neq,
        "ge" => OpCode::Ge,
        "le" => OpCode::Le,
        "gt" => OpCode::Gt,
        "lt" => OpCode::Lt,
        "exit" => OpCode::Exit,
        "poll" => OpCode::Poll,
        "render" => OpCode::Render,

        _ => OpCode::Invalid,
    }
}

fn parse_offset(off: impl AsRef<str>) -> Result<usize, IrError> {
    off.as_ref().parse().map_err(|_| IrError::InvalidOffset)
}

fn parse_label(label: impl AsRef<str>) -> String {
    label.as_ref()[1..].to_string()
}

fn parse_literal(lit: impl AsRef<str>) -> Result<Value, IrError> {
    let lit = lit.as_ref();

    match lit.chars().next().ok_or(IrError::InvalidLiteral)? {
        '"' => Ok(Value::String(lit[1..lit.len() - 1].to_string())),

        't' if lit == "true" => Ok(Value::Bool(true)),
        'f' if lit == "false" => Ok(Value::Bool(false)),

        c if c.is_ascii_digit() => {
            // Parse float or int
            let value = lit.parse::<FloatType>()
                .map(|v| Value::Float(v))
                .map_err(|_| IrError::InvalidLiteral)
                .or_else(|err| lit.parse::<IntType>()
                    .map(|v| Value::Int(v))
                    .map_err(|_| err)
                )?;

            Ok(value)
        }

        _ => Err(IrError::InvalidLiteral),
    }
}
