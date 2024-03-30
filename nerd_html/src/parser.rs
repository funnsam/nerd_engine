use crate::token::*;
use nerd_dom::*;
use std::collections::HashMap;

const VOID_ELMS: &[&str] = &["area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "source", "track", "wbr"];

struct Parser {
    tokens: Vec<Token>,
    idx: usize,
}

impl Parser {
    fn next(&mut self) -> Option<&Token> {
        self.idx += 1;
        self.tokens.get(self.idx - 1)
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.get(self.idx)
    }

    fn parse_elem(&mut self) -> Node {
        match self.next() {
            Some(Token::Tag(Tag { content: tag, kind: TagKind::TagOpen, attr })) => if !VOID_ELMS.contains(&tag.as_str()) {
                let tag = tag.clone();
                let attr = attr.into_iter().cloned().collect();
                let children = self.parse_children();
                assert!(matches!(self.next(), Some(Token::Tag(Tag { kind: TagKind::TagEnd, .. }))));
                Node { kind: NodeKind::Element(Element { tag, attr }), children }
            } else {
                Node { kind: NodeKind::Element(Element { tag: tag.clone(), attr: attr.into_iter().cloned().collect() }), children: vec![] }
            }
            Some(Token::String(c)) => Node { kind: NodeKind::Text(c.to_string()), children: vec![] },
            _ => todo!(),
        }
    }

    fn parse_children(&mut self) -> Vec<Node> {
        let mut l = Vec::new();

        while matches!(self.peek(), Some(Token::Tag(Tag { kind: TagKind::TagOpen, .. }) | Token::String(_))) {
            l.push(self.parse_elem())
        }

        l
    }
}

pub fn parse(t: Vec<Token>) -> Node {
    let mut parser = Parser {
        tokens: t,
        idx: 0,
    };

    let mut nodes = parser.parse_children();

    if nodes.len() == 1 {
        nodes.swap_remove(0)
    } else {
        Node { kind: NodeKind::Element(Element { tag: "html".to_string(), attr: HashMap::new() }), children: nodes }
    }
}
