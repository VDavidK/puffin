mod function;
mod instance;
mod native_function;
mod class;
mod module;
mod native_value;
mod node;
mod reactive;

use std::{cell::RefCell, fmt::Display, hash::Hash, rc::Rc};
pub use instance::{new_instance, Instance};
pub use function::Function;
pub use native_function::NativeFunction;
pub use class::{new_class, Class};
pub use module::{new_module, Module};
pub use native_value::{NativeValue, NativeValueTrait};
pub use node::{Node, LayoutNode, LayoutDirection, TextNode, ComponentNode};
pub use reactive::{Reactive};

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
pub type ListType = Rc<RefCell<Vec<Value>>>;
pub type NodeType = Rc<RefCell<Node>>;
pub type ReactiveType = Rc<RefCell<Reactive>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    Int(IntType),
    Float(FloatType),
    Bool(BoolType),
    String(StringType),
    Function(FunctionType),
    Class(ClassType),
    Module(ModuleType),
    List(ListType),
    Reactive(ReactiveType),
    Null,

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

impl TryFrom<Value> for ListType {
    type Error = RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::List(v) => Ok(v),
            _ => Err(RuntimeError::IncorrectType { ty: value.type_name().to_owned(), expected: "list".to_owned() }),
        }
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Value::List(Rc::new(RefCell::new(value)))
    }
}

impl TryFrom<Value> for ReactiveType {
    type Error = RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Reactive(v) => Ok(v),
            _ => Err(RuntimeError::IncorrectType { ty: value.type_name().to_owned(), expected: "reactive".to_owned() }),
        }
    }
}

impl From<Reactive> for Value {
    fn from(value: Reactive) -> Self {
        Value::Reactive(Rc::new(RefCell::new(value)))
    }
}

// impl<T> FromIterator<T> for Value where T: Into<Value> {
//     fn from_iter<I: IntoIterator<Item = T>>(mut iter: I) -> Self {
//         iter.into_iter()
//             .map(Into::into)
//     }
// }

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

impl From<FunctionType> for Value {
    fn from(value: FunctionType) -> Self {
        Value::Function(value)
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

impl From<NodeType> for Value {
    fn from(value: NodeType) -> Self {
        Value::Node(value)
    }
}

impl From<Node> for Value {
    fn from(value: Node) -> Self {
        Rc::new(RefCell::new(value)).into()
    }
}

impl TryFrom<Value> for NodeType {
    type Error = RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Node(v) => Ok(v),
            _ => Err(RuntimeError::IncorrectType { ty: value.type_name().to_owned(), expected: "node".to_owned() }),
        }
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
            Value::List(v) => f.write_fmt(format_args!("{}", ListDisplay(v))),
            Value::Node(v) => f.write_fmt(format_args!("{}", v.borrow())),
            Value::Reactive(v) => f.write_fmt(format_args!("{}", v.borrow())),
            Value::Null => f.write_fmt(format_args!("null")),
        }
    }
}

struct ListDisplay<'a>(&'a ListType);

impl<'a> Display for ListDisplay<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = self.0.borrow().iter()
            .map(Value::to_string)
            .collect::<Vec<String>>()
            .join(", ");

        f.write_fmt(format_args!("[{}]", inner))
    }
}

impl Value {
    pub fn try_add(&self, rhs: &Self) -> Result<Value, RuntimeError> {
        if let Value::Reactive(rhs) = rhs {
            return self.try_add(rhs.borrow().get());
        }

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

            Value::Reactive(inner) => inner.borrow().get().try_add(rhs),
            // TODO: Derived

            Value::String(lhs) => Ok(Value::String(format!("{lhs}{}", rhs.to_owned()))),

            _ => Err(RuntimeError::InvalidBinaryOperation { op: "add".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
        }
    }

    pub fn try_sub(&self, rhs: &Self) -> Result<Value, RuntimeError> {
        if let Value::Reactive(rhs) = rhs {
            return self.try_sub(rhs.borrow().get());
        }

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

            Value::Reactive(inner) => inner.borrow().get().try_sub(rhs),

            _ => Err(RuntimeError::InvalidBinaryOperation { op: "subtract".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
        }
    }

    pub fn try_div(&self, rhs: &Self) -> Result<Value, RuntimeError> {
        if let Value::Reactive(rhs) = rhs {
            return self.try_div(rhs.borrow().get());
        }

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

            Value::Reactive(inner) => inner.borrow().get().try_div(rhs),

            _ => Err(RuntimeError::InvalidBinaryOperation { op: "divide".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
        }
    }

    pub fn try_mul(&self, rhs: &Self) -> Result<Value, RuntimeError> {
        if let Value::Reactive(rhs) = rhs {
            return self.try_mul(rhs.borrow().get());
        }

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

            Value::Reactive(inner) => inner.borrow().get().try_mul(rhs),

            _ => Err(RuntimeError::InvalidBinaryOperation { op: "multiply".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
        }
    }

    pub fn try_mod(&self, rhs: &Self) -> Result<Value, RuntimeError> {
        if let Value::Reactive(rhs) = rhs {
            return self.try_mod(rhs.borrow().get());
        }

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

            Value::Reactive(inner) => inner.borrow().get().try_mod(rhs),

            _ => Err(RuntimeError::InvalidBinaryOperation { op: "modulo".to_owned(), lhs_type: self.type_name().to_owned(), rhs_type: rhs.type_name().to_owned() }),
        }
    }

    pub fn is_equal(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Value::Int(lhs), Value::Int(rhs)) => *lhs == *rhs,
            (Value::Int(lhs), Value::Float(rhs)) => *lhs as FloatType == *rhs,
            (Value::Float(lhs), Value::Int(rhs)) => *lhs == *rhs as FloatType,
            (Value::Float(lhs), Value::Float(rhs)) => *lhs == *rhs,

            (Value::Bool(lhs), Value::Bool(rhs)) => *lhs == *rhs,
            (Value::String(lhs), Value::String(rhs)) => *lhs == *rhs,
            (Value::NativeValue(lhs), Value::NativeValue(rhs)) => *lhs == *rhs,

            (Value::Instance(lhs), Value::Instance(rhs)) => lhs.as_ptr() == rhs.as_ptr(),
            (Value::List(lhs), Value::List(rhs)) => lhs.as_ptr() == rhs.as_ptr(),

            (Value::Function(lhs), Value::Function(rhs)) => Rc::as_ptr(lhs) == Rc::as_ptr(rhs),
            (Value::NativeFunction(lhs), Value::NativeFunction(rhs)) => Rc::as_ptr(lhs) == Rc::as_ptr(rhs),
            (Value::Class(lhs), Value::Class(rhs)) => Rc::as_ptr(lhs) == Rc::as_ptr(rhs),
            (Value::Module(lhs), Value::Module(rhs)) => Rc::as_ptr(lhs) == Rc::as_ptr(rhs),
            (Value::Node(lhs), Value::Node(rhs)) => Rc::as_ptr(lhs) == Rc::as_ptr(rhs),

            (lhs, Value::Reactive(inner)) => lhs.is_equal(inner.borrow().get()),
            (Value::Reactive(inner), rhs) => inner.borrow().get().is_equal(rhs),

            (Value::Null, Value::Null) => true,

            _ => false,
        }
    }

    pub fn not_equal(&self, rhs: &Self) -> bool {
        !self.is_equal(rhs)
    }

    pub fn greater(&self, rhs: &Self) -> bool {
        if let Value::Reactive(rhs) = rhs {
            return self.greater(rhs.borrow().get());
        }

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

            Value::Reactive(lhs) => lhs.borrow().get().greater(rhs),

            _ => false,
        }
    }

    pub fn lesser(&self, rhs: &Self) -> bool {
        if let Value::Reactive(rhs) = rhs {
            return self.lesser(rhs.borrow().get());
        }

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
            Value::Reactive(inner) => inner.borrow().get().try_negate(),
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
            Value::List(_) => true,
            Value::Node(_) => true,
            Value::Reactive(value) => value.borrow().get().truthy(),
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
            Value::List(_) => "list",
            Value::Node(_) => "node",
            Value::Null => "null",

            // TODO: Possibly show reactive outer. For example: reactive<inner>
            Value::Reactive(inner) => inner.borrow().get().type_name(),
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

    pub fn take_list(self) -> Result<ListType, RuntimeError> {
        TryInto::<ListType>::try_into(self)
    }

    pub fn take_node(self) -> Result<NodeType, RuntimeError> {
        TryInto::<NodeType>::try_into(self)
    }

    pub fn take_reactive(self) -> Result<ReactiveType, RuntimeError> {
        TryInto::<ReactiveType>::try_into(self)
    }
}
