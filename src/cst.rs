use std::rc::Rc;

use crate::{lexer::TokenStream, types::*};

#[derive(Debug)]
pub struct Cst {
    doc: Rc<Document>,
    token_stream: TokenStream,
    // the tokens in a pre-visit order.
    root: Node,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    kind: SyntaxKind,
    idx: usize,
    children: Vec<Node>,
}

impl Node {
    pub(crate) fn new(kind: SyntaxKind, idx: usize) -> Self {
        Self { kind, idx, children: vec![] }
    }
    pub(crate) fn add_child(&mut self, node: Node) -> &mut Self {
        self.children.push(node);
        self
    }
    pub(crate) fn extend_children(&mut self, nodes: impl IntoIterator<Item = Node>) -> &mut Self {
        self.children.extend(nodes);
        self
    }
}
