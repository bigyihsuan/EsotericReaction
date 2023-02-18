// pub struct Molecule {
//     pub atoms: StableUnGraph<Molecule, ()>,
//     pub head: NodeIndex,
// }

use super::{
    element::Element, functional_groups::FunctionalGroup, value::Weighable,
};

#[derive(Debug, Clone)]
pub enum Molecule {
    E(Element),          // a singular element
    Fg(FunctionalGroup), // a singular functional group
}

impl Weighable for Molecule {
    fn atomic_weight(&self) -> i64 {
        match self {
            Molecule::E(e) => e.atomic_weight(),
            Molecule::Fg(fg) => fg.atomic_weight(),
        }
    }
}
