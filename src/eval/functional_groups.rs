pub mod alkane;
pub mod amine;
pub mod borinic;
pub mod ether;
pub mod fg_macros;
pub mod sulfide;

use self::alkane::Alkane;
use self::sulfide::Sulfide;
use self::{amine::Amine, borinic::BorinicAcid, ether::Ether};

use super::atom_like::AtomLike;
use super::atoms::{AtomGraph, Atoms};
use super::element::Element;
use super::molecule::Molecule;
use super::value::{Valuable, Value, Weighable};
use petgraph::stable_graph::{EdgeIndex, NodeIndex};

pub enum FgElement {
    E(Element),
    F(FunctionalGroup),
}

impl FgElement {
    pub fn as_molecule(&self) -> Molecule {
        match self {
            FgElement::E(e) => Molecule::E(e.to_owned()),
            FgElement::F(f) => Molecule::F(f.to_owned()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum FunctionalGroup {
    Alkane(Alkane),
    Ether(Ether),
    BorinicAcid(BorinicAcid),
    Sulfide(Sulfide),
    Amine(Amine),
}

impl FunctionalGroup {
    pub fn new_alkane() -> Self {
        Self::Alkane(Alkane::new())
    }
    pub fn new_ether(r: FgElement) -> Self {
        Self::Ether(Ether::new_with(r))
    }
    pub fn new_borinic_acid(r: FgElement) -> Self {
        Self::BorinicAcid(BorinicAcid::new_with(r))
    }
    pub fn new_sulfide(r: FgElement) -> Self {
        Self::Sulfide(Sulfide::new_with(r))
    }
    pub fn new_amine(r1: FgElement, r2: FgElement) -> Self {
        Self::Amine(Amine::new_with(r1, r2))
    }

    fn unwrap(&self) -> &AtomGraph {
        match self {
            Self::Ether(a) => a.atoms(),
            Self::BorinicAcid(a) => a.atoms(),
            Self::Sulfide(a) => a.atoms(),
            Self::Amine(a) => a.atoms(),
            Self::Alkane(a) => a.atoms(),
        }
    }
}

impl AtomLike for FunctionalGroup {
    fn get_atoms(&self) -> &Atoms {
        match self {
            Self::Alkane(m) => m.get_atoms(),
            Self::Ether(m) => m.get_atoms(),
            Self::BorinicAcid(m) => m.get_atoms(),
            Self::Sulfide(m) => m.get_atoms(),
            Self::Amine(m) => m.get_atoms(),
        }
    }

    fn get_atoms_mut(&mut self) -> &mut Atoms {
        match self {
            Self::Alkane(m) => m.get_atoms_mut(),
            Self::Ether(m) => m.get_atoms_mut(),
            Self::BorinicAcid(m) => m.get_atoms_mut(),
            Self::Sulfide(m) => m.get_atoms_mut(),
            Self::Amine(m) => m.get_atoms_mut(),
        }
    }

    fn flatten(&self) -> Atoms {
        match self {
            Self::Ether(a) => a.0.flatten(),
            Self::BorinicAcid(a) => a.flatten(),
            Self::Sulfide(a) => a.0.flatten(),
            Self::Amine(a) => a.flatten(),
            Self::Alkane(a) => a.flatten(),
        }
    }

    fn add_node(&mut self, _m: Molecule) -> NodeIndex {
        todo!()
    }

    fn add_edge(&mut self, _m: NodeIndex, _n: NodeIndex) -> EdgeIndex {
        todo!()
    }
}

impl Weighable for FunctionalGroup {
    fn atomic_numbers(&self) -> i64 {
        match self {
            Self::Alkane(alk) => alk.atomic_numbers(),
            Self::Ether(eth) => eth.atomic_numbers(),
            Self::BorinicAcid(bor) => bor.atomic_numbers(),
            Self::Sulfide(sulf) => sulf.atomic_numbers(),
            Self::Amine(am) => am.atomic_numbers(),
        }
    }
}

impl Valuable for FunctionalGroup {
    fn value(&self) -> Value {
        match self {
            Self::Alkane(_) => todo!(),
            Self::Ether(eth) => eth.value(),
            Self::BorinicAcid(bor) => bor.value(),
            Self::Sulfide(sul) => sul.value(),
            Self::Amine(ami) => ami.value(),
        }
    }
}
