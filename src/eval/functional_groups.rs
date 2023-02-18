pub mod borinic;
pub mod ether;
pub mod sulfide;

use self::sulfide::Sulfide;
use self::{borinic::BorinicAcid, ether::Ether};

use super::alkane::Alkane;
use super::atom_like::AtomLike;
use super::atoms::{AtomGraph, Atoms};
use super::element::Element;
use super::molecule::Molecule;
use super::value::{Valuable, Value, Weighable};

#[derive(Debug, Clone)]
pub enum FunctionalGroup {
    Alkane(Alkane),
    Ether(Ether),
    BorinicAcid(BorinicAcid),
    Sulfide(Sulfide),
    Amine(Atoms),
}
use petgraph::stable_graph::{EdgeIndex, NodeIndex};

impl FunctionalGroup {
    pub fn new_alkane() -> Self {
        let alkane = Alkane::new();
        Self::Alkane(alkane)
    }
    pub fn new_ether(r: Molecule) -> Self {
        let mut atoms = Atoms::new();
        let o = atoms.add_node(Molecule::E(Element::O));
        let r_ = atoms.add_node(r);
        atoms.add_edge(o, r_);
        Self::Ether(Ether(atoms))
    }
    pub fn new_borinic_acid(r: Molecule) -> Self {
        let mut atoms = Atoms::new();
        let b = atoms.add_node(Molecule::E(Element::B));
        let o = atoms.add_node(Molecule::E(Element::O));
        let h = atoms.add_node(Molecule::E(Element::H));
        let r_ = atoms.add_node(r);
        atoms.add_edge(b, o);
        atoms.add_edge(o, h);
        atoms.add_edge(b, r_);
        Self::BorinicAcid(BorinicAcid(atoms))
    }
    pub fn new_sulfide(r: Molecule) -> Self {
        let mut atoms = Atoms::new();
        let s = atoms.add_node(Molecule::E(Element::S));
        let r_ = atoms.add_node(r);
        atoms.add_edge(s, r_);
        Self::Sulfide(Sulfide(atoms))
    }
    pub fn new_amine() -> Self {
        let amine = Atoms::new();
        Self::Amine(amine)
    }

    fn unwrap(&self) -> &AtomGraph {
        match self {
            Self::Ether(a) => a.0.atoms(),
            Self::BorinicAcid(a) => a.0.atoms(),
            Self::Sulfide(a) => a.0.atoms(),
            Self::Amine(a) => a.atoms(),
            Self::Alkane(a) => a.chain.atoms(),
        }
    }
}

impl AtomLike for FunctionalGroup {
    fn get_atoms(&self) -> &Atoms {
        match self {
            FunctionalGroup::Alkane(m) => m.get_atoms(),
            FunctionalGroup::Ether(m) => m.get_atoms(),
            FunctionalGroup::BorinicAcid(m) => m.get_atoms(),
            FunctionalGroup::Sulfide(m) => m.get_atoms(),
            FunctionalGroup::Amine(m) => m.get_atoms(),
        }
    }

    fn get_atoms_mut(&mut self) -> &mut Atoms {
        match self {
            FunctionalGroup::Alkane(m) => m.get_atoms_mut(),
            FunctionalGroup::Ether(m) => m.get_atoms_mut(),
            FunctionalGroup::BorinicAcid(m) => m.get_atoms_mut(),
            FunctionalGroup::Sulfide(m) => m.get_atoms_mut(),
            FunctionalGroup::Amine(m) => m.get_atoms_mut(),
        }
    }

    fn flatten(&self) -> Atoms {
        match self {
            Self::Ether(a) => a.0.flatten(),
            Self::BorinicAcid(a) => a.0.flatten(),
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

impl Valuable for FunctionalGroup {
    fn value(&self) -> Value {
        match self {
            Self::Alkane(_) => todo!(),
            Self::Ether(eth) => eth.value(),
            Self::BorinicAcid(bor) => bor.value(),
            Self::Sulfide(sul) => sul.value(),
            Self::Amine(_) => todo!(),
        }
    }
}

impl Weighable for FunctionalGroup {
    fn atomic_weight(&self) -> i64 {
        match self {
            Self::Alkane(_alk) => todo!(),
            Self::Ether(eth) => eth.atomic_weight(),
            Self::BorinicAcid(bor) => bor.atomic_weight(),
            Self::Sulfide(_sulf) => todo!(),
            Self::Amine(_am) => todo!(),
        }
    }
}
