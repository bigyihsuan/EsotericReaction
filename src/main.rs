mod eval;
mod lex;
mod tok;
mod util;

use petgraph::dot::{self, Config};

use crate::eval::{
    alkane::{Alkane, Molecule},
    functional_groups::FunctionalGroup,
};

fn main() {
    // let code = r"C_3H_8+5O_2 -> 3CO_2+4H_2O".to_string();
    // let mut lexer = lex::Lexer::new(code.to_string());

    // let tokens = lexer.all_tokens();
    // println!("{:?}", tokens);

    // let mut alk = Alkane::new();
    // println!("{:?}", &alk);

    // let mut tree: Tree<Atom> = Tree::new();

    let dot_config = [Config::EdgeNoLabel];

    let mut alk = Alkane::new();

    alk.add_carbon_after();
    alk.add_carbon_before();
    alk.add_carbon_after();
    alk.add_functional_group(FunctionalGroup::Amine);
    alk.move_up();
    alk.add_functional_group(FunctionalGroup::Hydride);
    alk.add_functional_group(FunctionalGroup::Borinic);
    alk.move_down();
    alk.move_down();
    alk.add_functional_group(FunctionalGroup::Ether);
    alk.add_molecule(eval::alkane::Molecule::E(eval::alkane::Element::Oxygen));
    alk.add_element(eval::alkane::Element::Hydrogen);
    println!("{:?}", dot::Dot::with_config(&alk.atoms, &dot_config));
    // alk.move_up()
}
