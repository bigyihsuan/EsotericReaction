mod args;
mod eval;
mod lex;
mod par;
mod util;

use args::parse_args;
use lex::lex::Lexer;
use par::par::Parser;

fn main() {
    // let code = r"C_3H_8+5O_2 -> 3CO_2+4H_2O".to_string();
    // let mut lexer = lex::Lexer::new(code.to_string());

    // let tokens = lexer.all_tokens();
    // println!("{:?}", tokens);

    // let mut alk = Alkane::new();
    // println!("{:?}", &alk);

    // let mut tree: Tree<Atom> = Tree::new();

    // let three = Ether::from(3);
    // let ten = Ether::from(10);
    // eprintln!("3={} 10={}", three.value(), ten.value());
    // let thirteen = three - ten;

    // let dot_config = [Config::EdgeNoLabel];
    // println!(
    //     "{:?}",
    //     dot::Dot::with_config(&thirteen.flatten().atoms(), &dot_config)
    // );

    let source = parse_args();

    let mut lexer = Lexer::new(source);
    let tokens = lexer.all_tokens();

    tokens.iter().for_each(|tok| println!("{}", tok));

    let mut parser = Parser::new(tokens);
    println!("{parser:?}")
}
