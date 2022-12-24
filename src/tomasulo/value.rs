use std::rc::Rc;

use super::*;

pub type Value = Rc<ValueInner>;

#[derive(Debug, Clone, PartialEq)]
pub enum ValueInner {
    /// An immediate value.
    Imm(i64),
    /// A float value.
    Float(f64),
    /// A unit.
    Unit(Unit),
    /// A memory address.
    MemAddr(Value),
    /// A operation.
    Op(Type, Value, Value),
}

pub fn new(inner: ValueInner) -> Value {
    Rc::new(inner)
}

pub fn apply_op(t: Type, v1: Value, v2: Value) -> Value {
    new(ValueInner::Op(t, v1, v2))
}

impl From<i64> for ValueInner {
    fn from(v: i64) -> ValueInner {
        ValueInner::Imm(v)
    }
}

impl From<Unit> for ValueInner {
    fn from(u: Unit) -> ValueInner {
        ValueInner::Unit(u)
    }
}

impl From<Value> for ValueInner {
    fn from(v: Value) -> ValueInner {
        ValueInner::MemAddr(v)
    }
}

impl From<(Type, Value, Value)> for ValueInner {
    fn from(t: (Type, Value, Value)) -> ValueInner {
        ValueInner::Op(t.0, t.1, t.2)
    }
}

impl std::fmt::Display for ValueInner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ValueInner::Imm(v) => write!(f, "{v}"),
            ValueInner::Float(v) => write!(f, "{v:2.2}"),
            ValueInner::Unit(u) => write!(f, "{u}"),
            ValueInner::MemAddr(v) => write!(f, "M[{v}]"),
            ValueInner::Op(t, v1, v2) => write!(f, "({v1}{}{v2})", t.op_str()),
        }
    }
}

impl ValueInner {
    pub fn brief(&self) -> String {
        match self {
            ValueInner::Imm(v) => format!("{v}"),
            ValueInner::Float(v) => format!("{v:2.2}"),
            ValueInner::Unit(u) => format!("{u}"),
            ValueInner::MemAddr(_) => "M[..]".to_string(),
            ValueInner::Op(t, _, _) => format!("..{}..", t.op_str()),
        }
    }
}
