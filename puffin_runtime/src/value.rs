use std::{collections::HashMap, fmt::Display};

use serde_derive::{Deserialize, Serialize};

use crate::RuntimeError;

pub type IntType = i64;
pub type FloatType = f64;
pub type BoolType = bool;
pub type StringType = String;
pub type ObjectType = Object;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    Int(IntType),
    Float(FloatType),
    Bool(BoolType),
    String(StringType),
    Object(ObjectType),
}

impl From<IntType> for Value {
    fn from(value: IntType) -> Self {
        Value::Int(value)
    }
}

impl TryFrom<Value> for IntType {
    type Error = RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(v) => Ok(v),
            _ => Err(RuntimeError::IncorrectType { ty: value.type_name().to_owned(), expected: "int".to_owned() }),
        }
    }
}

impl From<FloatType> for Value {
    fn from(value: FloatType) -> Self {
        Value::Float(value)
    }
}

impl TryFrom<Value> for FloatType {
    type Error = RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Float(v) => Ok(v),
            _ => Err(RuntimeError::IncorrectType { ty: value.type_name().to_owned(), expected: "float".to_owned() }),
        }
    }
}

impl From<BoolType> for Value {
    fn from(value: BoolType) -> Self {
        Value::Bool(value)
    }
}

impl TryFrom<Value> for BoolType {
    type Error = RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bool(v) => Ok(v),
            _ => Err(RuntimeError::IncorrectType { ty: value.type_name().to_owned(), expected: "bool".to_owned() }),
        }
    }
}

impl From<StringType> for Value {
    fn from(value: StringType) -> Self {
        Value::String(value)
    }
}

impl TryFrom<Value> for StringType {
    type Error = RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(s) => Ok(s),
            _ => Err(RuntimeError::IncorrectType { ty: value.type_name().to_owned(), expected: "string".to_owned() }),
        }
    }
}

impl<'a> From<&'a str> for Value {
    fn from(value: &'a str) -> Self {
        Value::String(value.to_owned())
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(v) => f.write_fmt(format_args!("{v}")),
            Value::Float(v) => f.write_fmt(format_args!("{v}")),
            Value::Bool(v) => f.write_fmt(format_args!("{v}")),
            Value::String(v) => f.write_fmt(format_args!("{v}")),
            Value::Object(v) => f.write_fmt(format_args!("{v}")),
        }
    }
}

impl Value {
    pub fn try_add(self, rhs: Self) -> Result<Value, RuntimeError> {
        match self {
            Value::Int(lhs) => match rhs {
                Value::Int(rhs) => Ok(Value::Int(lhs + rhs)),
                Value::Float(rhs) => Ok(Value::Float(lhs as FloatType + rhs)),
                _ => Err(RuntimeError::InvalidBinaryOperation { op: "add".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() })
            },
            Value::Float(lhs) => match rhs {
                Value::Int(rhs) => Ok(Value::Float(lhs + rhs as FloatType)),
                Value::Float(rhs) => Ok(Value::Float(lhs + rhs)),
                _ => Err(RuntimeError::InvalidBinaryOperation { op: "add".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
            },

            Value::String(lhs) => Ok(Value::String(format!("{lhs}{}", rhs.to_owned()))),

            _ => Err(RuntimeError::InvalidBinaryOperation { op: "add".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
        }
    }

    pub fn try_sub(self, rhs: Self) -> Result<Value, RuntimeError> {
        match self {
            Value::Int(lhs) => match rhs {
                Value::Int(rhs) => Ok(Value::Int(lhs - rhs)),
                Value::Float(rhs) => Ok(Value::Float(lhs as FloatType - rhs)),
                _ => Err(RuntimeError::InvalidBinaryOperation { op: "subtract".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
            },
            Value::Float(lhs) => match rhs {
                Value::Int(rhs) => Ok(Value::Float(lhs - rhs as FloatType)),
                Value::Float(rhs) => Ok(Value::Float(lhs - rhs)),
                _ => Err(RuntimeError::InvalidBinaryOperation { op: "subtract".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
            },

            _ => Err(RuntimeError::InvalidBinaryOperation { op: "subtract".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
        }
    }

    pub fn try_div(self, rhs: Self) -> Result<Value, RuntimeError> {
        match self {
            Value::Int(lhs) => match rhs {
                Value::Int(0) => Err(RuntimeError::DivideByZero),
                Value::Int(rhs) => Ok(Value::Int(lhs / rhs)),

                Value::Float(0.0) => Err(RuntimeError::DivideByZero),
                Value::Float(rhs) => Ok(Value::Float(lhs as FloatType / rhs)),

                _ => Err(RuntimeError::InvalidBinaryOperation { op: "divide".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
            },
            Value::Float(lhs) => match rhs {
                Value::Int(0) => Err(RuntimeError::DivideByZero),
                Value::Int(rhs) => Ok(Value::Float(lhs / rhs as FloatType)),

                Value::Float(0.0) => Err(RuntimeError::DivideByZero),
                Value::Float(rhs) => Ok(Value::Float(lhs / rhs)),

                _ => Err(RuntimeError::InvalidBinaryOperation { op: "divide".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
            },

            _ => Err(RuntimeError::InvalidBinaryOperation { op: "divide".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
        }
    }

    pub fn try_mul(self, rhs: Self) -> Result<Value, RuntimeError> {
        match self {
            Value::Int(lhs) => match rhs {
                Value::Int(rhs) => Ok(Value::Int(lhs * rhs)),
                Value::Float(rhs) => Ok(Value::Float(lhs as FloatType * rhs)),
                _ => Err(RuntimeError::InvalidBinaryOperation { op: "multiply".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
            },
            Value::Float(lhs) => match rhs {
                Value::Int(rhs) => Ok(Value::Float(lhs * rhs as FloatType)),
                Value::Float(rhs) => Ok(Value::Float(lhs * rhs)),
                _ => Err(RuntimeError::InvalidBinaryOperation { op: "multiply".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
            },

            _ => Err(RuntimeError::InvalidBinaryOperation { op: "multiply".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
        }
    }

    pub fn try_mod(self, rhs: Self) -> Result<Value, RuntimeError> {
        match self {
            Value::Int(lhs) => match rhs {
                Value::Int(rhs) => Ok(Value::Int(lhs % rhs)),
                Value::Float(rhs) => Ok(Value::Float(lhs as FloatType % rhs)),
                _ => Err(RuntimeError::InvalidBinaryOperation { op: "modulo".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
            },
            Value::Float(lhs) => match rhs {
                Value::Int(rhs) => Ok(Value::Float(lhs % rhs as FloatType)),
                Value::Float(rhs) => Ok(Value::Float(lhs % rhs)),
                _ => Err(RuntimeError::InvalidBinaryOperation { op: "modulo".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
            },

            _ => Err(RuntimeError::InvalidBinaryOperation { op: "modulo".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
        }
    }

    pub fn try_negate(self) -> Result<Value, RuntimeError> {
        match self {
            Value::Int(lhs) => Ok(Value::Int(-lhs)),
            Value::Float(lhs) => Ok(Value::Float(-lhs)),
            _ => Err(RuntimeError::InvalidUnaryOperation { op: "negate".to_owned(), rhs_type: self.type_name().to_owned() }),
        }
    }

    pub fn try_not(self) -> Result<Value, RuntimeError> {
        match self {
            Value::Int(lhs) => Ok(Value::Bool(lhs != 0)),
            Value::Float(lhs) => Ok(Value::Bool(lhs != 0.0)),
            Value::Bool(lhs) => Ok(Value::Bool(!lhs)),
            _ => Err(RuntimeError::InvalidUnaryOperation { op: "not".to_owned(), rhs_type: self.type_name().to_owned() }),
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::Bool(_) => "bool",
            Value::String(_) => "string",
            Value::Object(_) => "object",
        }
    }

    pub fn as_string(self) -> Result<StringType, RuntimeError> {
        TryInto::<StringType>::try_into(self)
    }

    pub fn as_int(self) -> Result<IntType, RuntimeError> {
        TryInto::<IntType>::try_into(self)
    }

    pub fn as_float(self) -> Result<FloatType, RuntimeError> {
        TryInto::<FloatType>::try_into(self)
    }

    pub fn as_bool(self) -> Result<BoolType, RuntimeError> {
        TryInto::<BoolType>::try_into(self)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Object {
    fields: HashMap<String, Value>,
}

impl Object {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_field(&mut self, name: String, value: Value) {
        self.fields.insert(name, value);
    }

    pub fn get_field(&self, name: impl AsRef<str>) -> Option<&Value> {
        self.fields.get(name.as_ref())
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[object Object]")
    }
}
