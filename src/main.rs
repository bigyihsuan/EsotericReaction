mod lex;
mod tok;

fn main() {
    let code = r"C_3H_8+5O_2 -> 3CO_2+4H_2O".to_string();
    let mut lexer = lex::Lexer::new(code.to_string());

    let tokens = lexer.all_tokens();
    println!("{:?}", tokens)
}
