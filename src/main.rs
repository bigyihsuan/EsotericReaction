mod eval;
mod lex;
mod tok;
mod util;

use petgraph::dot::{self, Config};

use crate::eval::atoms::{Alkane, Element, FunctionalGroup, Molecule};

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
    let propane = Alkane::new_n_alkane(3);

    alk.add_carbon_after();
    // alk.add_carbon_before();
    alk.add_carbon_after();

    // println!("{:?}", dot::Dot::with_config(&alk.atoms, &dot_config));
    println!("{:?}", dot::Dot::with_config(&alk.flatten(), &dot_config));
    // alk.move_up()
}
