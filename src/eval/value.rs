use std::{collections::HashMap, fmt::Display, hash::Hash};

#[derive(Debug, Clone)]
pub enum Value {
    Number(i64),                  // ether
    Boolean(bool),                // borinic acid
    String(Vec<char>),            // sulfide
    Pair(Box<Value>, Box<Value>), // amine
    List(Vec<Value>),             // alkane
    Map(HashMap<Value, Value>),   // alkane
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Pair(l0, l1), Self::Pair(r0, r1)) => **l0 == **r0 && **l1 == **r1,
            (Self::List(l0), Self::List(r0)) => {
                l0.len() == r0.len() && l0.iter().zip(r0.iter()).all(|t| t.0 == t.1)
            }
            (Self::Map(l0), Self::Map(r0)) => {
                l0.len() == r0.len()
                    && l0.keys().map(|k| k.clone()).all(|k| {
                        r0.keys()
                            .map(|c| c.clone())
                            .find(|e| k.clone() == e.clone())
                            .is_some()
                    })
            }
            _ => false,
        }
    }
}

impl Eq for Value {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => f.write_fmt(format_args!("{}", n)),
            Value::Boolean(b) => f.write_fmt(format_args!("{}", b)),
            Value::String(s) => {
                f.write_fmt(format_args!("\"{}\"", s.into_iter().collect::<String>()))
            }
            Value::Pair(l, r) => f.write_fmt(format_args!("({},{})", l, r)),
            Value::List(v) => f.write_fmt(format_args!(
                "[{}]",
                v.iter()
                    .map(|e| format!("{}", e))
                    .collect::<Vec<String>>()
                    .join(",")
            )),
            Value::Map(m) => f.write_fmt(format_args!(
                "{{{}}}",
                m.iter()
                    .map(|t| format!("{}:{}", t.0, t.1))
                    .collect::<Vec<String>>()
                    .join(",")
            )),
        }
    }
}

pub trait Valuable {
    fn value(&self) -> Value;
}

pub trait Weighable {
    fn atomic_numbers(&self) -> i64;
}
