// pub struct Molecule {
//     pub atoms: StableUnGraph<Molecule, ()>,
//     pub head: NodeIndex,
// }

use super::{element::Element, functional_groups::FunctionalGroup, traits::Weighable};

#[derive(Debug, Clone)]
pub enum Molecule {
    E(Element),         // a singular element
    F(FunctionalGroup), // a singular functional group
}

impl Weighable for Molecule {
    fn atomic_numbers(&self) -> i64 {
        match self {
            Molecule::E(e) => e.atomic_numbers(),
            Molecule::F(fg) => fg.atomic_numbers(),
        }
    }
}
