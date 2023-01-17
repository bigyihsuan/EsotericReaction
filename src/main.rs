mod lex;
mod tok;

fn main() {
    let code = r"Uue: Abc ; equation here
; usage
ABC + Uue -> AUue + BC ; use like any other element"
        .to_string();
    let mut lexer = lex::Lexer::new(code.to_string());

    let tokens = lexer.all_tokens();
    println!("{:?}", tokens)
}
