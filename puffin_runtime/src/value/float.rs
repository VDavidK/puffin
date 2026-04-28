use crate::RuntimeError;
use crate::value::{Value};
use crate::value::ops::{ValueAdd, ValueDiv, ValueMod, ValueSub, ValueMul, ValueNeg, ValueDef, ValueTruthy};

pub type FloatType = f64;

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

impl ValueAdd for FloatType {
    fn try_add(&self, rhs: &Value) -> Result<Value, RuntimeError> {
        match rhs {
            Value::Int(rhs) => Ok(Value::Float(*self + *rhs as FloatType)),
            Value::Float(rhs) => Ok(Value::Float(*self + rhs)),
            _ => Err(RuntimeError::InvalidBinaryOperation { op: "add".to_owned(), lhs_type: Self::type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
        }
    }
}

impl ValueSub for FloatType {
    fn try_sub(&self, rhs: &Value) -> Result<Value, RuntimeError> {
        match rhs {
            Value::Int(rhs) => Ok(Value::Float(*self - *rhs as FloatType)),
            Value::Float(rhs) => Ok(Value::Float(*self - rhs)),
            _ => Err(RuntimeError::InvalidBinaryOperation { op: "subtract".to_owned(), lhs_type: Self::type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
        }
    }
}

impl ValueMul for FloatType {
    fn try_mul(&self, rhs: &Value) -> Result<Value, RuntimeError> {
        match rhs {
            Value::Int(rhs) => Ok(Value::Float(*self * *rhs as FloatType)),
            Value::Float(rhs) => Ok(Value::Float(*self * rhs)),
            _ => Err(RuntimeError::InvalidBinaryOperation { op: "multiply".to_owned(), lhs_type: Self::type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
        }
    }
}

impl ValueDiv for FloatType {
    fn try_div(&self, rhs: &Value) -> Result<Value, RuntimeError> {
        match rhs {
            Value::Int(0) => Err(RuntimeError::DivideByZero),
            Value::Int(rhs) => Ok(Value::Float(*self / *rhs as FloatType)),

            Value::Float(0.0) => Err(RuntimeError::DivideByZero),
            Value::Float(rhs) => Ok(Value::Float(*self / rhs)),

            _ => Err(RuntimeError::InvalidBinaryOperation { op: "divide".to_owned(), lhs_type: Self::type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
        }
    }
}

impl ValueMod for FloatType {
    fn try_mod(&self, rhs: &Value) -> Result<Value, RuntimeError> {
        match rhs {
            Value::Int(rhs) => Ok(Value::Float(*self % *rhs as FloatType)),
            Value::Float(rhs) => Ok(Value::Float(*self % rhs)),
            _ => Err(RuntimeError::InvalidBinaryOperation { op: "modulo".to_owned(), lhs_type: Self::type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
        }
    }
}

impl ValueNeg for FloatType {
    fn try_neg(&self) -> Result<Value, RuntimeError> {
        Ok(Value::Float(-self))
    }
}

impl ValueTruthy for FloatType {
    fn truthy(&self) -> bool {
        *self != 0.0
    }
}

impl ValueDef for FloatType {
    const TYPE_NAME: &'static str = "float";
}