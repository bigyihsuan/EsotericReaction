pub mod amine;
pub mod borinic;
pub mod ether;
pub mod sulfide;

use self::sulfide::Sulfide;
use self::{amine::Amine, borinic::BorinicAcid, ether::Ether};

use super::alkane::Alkane;
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
        let alkane = Alkane::new();
        Self::Alkane(alkane)
    }
    pub fn new_ether(r: FgElement) -> Self {
        let mut atoms = Atoms::new();
        let o = atoms.add_node(Molecule::E(Element::O));
        let r = atoms.add_node(r.as_molecule());
        atoms.add_edge(o, r);
        Self::Ether(Ether(atoms))
    }
    pub fn new_borinic_acid(r: FgElement) -> Self {
        let mut atoms = Atoms::new();
        let b = atoms.add_node(Molecule::E(Element::B));
        let o = atoms.add_node(Molecule::E(Element::O));
        let h = atoms.add_node(Molecule::E(Element::H));
        let r = atoms.add_node(r.as_molecule());
        atoms.add_edge(b, o);
        atoms.add_edge(o, h);
        atoms.add_edge(b, r);
        Self::BorinicAcid(BorinicAcid(atoms))
    }
    pub fn new_sulfide(r: FgElement) -> Self {
        Self::Sulfide(Sulfide::new_with(r))
    }
    pub fn new_amine(r1: FgElement, r2: FgElement) -> Self {
        let mut atoms = Atoms::new();
        let n = atoms.add_node(Molecule::E(Element::N));
        let r1 = atoms.add_node(r1.as_molecule());
        let r2 = atoms.add_node(r2.as_molecule());
        atoms.add_edge(n, r1);
        atoms.add_edge(n, r2);
        Self::Amine(Amine(atoms))
    }

    fn unwrap(&self) -> &AtomGraph {
        match self {
            Self::Ether(a) => a.0.atoms(),
            Self::BorinicAcid(a) => a.0.atoms(),
            Self::Sulfide(a) => a.0.atoms(),
            Self::Amine(a) => a.0.atoms(),
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
