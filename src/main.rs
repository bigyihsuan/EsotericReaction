mod eval;
mod lex;
mod tok;
mod util;

use eval::alkane::AlkaneElement;
use eval::value::Valuable;
use petgraph::dot::{self, Config};

use crate::eval::alkane::Alkane;
use crate::eval::atom_like::AtomLike;
use crate::eval::functional_groups::amine::Amine;
use crate::eval::functional_groups::{FgElement, FunctionalGroup};
use crate::eval::{element::Element, molecule::Molecule};

fn main() {
    // let code = r"C_3H_8+5O_2 -> 3CO_2+4H_2O".to_string();
    // let mut lexer = lex::Lexer::new(code.to_string());

    // let tokens = lexer.all_tokens();
    // println!("{:?}", tokens);

    // let mut alk = Alkane::new();
    // println!("{:?}", &alk);

    // let mut tree: Tree<Atom> = Tree::new();

    let dot_config = [Config::EdgeNoLabel];

    // let mut alk = Alkane::new_n_alkane(4);
    // let ane = Alkane::new_n_alkane(2);
    // alk.move_down();
    // alk.move_down();
    // alk.add_alkane(ane);
    // let eth = FunctionalGroup::new_ether(Molecule::E(Element::N));
    // let bor = FunctionalGroup::new_borinic_acid(Molecule::Fg(eth.clone()));
    // alk.move_down();
    // alk.add_functional_group(bor.clone());

    // dbg!(&eth.value());
    // dbg!(&bor.value());

    let h = FunctionalGroup::new_ether(FgElement::E(Element::Hf));
    let e = FunctionalGroup::new_ether(FgElement::E(Element::Md));
    let l = FunctionalGroup::new_ether(FgElement::E(Element::Hs));
    let o = FunctionalGroup::new_ether(FgElement::E(Element::Rg));

    let mut alk = Alkane::new_n_alkane(5);
    alk.fill(vec![
        AlkaneElement::F(h.clone()),
        AlkaneElement::F(e.clone()),
        AlkaneElement::F(l.clone()),
        AlkaneElement::F(l.clone()),
        AlkaneElement::F(o.clone()),
    ]);

    let hello = FunctionalGroup::new_sulfide(FgElement::F(FunctionalGroup::Alkane(alk)));
    let six = FunctionalGroup::new_ether(FgElement::E(Element::C));

    let amine = FunctionalGroup::new_amine(FgElement::F(six), FgElement::F(hello));
    dbg!(&amine.value());
    println!(
        "{:?}",
        dot::Dot::with_config(&amine.flatten().atoms(), &dot_config)
    );
}
