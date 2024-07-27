use std::{fs, io, path::{Path, PathBuf}};

use logos::Logos;
use strum::Display;

#[derive(Debug)]
pub struct Document {
    content: String,
    file_path: Option<PathBuf>,
}

impl From<String> for Document {
    fn from(value: String) -> Self {
        Document { content: value, file_path: None }
    }
}

impl TryFrom<&Path> for Document {
    type Error = io::Error;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        Ok(Document{ content: fs::read_to_string(value)?, file_path: Some(value.to_path_buf()) })
    }
}

#[derive(Logos, Debug, PartialEq, Eq, Clone, Copy, Display)]
#[repr(u8)]
pub enum SyntaxKind {
    /// The CST root
    Program,
    // Keywords
    #[token("import")]
    Import,
    #[token("int")]
    Int,
    #[token("bool")]
    Bool,
    #[token("void")]
    Void,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("for")]
    For,
    #[token("while")]
    While,
    #[token("return")]
    Return,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,
    #[token("len")]
    Len,
    #[token("true")]
    True,
    #[token("false")]
    False,

    // Operators
    #[token("=")]
    Assign,
    #[token("+=")]
    PlusAssign,
    #[token("-=")]
    MinusAssign,
    #[token("++")]
    Increment,
    #[token("--")]
    Decrement,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("<")]
    LessThan,
    #[token(">")]
    GreaterThan,
    #[token("<=")]
    LessEqual,
    #[token(">=")]
    GreaterEqual,
    #[token("==")]
    Equal,
    #[token("!=")]
    NotEqual,
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("!")]
    Not,
    #[token("?")]
    Question,
    #[token(":")]
    Colon,

    // Delimiters
    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,

    // Literals
    #[regex(r"[0-9]+")]
    Integer,
    #[regex(r"0x[0-9a-fA-F]+")]
    HexInteger,
    #[regex(r"'[^']'")]
    Char,
    #[regex(r#""[^"]*""#)]
    String,

    // Identifiers
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    // Whitespace and comments
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
    #[regex(r"//[^\n]*", logos::skip)]
    LineComment,
    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
    BlockComment,
}
