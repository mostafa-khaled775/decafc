use logos::{Logos, Span};

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
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

pub struct Token(usize, TokenKind);

#[derive(Debug, Clone)]
pub struct TokenStream {
    tokens: Vec<TokenKind>,
    spans: Vec<Span>,
    doc: String
}

pub struct Lexer;

impl Lexer {
    pub fn tokenize(doc: String) -> TokenStream {
        let mut lexer = TokenKind::lexer(&doc);
        let mut tokens = vec![];
        let mut spans = vec![];
        while let Some(token) = lexer.next() {
            tokens.push(token.unwrap());
            spans.push(lexer.span());
        }
        TokenStream { tokens, spans, doc }
    }
}
