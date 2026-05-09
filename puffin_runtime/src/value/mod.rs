mod function;
mod instance;
mod native_function;
mod class;
mod module;
mod native_value;
mod node;
mod reactive;
mod ops;
mod int;
mod float;
mod bool;
mod string;
mod list;
mod dictionary;
mod derived;

use std::{fmt::Display, hash::Hash, rc::Rc};
pub use instance::{new_instance, Instance, InstanceType};
pub use function::{Function, FunctionType};
pub use native_function::{NativeFunction, NativeFunctionType};
pub use class::{new_class, Class, ClassType};
pub use module::{new_module, Module, ModuleType};
pub use native_value::{NativeValue, NativeValueTrait, NativeValueType};
pub use node::{Node, LayoutNode, LayoutDirection, TextNode, ComponentNode, NodeType, FrameNode, ConditionalNode, BlockNode};
pub use reactive::{Reactive, ReactiveType};
pub use list::{ListDisplay, ListType};
pub use float::FloatType;
pub use int::IntType;
pub use bool::BoolType;
pub use string::StringType;
pub use dictionary::{DictionaryDisplay, DictionaryType};

use serde_derive::{Deserialize, Serialize};
use crate::RuntimeError;
use crate::value::derived::{derive_binary, derive_unary, DerivedType};
use crate::value::ops::{ValueAdd, ValueSub, ValueMul, ValueDiv, ValueMod, ValueDef, ValueTruthy, ValueNeg};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum Value {
    #[default]
    Null,

    Int(IntType),
    Float(FloatType),
    Bool(BoolType),
    String(StringType),
    Function(FunctionType),
    Class(ClassType),
    Module(ModuleType),
    List(ListType),
    Dictionary(DictionaryType),

    #[serde(skip)]
    Reactive(ReactiveType),

    #[serde(skip)]
    Derived(DerivedType),

    #[serde(skip)]
    Instance(InstanceType),

    #[serde(skip)]
    NativeValue(NativeValueType),

    #[serde(skip)]
    NativeFunction(NativeFunctionType),

    Node(NodeType),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Instance(l0), Self::Instance(r0)) => l0.as_ptr() == r0.as_ptr(),
            (Self::List(l0), Self::List(r0)) => l0.as_ptr() == r0.as_ptr(),
            (Self::Function(l0), Self::Function(r0)) => Rc::as_ptr(l0) == Rc::as_ptr(r0),
            (Self::NativeFunction(l0), Self::NativeFunction(r0)) => Rc::as_ptr(l0) == Rc::as_ptr(r0),
            (Self::Class(l0), Self::Class(r0)) => Rc::as_ptr(l0) == Rc::as_ptr(r0),
            (Self::Module(l0), Self::Module(r0)) => Rc::as_ptr(l0) == Rc::as_ptr(r0),
            (Self::Node(l0), Self::Node(r0)) => Rc::as_ptr(l0) == Rc::as_ptr(r0),
            (Self::Reactive(l0), Self::Reactive(r0)) => Rc::as_ptr(l0) == Rc::as_ptr(r0),
            (Self::Derived(l0), Self::Derived(r0)) => l0 == r0,
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

// impl<T> FromIterator<T> for Value where T: Into<Value> {
//     fn from_iter<I: IntoIterator<Item = T>>(mut iter: I) -> Self {
//         iter.into_iter()
//             .map(Into::into)
//     }
// }

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(v) => f.write_fmt(format_args!("{v}")),
            Value::Float(v) => f.write_fmt(format_args!("{v}")),
            Value::Bool(v) => f.write_fmt(format_args!("{v}")),
            Value::String(v) => f.write_fmt(format_args!("{}", v.borrow())),
            Value::Instance(v) => f.write_fmt(format_args!("{}", v.borrow())),
            Value::Function(v) => f.write_fmt(format_args!("{}", v.borrow())),
            Value::NativeFunction(v) => f.write_fmt(format_args!("{}", v.borrow())),
            Value::Class(v) => f.write_fmt(format_args!("{}", v.borrow())),
            Value::Module(v) => f.write_fmt(format_args!("{}", v.borrow())),
            Value::NativeValue(v) => f.write_fmt(format_args!("{}", v)),
            Value::List(v) => f.write_fmt(format_args!("{}", ListDisplay(v))),
            Value::Dictionary(v) => f.write_fmt(format_args!("{}", DictionaryDisplay(v))),
            Value::Node(v) => f.write_fmt(format_args!("{}", v.borrow())),
            Value::Reactive(v) => f.write_fmt(format_args!("{}", v.borrow())),
            Value::Derived(v) => f.write_fmt(format_args!("{}", v.eval_or_null())),
            Value::Null => f.write_fmt(format_args!("null")),
        }
    }
}

impl Value {
    pub fn is_reactive(&self) -> bool {
        matches!(self, Value::Reactive(_)) || matches!(self, Value::Derived(_))
    }

    fn references(&self, value: &Value) -> bool {
        match self {
            Value::Reactive(inner) => inner
                .borrow()
                .get() == value,
            Value::Derived(inner) => inner
                .references(value),

            _ => self == value,
        }
    }

    pub fn eval(&self) -> Result<Value, RuntimeError> {
        match self {
            Value::Reactive(inner) => Ok(inner.borrow().get().eval()?),
            Value::Derived(inner) => inner.eval(),
            val => Ok(val.clone()),
        }
    }

    pub fn try_add_derivable(&self, rhs: &Self) -> Result<Value, RuntimeError> {
        if self.is_reactive() || rhs.is_reactive() {
            derive_binary(self, rhs, |lhs, rhs| lhs.try_add(rhs))
        } else {
            self.try_add(rhs)
        }
    }

    pub fn try_add(&self, rhs: &Self) -> Result<Value, RuntimeError> {
        let lhs = self.eval()?;
        let rhs = rhs.eval()?;

        match lhs {
            Value::Int(lhs) => lhs.try_add(&rhs),
            Value::Float(lhs) => lhs.try_add(&rhs),

            Value::String(lhs) => lhs.try_add(&rhs),
            _ => Err(RuntimeError::InvalidBinaryOperation { op: "add".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
        }
    }

    pub fn try_sub_derivable(&self, rhs: &Self) -> Result<Value, RuntimeError> {
        if self.is_reactive() || rhs.is_reactive() {
            derive_binary(self, rhs, |lhs, rhs| lhs.try_sub(rhs))
        } else {
            self.try_sub(rhs)
        }
    }

    pub fn try_sub(&self, rhs: &Self) -> Result<Value, RuntimeError> {
        let lhs = self.eval()?;
        let rhs = rhs.eval()?;

        match lhs {
            Value::Int(lhs) => lhs.try_sub(&rhs),
            Value::Float(lhs) => lhs.try_sub(&rhs),
            Value::Reactive(inner) => inner.borrow().get().try_sub(&rhs),

            _ => Err(RuntimeError::InvalidBinaryOperation { op: "subtract".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
        }
    }

    pub fn try_div_derivable(&self, rhs: &Self) -> Result<Value, RuntimeError> {
        if self.is_reactive() || rhs.is_reactive() {
            derive_binary(self, rhs, |lhs, rhs| lhs.try_div(rhs))
        } else {
            self.try_div(rhs)
        }
    }

    pub fn try_div(&self, rhs: &Self) -> Result<Value, RuntimeError> {
        let lhs = self.eval()?;
        let rhs = rhs.eval()?;

        match lhs {
            Value::Int(lhs) => lhs.try_div(&rhs),
            Value::Float(lhs) => lhs.try_div(&rhs),
            Value::Reactive(inner) => inner.borrow().get().try_div(&rhs),

            _ => Err(RuntimeError::InvalidBinaryOperation { op: "divide".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
        }
    }

    pub fn try_mul_derivable(&self, rhs: &Self) -> Result<Value, RuntimeError> {
        if self.is_reactive() || rhs.is_reactive() {
            derive_binary(self, rhs, |lhs, rhs| lhs.try_mul(rhs))
        } else {
            self.try_mul(rhs)
        }
    }

    pub fn try_mul(&self, rhs: &Self) -> Result<Value, RuntimeError> {
        let lhs = self.eval()?;
        let rhs = rhs.eval()?;

        match lhs {
            Value::Int(lhs) => lhs.try_mul(&rhs),
            Value::Float(lhs) => lhs.try_mul(&rhs),
            Value::Reactive(inner) => inner.borrow().get().try_mul(&rhs),

            _ => Err(RuntimeError::InvalidBinaryOperation { op: "multiply".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
        }
    }

    pub fn try_mod_derivable(&self, rhs: &Self) -> Result<Value, RuntimeError> {
        if self.is_reactive() || rhs.is_reactive() {
            derive_binary(self, rhs, |lhs, rhs| lhs.try_mod(rhs))
        } else {
            self.try_mod(rhs)
        }
    }

    pub fn try_mod(&self, rhs: &Self) -> Result<Value, RuntimeError> {
        let lhs = self.eval()?;
        let rhs = rhs.eval()?;

        match lhs {
            Value::Int(lhs) => lhs.try_mod(&rhs),
            Value::Float(lhs) => lhs.try_mod(&rhs),
            Value::Reactive(inner) => inner.borrow().get().try_mod(&rhs),

            _ => Err(RuntimeError::InvalidBinaryOperation { op: "modulo".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
        }
    }

    pub fn is_equal_derivable(&self, rhs: &Self) -> Result<Value, RuntimeError> {
        if self.is_reactive() || rhs.is_reactive() {
            derive_binary(self, rhs, |lhs, rhs| Ok(Value::Bool(lhs.is_equal(rhs)?)))
        } else {
            Ok(Value::Bool(self.is_equal(rhs)?))
        }
    }

    pub fn is_equal(&self, rhs: &Self) -> Result<bool, RuntimeError> {
        let lhs = self.eval()?;
        let rhs = rhs.eval()?;

        Ok(match (lhs, rhs) {
            (Value::Int(lhs), Value::Int(rhs)) => lhs == rhs,
            (Value::Int(lhs), Value::Float(rhs)) => lhs as FloatType == rhs,
            (Value::Float(lhs), Value::Int(rhs)) => lhs == rhs as FloatType,
            (Value::Float(lhs), Value::Float(rhs)) => lhs == rhs,

            (Value::Bool(lhs), Value::Bool(rhs)) => lhs == rhs,
            (Value::String(lhs), Value::String(rhs)) => lhs == rhs,
            (Value::NativeValue(lhs), Value::NativeValue(rhs)) => lhs == rhs,

            (Value::Instance(lhs), Value::Instance(rhs)) => lhs.as_ptr() == rhs.as_ptr(),
            (Value::List(lhs), Value::List(rhs)) => lhs.as_ptr() == rhs.as_ptr(),

            (Value::Function(lhs), Value::Function(rhs)) => lhs.as_ptr() == rhs.as_ptr(),
            (Value::NativeFunction(lhs), Value::NativeFunction(rhs)) => lhs.as_ptr() == rhs.as_ptr(),
            (Value::Class(lhs), Value::Class(rhs)) => lhs.as_ptr() == rhs.as_ptr(),
            (Value::Module(lhs), Value::Module(rhs)) => lhs.as_ptr() == rhs.as_ptr(),
            (Value::Node(lhs), Value::Node(rhs)) => lhs.as_ptr() == rhs.as_ptr(),

            (Value::Null, Value::Null) => true,

            _ => false,
        })
    }

    pub fn not_equal_derivable(&self, rhs: &Self) -> Result<Value, RuntimeError> {
        if self.is_reactive() || rhs.is_reactive() {
            derive_binary(self, rhs, |lhs, rhs| Ok(Value::Bool(lhs.not_equal(rhs)?)))
        } else {
            Ok(Value::Bool(self.not_equal(rhs)?))
        }
    }

    pub fn not_equal(&self, rhs: &Self) -> Result<bool, RuntimeError> {
        Ok(!self.is_equal(rhs)?)
    }

    pub fn greater_derivable(&self, rhs: &Self) -> Result<Value, RuntimeError> {
        if self.is_reactive() || rhs.is_reactive() {
            derive_binary(self, rhs, |lhs, rhs| Ok(Value::Bool(lhs.greater(rhs)?)))
        } else {
            Ok(Value::Bool(self.greater(rhs)?))
        }
    }

    pub fn greater(&self, rhs: &Self) -> Result<bool, RuntimeError> {
        let lhs = self.eval()?;
        let rhs = rhs.eval()?;

        Ok(match lhs {
            Value::Int(lhs) => match rhs {
                Value::Int(rhs) => lhs > rhs,
                Value::Float(rhs) => lhs as FloatType > rhs,

                _ => false,
            },
            Value::Float(lhs) => match rhs {
                Value::Int(rhs) => lhs > rhs as FloatType,
                Value::Float(rhs) => lhs > rhs,

                _ => false,
            },

            _ => false,
        })
    }

    pub fn lesser_derivable(&self, rhs: &Self) -> Result<Value, RuntimeError> {
        if self.is_reactive() || rhs.is_reactive() {
            derive_binary(self, rhs, |lhs, rhs| Ok(Value::Bool(lhs.lesser(rhs)?)))
        } else {
            Ok(Value::Bool(self.lesser(rhs)?))
        }
    }

    pub fn lesser(&self, rhs: &Self) -> Result<bool, RuntimeError> {
        let lhs = self.eval()?;
        let rhs = rhs.eval()?;

        Ok(match lhs {
            Value::Int(lhs) => match rhs {
                Value::Int(rhs) => lhs < rhs,
                Value::Float(rhs) => (lhs as FloatType) < rhs,

                _ => false,
            },
            Value::Float(lhs) => match rhs {
                Value::Int(rhs) => lhs < rhs as FloatType,
                Value::Float(rhs) => lhs < rhs,

                _ => false,
            },

            _ => false,
        })
    }

    pub fn greater_equal_derivable(&self, rhs: &Self) -> Result<Value, RuntimeError> {
        if self.is_reactive() || rhs.is_reactive() {
            derive_binary(self, rhs, |lhs, rhs| Ok(Value::Bool(lhs.greater_equal(rhs)?)))
        } else {
            Ok(Value::Bool(self.greater_equal(rhs)?))
        }
    }

    pub fn greater_equal(&self, rhs: &Self) -> Result<bool, RuntimeError> {
        Ok(self.greater(rhs)? || self.is_equal(rhs)?)
    }

    pub fn lesser_equal_derivable(&self, rhs: &Self) -> Result<Value, RuntimeError> {
        if self.is_reactive() || rhs.is_reactive() {
            derive_binary(self, rhs, |lhs, rhs| Ok(Value::Bool(lhs.lesser_equal(rhs)?)))
        } else {
            Ok(Value::Bool(self.lesser_equal(rhs)?))
        }
    }

    pub fn lesser_equal(&self, rhs: &Self) -> Result<bool, RuntimeError> {
        Ok(self.lesser(rhs)? || self.is_equal(rhs)?)
    }

    pub fn try_negate_derivable(&self) -> Result<Value, RuntimeError> {
        if self.is_reactive() {
            derive_unary(self, |value| value.try_negate())
        } else {
            self.try_negate()
        }
    }

    pub fn try_negate(&self) -> Result<Value, RuntimeError> {
        let value = self.eval()?;

        match value {
            Value::Int(lhs) => lhs.try_neg(),
            Value::Float(lhs) => lhs.try_neg(),
            _ => Err(RuntimeError::InvalidUnaryOperation { op: "negate".to_owned(), rhs_type: self.type_name().to_owned() }),
        }
    }

    pub fn not_derivable(&self) -> Result<Value, RuntimeError> {
        if self.is_reactive() {
            derive_unary(self, |value| value.not())
        } else {
            self.not()
        }
    }

    pub fn not(&self) -> Result<Value, RuntimeError> {
        Ok(Value::Bool(!self.truthy()?))
    }

    pub fn truthy_derivable(&self) -> Result<Value, RuntimeError> {
        if self.is_reactive() {
            derive_unary(self, |value| Ok(Value::Bool(value.truthy()?)))
        } else {
            Ok(Value::Bool(self.truthy()?))
        }
    }

    pub fn truthy(&self) -> Result<bool, RuntimeError> {
        let value = self.eval()?;

        Ok(match value {
            Value::Int(v) => v.truthy(),
            Value::Float(v) => v.truthy(),
            Value::Bool(v) => v.truthy(),
            Value::String(v) => v.truthy(),
            Value::Instance(v) => v.truthy(),
            Value::Function(v) => v.truthy(),
            Value::NativeFunction(v) => v.truthy(),
            Value::Class(v) => v.truthy(),
            Value::Module(v) => v.truthy(),
            Value::NativeValue(v) => v.truthy(),
            Value::List(v) => v.truthy(),
            Value::Dictionary(v) => v.truthy(),
            Value::Node(v) => v.truthy(),
            Value::Null => false,

            Value::Derived(_) |
            Value::Reactive(_) => unreachable!("should not be covered as the eval() function should never return a reactive or derived value directly"),
        })
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Int(_) => IntType::type_name(),
            Value::Float(_) => FloatType::type_name(),
            Value::Bool(_) => BoolType::type_name(),
            Value::String(_) => StringType::type_name(),
            Value::Instance(_) => InstanceType::type_name(),
            Value::Function(_) => FunctionType::type_name(),
            Value::NativeFunction(_) => NativeFunctionType::type_name(),
            Value::Class(_) => ClassType::type_name(),
            Value::Module(_) => ModuleType::type_name(),
            Value::NativeValue(_) => NativeValueType::type_name(),
            Value::List(_) => ListType::type_name(),
            Value::Dictionary(_) => DictionaryType::type_name(),
            Value::Node(_) => NodeType::type_name(),
            Value::Null => "null",

            // TODO: Possibly show reactive outer. For example: reactive<inner>
            // Value::Reactive(inner) => inner.borrow().get().type_name(),
            // Value::Derived(inner) => inner.eval_or_null().type_name(),
            Value::Reactive(_) => "reactive",
            Value::Derived(_) => "derived",
        }
    }

    pub fn take_string(self) -> Result<StringType, RuntimeError> {
        TryInto::<StringType>::try_into(self.eval()?)
    }
    
    pub fn take_int(self) -> Result<IntType, RuntimeError> {
        TryInto::<IntType>::try_into(self.eval()?)
    }

    pub fn take_float(self) -> Result<FloatType, RuntimeError> {
        TryInto::<FloatType>::try_into(self.eval()?)
    }

    pub fn take_bool(self) -> Result<BoolType, RuntimeError> {
        TryInto::<BoolType>::try_into(self.eval()?)
    }

    pub fn take_instance(self) -> Result<InstanceType, RuntimeError> {
        TryInto::<InstanceType>::try_into(self.eval()?)
    }

    pub fn take_function(self) -> Result<FunctionType, RuntimeError> {
        TryInto::<FunctionType>::try_into(self.eval()?)
    }

    pub fn take_native_function(self) -> Result<NativeFunctionType, RuntimeError> {
        TryInto::<NativeFunctionType>::try_into(self.eval()?)
    }

    pub fn take_class(self) -> Result<ClassType, RuntimeError> {
        TryInto::<ClassType>::try_into(self.eval()?)
    }

    pub fn take_module(self) -> Result<ModuleType, RuntimeError> {
        TryInto::<ModuleType>::try_into(self.eval()?)
    }

    pub fn take_native_value(self) -> Result<NativeValueType, RuntimeError> {
        TryInto::<NativeValueType>::try_into(self.eval()?)
    }

    pub fn take_list(self) -> Result<ListType, RuntimeError> {
        TryInto::<ListType>::try_into(self.eval()?)
    }

    pub fn take_dictionary(self) -> Result<DictionaryType, RuntimeError> {
        TryInto::<DictionaryType>::try_into(self.eval()?)
    }

    pub fn take_node(self) -> Result<NodeType, RuntimeError> {
        TryInto::<NodeType>::try_into(self.eval()?)
    }

    pub fn take_reactive(self) -> Result<ReactiveType, RuntimeError> {
        TryInto::<ReactiveType>::try_into(self.eval()?)
    }
}
