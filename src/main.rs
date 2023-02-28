mod eval;
mod lex;
mod par;
mod util;

use lex::lexer::Lexer;
use par::parser::Parser;
use util::args::parse_args;

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

    let source = parse_args();
    eprintln!("{source}\n");

    let mut lexer = Lexer::new(source);
    let tokens = lexer.all_tokens();

    tokens.iter().for_each(|tok| eprintln!("{tok}"));
    eprintln!("\n");

    let mut parser = Parser::new(tokens);
    // println!("{parser:?}")
    let result = parser.parse();
    match result {
        Ok(result) => println!("{result:?}"),
        Err(error) => eprintln!("{error:?}"),
    }
}
