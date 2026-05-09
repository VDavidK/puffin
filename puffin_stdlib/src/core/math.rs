use puffin_runtime::chunk::LocalOffset;
use puffin_runtime::value::{FloatType, Module, NativeFunction, Value};
use crate::declaration::Declaration;
use puffin_runtime::RuntimeError;

struct Math;

impl Declaration for Math {
    const NAME: &'static str = "math";

    fn declare(module: &mut Module) {
        module.set_item("pow", NativeFunction::new(|runtime, _, _| {
            let a = runtime.get_local(-2)?
                .to_owned().eval()?;

            let pow = runtime.get_local(-1)?.to_owned().eval()?;

            let value: Value = match (a, pow) {
                (Value::Int(lhs), Value::Int(rhs)) => {
                    if rhs < 0 {
                        (1 / (lhs.pow(-rhs as u32))).into()
                    } else {
                        lhs.pow(rhs as u32).into()
                    }
                },
                (Value::Float(lhs), Value::Float(rhs)) => lhs.powf(rhs).into(),
                (Value::Int(lhs), Value::Float(rhs)) => ((lhs as f64).powf(rhs)).into(),
                (Value::Float(lhs), Value::Int(rhs)) => {
                    if rhs < 0 {
                        (1.0 / (lhs.powi(-rhs as i32))).into()
                    } else {
                        lhs.powi(rhs as i32).into()
                    }
                },
                (Value::Int(_) | Value::Float(_), p) => Err(RuntimeError::IncorrectType {ty: p.type_name().to_owned(), expected: "int or float".to_owned() })?,
                (v, _) => Err(RuntimeError::IncorrectType { ty: v.type_name().to_owned(), expected: "float or int".to_owned() })?,
            };
            Ok(value)
        }));
        module.set_item("cos", NativeFunction::new(|runtime, _, _| {
            let val = runtime.get_local(-1)?.to_owned().eval()?;

            let value: Value = match val {
                Value::Int(v) => (v as f64).cos().into(),
                Value::Float(v) => v.cos().into(),
                v => Err(RuntimeError::IncorrectType { ty: v.type_name().to_owned(), expected: "float or int".to_owned() })?,
            };
            Ok(value)
        }));
        module.set_item("sin", NativeFunction::new(|runtime, _, _| {
            let val = runtime.get_local(-1)?.to_owned().eval()?;

            let value: Value = match val {
                Value::Int(v) => (v as f64).sin().into(),
                Value::Float(v) => v.sin().into(),
                v => Err(RuntimeError::IncorrectType { ty: v.type_name().to_owned(), expected: "float or int".to_owned() })?,
            };
            Ok(value)
        }));
        module.set_item("tan", NativeFunction::new(|runtime, _, _| {
            let val = runtime.get_local(-1)?.to_owned().eval()?;

            let value: Value = match val {
                Value::Int(v) => (v as f64).tan().into(),
                Value::Float(v) => v.tan().into(),
                v => Err(RuntimeError::IncorrectType { ty: v.type_name().to_owned(), expected: "float or int".to_owned() })?,
            };
            Ok(value)
        }));
        module.set_item("acos", NativeFunction::new(|runtime, _, _| {
            let val = runtime.get_local(-1)?.to_owned().eval()?;

            let value: Value = match val {
                Value::Int(v) => (v as f64).acos().into(),
                Value::Float(v) => v.acos().into(),
                v => Err(RuntimeError::IncorrectType { ty: v.type_name().to_owned(), expected: "float or int".to_owned() })?,
            };
            Ok(value)
        }));
        module.set_item("asin", NativeFunction::new(|runtime, _, _| {
            let val = runtime.get_local(-1)?.to_owned().eval()?;

            let value: Value = match val {
                Value::Int(v) => (v as f64).asin().into(),
                Value::Float(v) => v.asin().into(),
                v => Err(RuntimeError::IncorrectType { ty: v.type_name().to_owned(), expected: "float or int".to_owned() })?,
            };
            Ok(value)
        }));
        module.set_item("atan", NativeFunction::new(|runtime, _, _| {
            let val = runtime.get_local(-1)?.to_owned().eval()?;

            let value: Value = match val {
                Value::Int(v) => (v as f64).atan().into(),
                Value::Float(v) => v.atan().into(),
                v => Err(RuntimeError::IncorrectType { ty: v.type_name().to_owned(), expected: "float or int".to_owned() })?,
            };
            Ok(value)
        }));
        module.set_item("to_rad", NativeFunction::new(|runtime, _, _| {
            let val = runtime.get_local(-1)?.to_owned().eval()?;

            let value: Value = match val {
                Value::Int(v) => (v as f64).to_radians().into(),
                Value::Float(v) => v.to_radians().into(),
                v => Err(RuntimeError::IncorrectType { ty: v.type_name().to_owned(), expected: "float or int".to_owned() })?,
            };
            Ok(value)
        }));
        module.set_item("to_deg", NativeFunction::new(|runtime, _, _| {
            let val = runtime.get_local(-1)?.to_owned().eval()?;

            let value: Value = match val {
                Value::Int(v) => (v as f64).to_degrees().into(),
                Value::Float(v) => v.to_degrees().into(),
                v => Err(RuntimeError::IncorrectType { ty: v.type_name().to_owned(), expected: "float or int".to_owned() })?,
            };
            Ok(value)
        }));
        module.set_item("max", NativeFunction::new(|runtime, argc, _| {
            let mut max_val = Value::Null;
            for i in -(argc as isize)..0 {
                let local = runtime.get_local(i as LocalOffset)?.to_owned().eval()?;
                max_val = match (local, max_val) {
                    (Value::Int(a), Value::Int(b)) => a.max(b).into(),
                    (Value::Float(a), Value::Float(b)) => a.max(b).into(),
                    (Value::Int(a), Value::Float(b)) => b.max(a as FloatType).into(),
                    (Value::Float(a), Value::Int(b)) => a.max(b as FloatType).into(),
                    (Value::Int(a), Value::Null) => a.into(),
                    (Value::Float(a), Value::Null) => a.into(),
                    (v, _) => Err(RuntimeError::IncorrectType { ty: v.type_name().to_owned(), expected: "float or int".to_owned() })?,
                }
            };

            Ok(max_val)
        }));
        module.set_item("min", NativeFunction::new(|runtime, argc, _| {
            let mut max_val = Value::Null;
            for i in -(argc as isize)..0 {
                let local = runtime.get_local(i as LocalOffset)?.to_owned().eval()?;
                max_val = match (local, max_val) {
                    (Value::Int(a), Value::Int(b)) => a.min(b).into(),
                    (Value::Float(a), Value::Float(b)) => a.min(b).into(),
                    (Value::Int(a), Value::Float(b)) => b.min(a as FloatType).into(),
                    (Value::Float(a), Value::Int(b)) => a.min(b as FloatType).into(),
                    (Value::Int(a), Value::Null) => a.into(),
                    (Value::Float(a), Value::Null) => a.into(),
                    (v, _) => Err(RuntimeError::IncorrectType { ty: v.type_name().to_owned(), expected: "float or int".to_owned() })?,
                }
            };

            Ok(max_val)
        }));
        module.set_item("clamp", NativeFunction::new(|runtime, _, _| {
            let value = runtime.get_local(-3)?
                .to_owned().eval()?;

            let min = runtime.get_local(-2)?.to_owned().eval()?;
            let max = runtime.get_local(-1)?.to_owned().eval()?;

            let clamped = match (value, min, max) {
                (Value::Int(x), Value::Int(y), Value::Int(z)) => x.clamp(y, z).into(),
                (Value::Int(x), Value::Int(y), Value::Float(z)) => (x as FloatType).clamp(y as FloatType, z).into(),
                (Value::Int(x), Value::Float(y), Value::Int(z)) => (x as FloatType).clamp(y, z as FloatType).into(),
                (Value::Int(x), Value::Float(y), Value::Float(z)) => (x as FloatType).clamp(y, z).into(),
                (Value::Float(x), Value::Float(y), Value::Float(z)) => x.clamp(y, z).into(),
                (Value::Float(x), Value::Float(y), Value::Int(z)) => x.clamp(y, z as FloatType).into(),
                (Value::Float(x), Value::Int(y), Value::Float(z)) => x.clamp(y as FloatType, z).into(),
                (Value::Float(x), Value::Int(y), Value::Int(z)) => x.clamp(y as FloatType, z as FloatType).into(),
                (x, y, z) => Err(RuntimeError::IncorrectType { ty: format!("{}, {}, {}", x.type_name(), y.type_name(), z.type_name()), expected: "float or int".to_owned() })?,
            };

            Ok(clamped)
        }));
    }
}

pub fn module() -> Module {
    let mut module = Module::new(Math::NAME);
    Math::declare(&mut module);

    module
}
