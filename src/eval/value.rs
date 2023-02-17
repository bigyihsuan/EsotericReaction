use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Number(i64),
    Boolean(bool),
    String(Vec<char>),
    Pair((Box<Value>, Box<Value>)),
    List(Vec<Box<Value>>),
    Map(HashMap<Box<Value>, Box<Value>>),
}

pub trait Valuable {
    fn value(&self) -> Value;
}

pub trait Weighable {
    fn atomic_weight(&self) -> i64;
}
