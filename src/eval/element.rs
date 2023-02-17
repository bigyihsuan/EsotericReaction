use super::value::Weighable;

#[derive(Debug, Clone)]
pub enum Element {
    Header(ElementHeader),
    H,
    C,
    O,
    B,
    N,
    S,
}

impl Weighable for Element {
    fn atomic_weight(&self) -> i64 {
        match self {
            Element::Header(_) => 0,
            Element::H => 1,
            Element::B => 5,
            Element::C => 6,
            Element::N => 7,
            Element::O => 8,
            Element::S => 16,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ElementHeader {
    AlkaneHeader,
}
