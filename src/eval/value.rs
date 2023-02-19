use std::{collections::HashMap, fmt::Display, hash::Hash, ops::Add};

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

impl Eq for Value {}

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

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(l), Value::Number(r)) => Value::Number(l + r),
            (Value::Boolean(l), Value::Boolean(r)) => Value::Boolean(l && r),
            (Value::String(l), Value::String(r)) => {
                let mut v = l;
                r.iter().for_each(|e| v.push(*e));
                Value::String(v)
            }
            (Value::Pair(la, lb), Value::Pair(ra, rb)) => {
                let a = *la + *ra;
                let b = *lb + *rb;
                Value::Pair(Box::new(a), Box::new(b))
            }
            (Value::List(l), Value::List(r)) => {
                let mut v = l;
                r.iter().for_each(|e| v.push(e.clone()));
                Value::List(v)
            }
            (Value::Map(l), Value::Map(r)) => {
                let mut m = l;
                r.iter().for_each(|(k, v)| {
                    m.insert(k.clone(), v.clone());
                });
                Value::Map(m)
            }
            (l, r) => panic!("not supported for Add: {} and {}", l, r),
        }
    }
}

#[derive(Clone)]
enum ValueType {
    // Number
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    // Bool
    Bool(bool),
    // String
    String(String),
    Str(&'static str),
    VecChar(Vec<char>),
    // Pair
    Pair((Box<ValueType>, Box<ValueType>)),
    // List
    Vec(Vec<ValueType>),
    // Map
    HashMap(HashMap<ValueType, ValueType>),
}

macro_rules! impl_from_num_for_value {
    ($T:ty) => {
        impl From<$T> for Value {
            fn from(value: $T) -> Self {
                Value::Number(value as i64)
            }
        }
    };
}

impl_from_num_for_value!(u8);
impl_from_num_for_value!(u16);
impl_from_num_for_value!(u32);
impl_from_num_for_value!(u64);
impl_from_num_for_value!(u128);
impl_from_num_for_value!(i8);
impl_from_num_for_value!(i16);
impl_from_num_for_value!(i32);
impl_from_num_for_value!(i64);
impl_from_num_for_value!(i128);

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Boolean(value)
    }
}
impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value.chars().collect())
    }
}
impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.chars().collect())
    }
}
impl From<Vec<char>> for Value {
    fn from(value: Vec<char>) -> Self {
        Value::String(value)
    }
}
impl From<(Value, Value)> for Value {
    fn from(value: (Value, Value)) -> Self {
        let l = value.0;
        let r = value.1;
        Value::Pair(Box::new(l), Box::new(r))
    }
}
impl From<(ValueType, ValueType)> for Value {
    fn from(value: (ValueType, ValueType)) -> Self {
        let l = value.0;
        let r = value.1;
        Value::Pair(Box::new(Value::from(l)), Box::new(Value::from(r)))
    }
}
impl From<(Box<ValueType>, Box<ValueType>)> for Value {
    fn from(value: (Box<ValueType>, Box<ValueType>)) -> Self {
        let l = value.0;
        let r = value.1;
        Value::Pair(Box::new(Value::from(*l)), Box::new(Value::from(*r)))
    }
}
impl From<Vec<ValueType>> for Value {
    fn from(value: Vec<ValueType>) -> Self {
        Value::List(value.iter().map(|e| Value::from(e.clone())).collect())
    }
}

impl From<HashMap<ValueType, ValueType>> for Value {
    fn from(value: HashMap<ValueType, ValueType>) -> Self {
        Value::Map(
            value
                .iter()
                .map(|(k, v)| {
                    let k = Value::from(k.clone());
                    let v = Value::from(v.clone());
                    (k, v)
                })
                .collect(),
        )
    }
}

impl From<ValueType> for Value {
    fn from(value: ValueType) -> Self {
        match value {
            ValueType::U8(v) => Value::from(v),
            ValueType::U16(v) => Value::from(v),
            ValueType::U32(v) => Value::from(v),
            ValueType::U64(v) => Value::from(v),
            ValueType::U128(v) => Value::from(v),
            ValueType::I8(v) => Value::from(v),
            ValueType::I16(v) => Value::from(v),
            ValueType::I32(v) => Value::from(v),
            ValueType::I64(v) => Value::from(v),
            ValueType::I128(v) => Value::from(v),
            ValueType::Bool(v) => Value::from(v),
            ValueType::String(v) => Value::from(v),
            ValueType::Str(v) => Value::from(v),
            ValueType::VecChar(v) => Value::from(v),
            ValueType::Pair(v) => Value::from(v),
            ValueType::Vec(v) => Value::from(v),
            ValueType::HashMap(v) => Value::from(v),
        }
    }
}
