use crate::{lexer::TokenStream, types::*, cst::Node};
// Define the Parser struct and implement methods
pub struct Parser {
    tokens: TokenStream,
    current: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Expected(SyntaxKind),
    UnexpectedEof,
}

type Result<T> = core::result::Result<T, Error>;

impl Parser {
    pub fn new(tokens: TokenStream) -> Self {
        Parser { tokens, current: 0 }
    }

    fn current_token(&self) -> Option<SyntaxKind> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) -> Node {
        let sk = self.current_token().unwrap();
        self.current += 1;
        Node::new(sk, self.current - 1)
    }

    fn one_of<const N: usize>(&mut self, kinds: [SyntaxKind; N]) -> Result<Node> {
        for expected_kind in kinds {
            if let Some(kind) = self.current_token() {
                if expected_kind == kind {
                    let idx = self.current;
                    self.advance();
                    return Ok(Node::new(kind, idx))
                }
            }
        }
        todo!()
    }

    fn series<const N: usize>(&mut self, kinds: [SyntaxKind; N]) -> Result<Vec<Node>> {
        let mut nodes = vec![];
        for expected_kind in kinds {
            if let Some(kind) = self.current_token() {
                if expected_kind == kind {
                    nodes.push(self.advance())
                }
            } else {
                return Err(Error::UnexpectedEof)
            }
        }
        Ok(nodes)
    }

    fn parse_import(&mut self) -> Result<Node> {
        let mut import_node = self.one_of([SyntaxKind::Import])?;
        import_node
            .extend_children(self.series([SyntaxKind::Identifier, SyntaxKind::Semicolon])?);
        Ok(import_node)
    }

    fn parse_decl(&mut self) -> Result<Node> {
        fn parse_var(&mut self) -> Result<Node> {
            let id = self.one_of([SyntaxKind::Identifier])?;

        }
        let mut decl_node = self.one_of([SyntaxKind::Int, SyntaxKind::Bool])?;
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::Lexer;

    use super::*;
    use SyntaxKind as SK;

    #[test]
    fn simple_import() {
        let ts = Lexer::tokenize("import printf ;".to_string());
        let mut expected = Node::new(SK::Import, 0);
        expected.add_child(Node::new(SK::Identifier, 1))
            .add_child(Node::new(SK::Semicolon, 2));
        assert_eq!(Parser::new(ts).parse_import().unwrap(),expected)
    }

    #[test]
    #[should_panic]
    fn invalid_import() {
        let ts = Lexer::tokenize("import printf ".to_string());
        Parser::new(ts).parse_import().unwrap();
    }
}
