use std::str::FromStr;
use puffin_runtime::runtime::Runtime;
use puffin_runtime::RuntimeError;
use puffin_runtime::value::{FloatType, IntType, NativeFunction, Value};

pub fn define_string_cast_fn(runtime: &mut Runtime) -> Result<(), RuntimeError>  {
    let func = NativeFunction::new(|runtime, _argc, _| {
        let val = runtime.get_local(-1)?.to_owned().eval()?;
        Ok(val.to_string().into())
    });

    runtime.add_global("str", func)?;
    Ok(())
}

pub fn define_int_cast_fn(runtime: &mut Runtime) -> Result<(), RuntimeError>  {
    let func = NativeFunction::new(|runtime, _argc, _| {
        let val = runtime.get_local(-1)?.to_owned().eval()?;
        let parsed = match val {
            Value::Int(i) => i.into(),
            Value::Float(f) => (f as IntType).into(),
            Value::String(s) => {
                if s.borrow().contains(".") {
                    (FloatType::from_str(&s.borrow().replace("_", ""))? as IntType).into()
                } else {
                    IntType::from_str(&s.borrow().replace("_", ""))?.into()
                }
            },
            v => Err(RuntimeError::IncorrectType { ty: v.type_name().to_owned(), expected: "int, float or string".to_owned() })?,
        };
        Ok(parsed)
    });

    runtime.add_global("int", func)?;
    Ok(())
}

pub fn define_float_cast_fn(runtime: &mut Runtime) -> Result<(), RuntimeError>  {
    let func = NativeFunction::new(|runtime, _argc, _| {
        let val = runtime.get_local(-1)?.to_owned().eval()?;
        let parsed = match val {
            Value::Int(i) => (i as FloatType).into(),
            Value::Float(f) => f.into(),
            Value::String(s) => FloatType::from_str(&s.borrow().replace("_", ""))?.into(),
            v => Err(RuntimeError::IncorrectType { ty: v.type_name().to_owned(), expected: "int, float or string".to_owned() })?,
        };
        Ok(parsed)
    });

    runtime.add_global("float", func)?;
    Ok(())
}

pub fn define_bool_cast_fn(runtime: &mut Runtime) -> Result<(), RuntimeError>  {
    let func = NativeFunction::new(|runtime, _argc, _| {
        let val = runtime.get_local(-1)?.to_owned().eval()?;
        Ok(val.truthy()?.into())
    });

    runtime.add_global("bool", func)?;
    Ok(())
}