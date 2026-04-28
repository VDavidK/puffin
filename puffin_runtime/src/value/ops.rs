use crate::RuntimeError;
use crate::value::Value;

pub trait ValueDiv {
    fn try_div(&self, rhs: &Value) -> Result<Value, RuntimeError>;
}

pub trait ValueAdd {
    fn try_add(&self, rhs: &Value) -> Result<Value, RuntimeError>;
}

pub trait ValueMod {
    fn try_mod(&self, rhs: &Value) -> Result<Value, RuntimeError>;
}

pub trait ValueSub {
    fn try_sub(&self, rhs: &Value) -> Result<Value, RuntimeError>;
}

pub trait ValueMul {
    fn try_mul(&self, rhs: &Value) -> Result<Value, RuntimeError>;
}

pub trait ValueNeg {
    fn try_neg(&self) -> Result<Value, RuntimeError>;
}

pub trait ValueTruthy {
    fn truthy(&self) -> bool;
    fn not(&self) -> bool {
        !self.truthy()
    }
}

pub trait ValueDef {
    const TYPE_NAME: &'static str;
    fn type_name() -> &'static str {
        Self::TYPE_NAME
    }
}