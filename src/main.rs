mod lexer;
use lexer::Lexer;

fn main() {
    println!("{:?}", Lexer::tokenize("import printf;".to_string()));
}
