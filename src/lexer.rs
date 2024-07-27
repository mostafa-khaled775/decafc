use std::ops::Index;

use logos::{Logos, Span};

use crate::types::{Document, SyntaxKind};

pub struct Token(usize, SyntaxKind);

#[derive(Debug)]
pub struct TokenStream {
    tokens: Vec<SyntaxKind>,
    spans: Vec<Span>,
    doc: Document
}

impl TokenStream {
    pub fn get(&self, idx: usize) -> Option<SyntaxKind> {
        self.tokens.get(idx).copied()
    }
}

impl Index<usize> for TokenStream {
    type Output = SyntaxKind;
    fn index(&self, index: usize) -> &Self::Output {
        &self.tokens[index]
    }
}

pub struct Lexer;

impl Lexer {
    pub fn tokenize(doc: String) -> TokenStream {
        let mut lexer = SyntaxKind::lexer(&doc);
        let mut tokens = vec![];
        let mut spans = vec![];
        while let Some(token) = lexer.next() {
            tokens.push(token.unwrap());
            spans.push(lexer.span());
        }
        TokenStream { tokens, spans, doc: doc.into() }
    }
}
