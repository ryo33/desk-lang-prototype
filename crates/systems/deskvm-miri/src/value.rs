use std::collections::HashMap;

use conc_types::{ConcEffect, ConcType};
use mir::ControlFlowGraphId;

use crate::eval_mir::Handler;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Value {
    String(String),
    Int(i64),
    Float(f64),
    Rational(i64, i64),
    Tuple(Vec<Value>),
    Variant { id: usize, value: Box<Value> },
    FnRef(FnRef),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FnRef {
    Link(ConcType),
    Closure(Closure),
    Recursion,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Closure {
    pub mir: ControlFlowGraphId,
    pub captured: HashMap<ConcType, Value>,
    pub handlers: HashMap<ConcEffect, Handler>,
}