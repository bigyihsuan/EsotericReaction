use super::value::Value;

pub trait Valuable {
    fn value(&self) -> Value;
}
pub trait Weighable {
    fn atomic_numbers(&self) -> i64;
}
