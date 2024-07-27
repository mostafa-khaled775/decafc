mod lexer;
mod cst;
mod types;
mod parser;

use lexer::Lexer;

fn main() {
    println!("{:?}", Lexer::tokenize("import printf;".to_string()));
}
