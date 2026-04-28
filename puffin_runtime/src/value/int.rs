use crate::RuntimeError;
use crate::value::{FloatType, Value};
use crate::value::ops::{ValueAdd, ValueDiv, ValueMod, ValueSub, ValueMul, ValueNeg, ValueDef, ValueTruthy};

pub type IntType = i64;

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

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Value::Int(value as IntType)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Int(value as IntType)
    }
}

impl ValueAdd for IntType {
    fn try_add(&self, rhs: &Value) -> Result<Value, RuntimeError> {
        match rhs {
            Value::Int(inner) => Ok(Value::Int(self + inner)),
            Value::Float(inner) => Ok(Value::Float(*self as FloatType + inner)),
            _ => Err(RuntimeError::InvalidBinaryOperation { op: "add".to_owned(), lhs_type: Self::type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
        }
    }
}

impl ValueSub for IntType {
    fn try_sub(&self, rhs: &Value) -> Result<Value, RuntimeError> {
        match rhs {
            Value::Int(inner) => Ok(Value::Int(self - inner)),
            Value::Float(inner) => Ok(Value::Float(*self as FloatType - inner)),
            _ => Err(RuntimeError::InvalidBinaryOperation { op: "sub".to_owned(), lhs_type: Self::type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
        }
    }
}

impl ValueMul for IntType {
    fn try_mul(&self, rhs: &Value) -> Result<Value, RuntimeError> {
        match rhs {
            Value::Int(inner) => Ok(Value::Int(self * inner)),
            Value::Float(inner) => Ok(Value::Float(*self as FloatType * inner)),
            _ => Err(RuntimeError::InvalidBinaryOperation { op: "mul".to_owned(), lhs_type: Self::type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
        }
    }
}

impl ValueDiv for IntType {
    fn try_div(&self, rhs: &Value) -> Result<Value, RuntimeError> {
        match rhs {
            Value::Int(0) => Err(RuntimeError::DivideByZero),
            Value::Int(inner) => Ok(Value::Int(self / inner)),
            Value::Float(inner) => Ok(Value::Float(*self as FloatType / inner)),
            _ => Err(RuntimeError::InvalidBinaryOperation { op: "div".to_owned(), lhs_type: Self::type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
        }
    }
}

impl ValueMod for IntType {
    fn try_mod(&self, rhs: &Value) -> Result<Value, RuntimeError> {
        match rhs {
            Value::Int(rhs) => Ok(Value::Int(self % rhs)),
            Value::Float(rhs) => Ok(Value::Float(*self as FloatType % rhs)),
            _ => Err(RuntimeError::InvalidBinaryOperation { op: "modulo".to_owned(), lhs_type: Self::type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
        }
    }
}

impl ValueNeg for IntType {
    fn try_neg(&self) -> Result<Value, RuntimeError> {
        Ok(Value::Int(-self))
    }
}

impl ValueTruthy for IntType {
    fn truthy(&self) -> bool {
        *self != 0
    }
}

impl ValueDef for IntType {
    const TYPE_NAME: &'static str = "int";
}