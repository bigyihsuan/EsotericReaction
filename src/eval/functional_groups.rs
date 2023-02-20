pub mod alkane;
pub mod amine;
pub mod borinic;
pub mod ether;
pub mod fg_macros;
pub mod sulfide;

use std::fmt::Display;
use std::ops::{Add, Sub};

use self::alkane::Alkane;
use self::sulfide::Sulfide;
use self::{amine::Amine, borinic::BorinicAcid, ether::Ether};

use super::atom_like::AtomLike;
use super::atoms::{AtomGraph, Atoms};
use super::element::Element;
use super::molecule::Molecule;
use super::traits::{Valuable, Weighable};
use super::value::Value;
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
    Ether(Ether),
    BorinicAcid(BorinicAcid),
    Sulfide(Sulfide),
    Amine(Amine),
    Alkane(Alkane),
}

impl FunctionalGroup {
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
    pub fn new_alkane() -> Self {
        Self::Alkane(Alkane::new())
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
            Self::Ether(m) => m.get_atoms(),
            Self::BorinicAcid(m) => m.get_atoms(),
            Self::Sulfide(m) => m.get_atoms(),
            Self::Amine(m) => m.get_atoms(),
            Self::Alkane(m) => m.get_atoms(),
        }
    }

    fn get_atoms_mut(&mut self) -> &mut Atoms {
        match self {
            Self::Ether(m) => m.get_atoms_mut(),
            Self::BorinicAcid(m) => m.get_atoms_mut(),
            Self::Sulfide(m) => m.get_atoms_mut(),
            Self::Amine(m) => m.get_atoms_mut(),
            Self::Alkane(m) => m.get_atoms_mut(),
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
            Self::Ether(eth) => eth.atomic_numbers(),
            Self::BorinicAcid(bor) => bor.atomic_numbers(),
            Self::Sulfide(sulf) => sulf.atomic_numbers(),
            Self::Amine(am) => am.atomic_numbers(),
            Self::Alkane(alk) => alk.atomic_numbers(),
        }
    }
}

impl Valuable for FunctionalGroup {
    fn value(&self) -> Value {
        match self {
            Self::Ether(eth) => eth.value(),
            Self::BorinicAcid(bor) => bor.value(),
            Self::Sulfide(sul) => sul.value(),
            Self::Amine(ami) => ami.value(),
            Self::Alkane(alk) => alk.value(),
        }
    }
}

impl Display for FunctionalGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let p = match self {
            FunctionalGroup::Ether(fg) => ("Ether", fg.value()),
            FunctionalGroup::BorinicAcid(fg) => ("BorinicAcid", fg.value()),
            FunctionalGroup::Sulfide(fg) => ("Sulfide", fg.value()),
            FunctionalGroup::Amine(fg) => ("Amine", fg.value()),
            FunctionalGroup::Alkane(fg) => ("Alkane", fg.value()),
        };
        let name = p.0;
        let val = p.1;
        f.write_fmt(format_args!("{name}({val})"))
    }
}

impl Add for FunctionalGroup {
    type Output = FunctionalGroup;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (FunctionalGroup::Ether(l), FunctionalGroup::Ether(r)) => FunctionalGroup::Ether(l + r),
            (FunctionalGroup::BorinicAcid(l), FunctionalGroup::BorinicAcid(r)) => {
                let l = l.value();
                let r = r.value();
                let v = l + r;
                FunctionalGroup::BorinicAcid(BorinicAcid::from(v))
            }
            (FunctionalGroup::Sulfide(l), FunctionalGroup::Sulfide(r)) => {
                let l = l.value();
                let r = r.value();
                let v = l + r;
                FunctionalGroup::Sulfide(Sulfide::from(v))
            }
            (FunctionalGroup::Amine(l), FunctionalGroup::Amine(r)) => {
                let l = l.value();
                let r = r.value();
                let v = l + r;
                FunctionalGroup::Amine(Amine::from(v))
            }
            (FunctionalGroup::Alkane(l), FunctionalGroup::Alkane(r)) => {
                let l = l.value();
                let r = r.value();
                let v = l + r;
                FunctionalGroup::Alkane(Alkane::from(v))
            }
            (l, r) => panic!("not supported for Add: {} and {}", l, r),
        }
    }
}
impl Sub for FunctionalGroup {
    type Output = FunctionalGroup;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (FunctionalGroup::Ether(l), FunctionalGroup::Ether(r)) => FunctionalGroup::Ether(l + r),
            (FunctionalGroup::BorinicAcid(l), FunctionalGroup::BorinicAcid(r)) => {
                let l = l.value();
                let r = r.value();
                let v = l - r;
                FunctionalGroup::BorinicAcid(BorinicAcid::from(v))
            }
            (FunctionalGroup::Sulfide(l), FunctionalGroup::Sulfide(r)) => {
                let l = l.value();
                let r = r.value();
                let v = l - r;
                FunctionalGroup::Sulfide(Sulfide::from(v))
            }
            (FunctionalGroup::Amine(l), FunctionalGroup::Amine(r)) => {
                let l = l.value();
                let r = r.value();
                let v = l - r;
                FunctionalGroup::Amine(Amine::from(v))
            }
            (FunctionalGroup::Alkane(l), FunctionalGroup::Alkane(r)) => {
                let l = l.value();
                let r = r.value();
                let v = l - r;
                FunctionalGroup::Alkane(Alkane::from(v))
            }
            (l, r) => panic!("not supported for Sub: {} and {}", l, r),
        }
    }
}

impl From<Value> for FunctionalGroup {
    fn from(value: Value) -> Self {
        match value {
            Value::Number(_) => FunctionalGroup::Ether(Ether::from(value)),
            Value::Boolean(_) => FunctionalGroup::BorinicAcid(BorinicAcid::from(value)),
            Value::String(_) => FunctionalGroup::Sulfide(Sulfide::from(value)),
            Value::Pair(_, _) => FunctionalGroup::Amine(Amine::from(value)),
            Value::List(_) | Value::Map(_) => FunctionalGroup::Alkane(Alkane::from(value)),
        }
    }
}
