mod function;
mod instance;
mod native_function;
mod class;
mod module;
mod native_value;

use std::{cell::RefCell, fmt::Display, hash::Hash, rc::Rc};
use std::sync::Arc;
pub use instance::{new_instance, Instance};
pub use function::Function;
pub use native_function::NativeFunction;
pub use class::{new_class, Class};
pub use module::{new_module, Module};
pub use native_value::{NativeValue, NativeValueTrait};

use serde_derive::{Deserialize, Serialize};
use crate::RuntimeError;

pub type IntType = i64;
pub type FloatType = f64;
pub type BoolType = bool;
pub type StringType = String;
pub type InstanceType = Rc<RefCell<Instance>>;
pub type ClassType = Rc<RefCell<Class>>;
pub type ModuleType = Rc<RefCell<Module>>;
pub type FunctionType = Rc<Function>;
pub type NativeFunctionType = Rc<NativeFunction>;
pub type NativeValueType = NativeValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    Int(IntType),
    Float(FloatType),
    Bool(BoolType),
    String(StringType),
    Function(FunctionType),
    Class(ClassType),
    Module(ModuleType),
    Null,

    #[serde(skip)]
    Instance(InstanceType),

    #[serde(skip)]
    NativeValue(NativeValueType),

    #[serde(skip)]
    NativeFunction(NativeFunctionType),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Instance(l0), Self::Instance(r0)) => l0.as_ptr() == r0.as_ptr(),
            (Self::Function(l0), Self::Function(r0)) => Rc::as_ptr(l0) == Rc::as_ptr(r0),
            (Self::NativeFunction(l0), Self::NativeFunction(r0)) => Rc::as_ptr(l0) == Rc::as_ptr(r0),
            (Self::Class(l0), Self::Class(r0)) => Rc::as_ptr(l0) == Rc::as_ptr(r0),
            (Self::Module(l0), Self::Module(r0)) => Rc::as_ptr(l0) == Rc::as_ptr(r0),
            (Self::NativeValue(l0), Self::NativeValue(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Eq for Value { }

impl Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
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

impl From<InstanceType> for Value {
    fn from(value: InstanceType) -> Self {
        Value::Instance(value)
    }
}

impl From<Instance> for Value {
    fn from(value: Instance) -> Self {
        Value::Instance(Rc::new(RefCell::new(value)))
    }
}

impl From<NativeValue> for Value {
    fn from(value: NativeValue) -> Self {
        Value::NativeValue(value)
    }
}

impl From<ClassType> for Value {
    fn from(value: ClassType) -> Self {
        Value::Class(value)
    }
}

impl From<Class> for Value {
    fn from(value: Class) -> Self {
        Value::Class(Rc::new(RefCell::new(value)))
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

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(v) => f.write_fmt(format_args!("{v}")),
            Value::Float(v) => f.write_fmt(format_args!("{v}")),
            Value::Bool(v) => f.write_fmt(format_args!("{v}")),
            Value::String(v) => f.write_fmt(format_args!("{v}")),
            Value::Instance(v) => f.write_fmt(format_args!("{}", v.borrow())),
            Value::Function(v) => f.write_fmt(format_args!("{}", v)),
            Value::NativeFunction(v) => f.write_fmt(format_args!("{}", v)),
            Value::Class(v) => f.write_fmt(format_args!("{}", v.borrow())),
            Value::Module(v) => f.write_fmt(format_args!("{}", v.borrow())),
            Value::NativeValue(v) => f.write_fmt(format_args!("{}", v)),
            Value::Null => f.write_fmt(format_args!("null")),
        }
    }
}

impl Value {
    pub fn try_add(&self, rhs: &Self) -> Result<Value, RuntimeError> {
        match self {
            Value::Int(lhs) => match rhs {
                Value::Int(rhs) => Ok(Value::Int(lhs + rhs)),
                Value::Float(rhs) => Ok(Value::Float(*lhs as FloatType + rhs)),
                _ => Err(RuntimeError::InvalidBinaryOperation { op: "add".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() })
            },
            Value::Float(lhs) => match rhs {
                Value::Int(rhs) => Ok(Value::Float(lhs + *rhs as FloatType)),
                Value::Float(rhs) => Ok(Value::Float(lhs + rhs)),
                _ => Err(RuntimeError::InvalidBinaryOperation { op: "add".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
            },

            Value::String(lhs) => Ok(Value::String(format!("{lhs}{}", rhs.to_owned()))),

            _ => Err(RuntimeError::InvalidBinaryOperation { op: "add".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
        }
    }

    pub fn try_sub(&self, rhs: &Self) -> Result<Value, RuntimeError> {
        match self {
            Value::Int(lhs) => match rhs {
                Value::Int(rhs) => Ok(Value::Int(lhs - rhs)),
                Value::Float(rhs) => Ok(Value::Float(*lhs as FloatType - rhs)),
                _ => Err(RuntimeError::InvalidBinaryOperation { op: "subtract".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
            },
            Value::Float(lhs) => match rhs {
                Value::Int(rhs) => Ok(Value::Float(lhs - *rhs as FloatType)),
                Value::Float(rhs) => Ok(Value::Float(lhs - rhs)),
                _ => Err(RuntimeError::InvalidBinaryOperation { op: "subtract".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
            },

            _ => Err(RuntimeError::InvalidBinaryOperation { op: "subtract".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
        }
    }

    pub fn try_div(&self, rhs: &Self) -> Result<Value, RuntimeError> {
        match self {
            Value::Int(lhs) => match rhs {
                Value::Int(0) => Err(RuntimeError::DivideByZero),
                Value::Int(rhs) => Ok(Value::Int(lhs / rhs)),

                Value::Float(0.0) => Err(RuntimeError::DivideByZero),
                Value::Float(rhs) => Ok(Value::Float(*lhs as FloatType / rhs)),

                _ => Err(RuntimeError::InvalidBinaryOperation { op: "divide".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
            },
            Value::Float(lhs) => match rhs {
                Value::Int(0) => Err(RuntimeError::DivideByZero),
                Value::Int(rhs) => Ok(Value::Float(lhs / *rhs as FloatType)),

                Value::Float(0.0) => Err(RuntimeError::DivideByZero),
                Value::Float(rhs) => Ok(Value::Float(lhs / rhs)),

                _ => Err(RuntimeError::InvalidBinaryOperation { op: "divide".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
            },

            _ => Err(RuntimeError::InvalidBinaryOperation { op: "divide".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
        }
    }

    pub fn try_mul(&self, rhs: &Self) -> Result<Value, RuntimeError> {
        match self {
            Value::Int(lhs) => match rhs {
                Value::Int(rhs) => Ok(Value::Int(lhs * rhs)),
                Value::Float(rhs) => Ok(Value::Float(*lhs as FloatType * rhs)),
                _ => Err(RuntimeError::InvalidBinaryOperation { op: "multiply".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
            },
            Value::Float(lhs) => match rhs {
                Value::Int(rhs) => Ok(Value::Float(lhs * *rhs as FloatType)),
                Value::Float(rhs) => Ok(Value::Float(lhs * rhs)),
                _ => Err(RuntimeError::InvalidBinaryOperation { op: "multiply".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
            },

            _ => Err(RuntimeError::InvalidBinaryOperation { op: "multiply".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
        }
    }

    pub fn try_mod(&self, rhs: &Self) -> Result<Value, RuntimeError> {
        match self {
            Value::Int(lhs) => match rhs {
                Value::Int(rhs) => Ok(Value::Int(lhs % rhs)),
                Value::Float(rhs) => Ok(Value::Float(*lhs as FloatType % rhs)),
                _ => Err(RuntimeError::InvalidBinaryOperation { op: "modulo".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
            },
            Value::Float(lhs) => match rhs {
                Value::Int(rhs) => Ok(Value::Float(lhs % *rhs as FloatType)),
                Value::Float(rhs) => Ok(Value::Float(lhs % rhs)),
                _ => Err(RuntimeError::InvalidBinaryOperation { op: "modulo".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
            },

            _ => Err(RuntimeError::InvalidBinaryOperation { op: "modulo".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
        }
    }

    pub fn is_equal(&self, rhs: &Self) -> bool {
        match self {
            Value::Int(lhs) => match rhs {
                Value::Int(rhs) => *lhs == *rhs,
                Value::Float(rhs) => *lhs as FloatType == *rhs,
                _ => false,
            },
            Value::Float(lhs) => match rhs {
                Value::Int(rhs) => *lhs == *rhs as FloatType,
                Value::Float(rhs) => *lhs == *rhs,
                _ => false,
            },

            Value::Bool(lhs) => match rhs {
                Value::Bool(rhs) => *lhs == *rhs,

                _ => false,
            },

            Value::String(lhs) => match rhs {
                Value::String(rhs) => *lhs == *rhs,

                _ => false,
            },

            Value::Instance(lhs) => match rhs {
                Value::Instance(rhs) => lhs.as_ptr() == rhs.as_ptr(),

                _ => false,
            }

            Value::Function(lhs) => match rhs {
                Value::Function(rhs) => Rc::as_ptr(lhs) == Rc::as_ptr(rhs),

                _ => false,
            }

            Value::NativeFunction(lhs) => match rhs {
                Value::NativeFunction(rhs) => Rc::as_ptr(lhs) == Rc::as_ptr(rhs),

                _ => false,
            }

            Value::Class(lhs) => match rhs {
                Value::Class(rhs) => Rc::as_ptr(lhs) == Rc::as_ptr(rhs),

                _ => false,
            }

            Value::Module(lhs) => match rhs {
                Value::Module(rhs) => Rc::as_ptr(lhs) == Rc::as_ptr(rhs),

                _ => false,
            }

            Value::NativeValue(lhs) => match rhs {
                Value::NativeValue(rhs) => lhs == rhs,

                _ => false,
            }

            Value::Null => matches!(rhs, Value::Null),
        }
    }

    pub fn not_equal(&self, rhs: &Self) -> bool {
        !self.is_equal(rhs)
    }

    pub fn greater(&self, rhs: &Self) -> bool {
        match self {
            Value::Int(lhs) => match rhs {
                Value::Int(rhs) => *lhs > *rhs,
                Value::Float(rhs) => *lhs as FloatType > *rhs,

                _ => false,
            },
            Value::Float(lhs) => match rhs {
                Value::Int(rhs) => *lhs > *rhs as FloatType,
                Value::Float(rhs) => *lhs > *rhs,

                _ => false,
            },

            _ => false,
        }
    }

    pub fn lesser(&self, rhs: &Self) -> bool {
        match self {
            Value::Int(lhs) => match rhs {
                Value::Int(rhs) => *lhs < *rhs,
                Value::Float(rhs) => (*lhs as FloatType) < *rhs,

                _ => false,
            },
            Value::Float(lhs) => match rhs {
                Value::Int(rhs) => *lhs < *rhs as FloatType,
                Value::Float(rhs) => *lhs < *rhs,

                _ => false,
            },

            _ => false,
        }
    }

    pub fn greater_equal(&self, rhs: &Self) -> bool {
        self.greater(rhs) || self.is_equal(rhs)
    }

    pub fn lesser_equal(&self, rhs: &Self) -> bool {
        self.lesser(rhs) || self.is_equal(rhs)
    }

    pub fn try_negate(&self) -> Result<Value, RuntimeError> {
        match self {
            Value::Int(lhs) => Ok(Value::Int(-lhs)),
            Value::Float(lhs) => Ok(Value::Float(-lhs)),
            _ => Err(RuntimeError::InvalidUnaryOperation { op: "negate".to_owned(), rhs_type: self.type_name().to_owned() }),
        }
    }

    pub fn not(&self) -> Value {
        Value::Bool(!self.truthy())
    }

    pub fn truthy(&self) -> bool {
        match self {
            Value::Int(val) => *val != 0,
            Value::Float(val) => *val != 0.0,
            Value::Bool(val) => *val,
            Value::String(val) => !val.is_empty(),
            Value::Instance(_) => true,
            Value::Function(_) => true,
            Value::NativeFunction(_) => true,
            Value::Class(_) => true,
            Value::Module(_) => true,
            Value::NativeValue(_) => true,
            Value::Null => false,
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::Bool(_) => "bool",
            Value::String(_) => "string",
            Value::Instance(_) => "instance",
            Value::Function(_) => "function",
            Value::NativeFunction(_) => "native_function",
            Value::Class(_) => "class",
            Value::Module(_) => "module",
            Value::NativeValue(_) => "native_value",
            Value::Null => "null",
        }
    }

    pub fn take_string(self) -> Result<StringType, RuntimeError> {
        TryInto::<StringType>::try_into(self)
    }

    pub fn take_int(self) -> Result<IntType, RuntimeError> {
        TryInto::<IntType>::try_into(self)
    }

    pub fn take_float(self) -> Result<FloatType, RuntimeError> {
        TryInto::<FloatType>::try_into(self)
    }

    pub fn take_bool(self) -> Result<BoolType, RuntimeError> {
        TryInto::<BoolType>::try_into(self)
    }

    pub fn take_instance(self) -> Result<InstanceType, RuntimeError> {
        TryInto::<InstanceType>::try_into(self)
    }

    pub fn take_function(self) -> Result<FunctionType, RuntimeError> {
        TryInto::<FunctionType>::try_into(self)
    }

    pub fn take_native_function(self) -> Result<NativeFunctionType, RuntimeError> {
        TryInto::<NativeFunctionType>::try_into(self)
    }

    pub fn take_class(self) -> Result<ClassType, RuntimeError> {
        TryInto::<ClassType>::try_into(self)
    }

    pub fn take_module(self) -> Result<ModuleType, RuntimeError> {
        TryInto::<ModuleType>::try_into(self)
    }

    pub fn take_native_value(self) -> Result<NativeValueType, RuntimeError> {
        TryInto::<NativeValueType>::try_into(self)
    }
}
