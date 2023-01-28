use std::iter::Peekable;
use std::slice::Iter;

use anyhow::{anyhow, Result};
use markdown::mdast as m;

use crate::note::model::*;
use crate::Metadata;

struct Parser {}

impl Parser {
    fn parse(&self, node: &m::Node) -> Result<Note> {
        match node {
            m::Node::Root(node) => {
                let mut iter = node.children.iter().peekable();

                let metadata = self.parse_metadata(&mut iter)?;
                let head = self.parse_head(&mut iter)?;
                let body = self.parse_body(&mut iter)?;

                Ok(Note {
                    metadata,
                    head,
                    body,
                })
            },
            _ => Err(anyhow!("invalid")),
        }
    }

    fn parse_metadata(&self, iter: &mut Peekable<Iter<m::Node>>) -> Result<Option<Metadata>> {
        if let Some(m::Node::Yaml(node)) = iter.peek() {
            iter.next();
            return Ok(Some(Metadata::from_str(&node.value)?));
        }
        Ok(None)
    }

    fn parse_head(&self, iter: &mut Peekable<Iter<m::Node>>) -> Result<Vec<Block>> {
        self.parse_block(iter, 2)
    }

    fn parse_body(&self, iter: &mut Peekable<Iter<m::Node>>) -> Result<Vec<Section>> {
        let mut res: Vec<Section> = vec![];
        while let Some(node) = iter.peek() {
            match *node {
                m::Node::Heading(h @ m::Heading { depth, .. }) if *depth == 1 => {
                    iter.next();
                    let title = self.parse_heading(h);
                    let children = self.parse_block(iter, *depth)?;
                    res.push(Section::new(title, children));
                },
                _ => {
                    // Ignore Node
                    iter.next();
                },
            }
        }

        Ok(res)
    }

    fn parse_block(&self, iter: &mut Peekable<Iter<m::Node>>, min_depth: u8) -> Result<Vec<Block>> {
        let mut res: Vec<Block> = vec![];
        while let Some(node) = iter.peek() {
            match *node {
                m::Node::Heading(m::Heading { depth, .. })
                    if min_depth >= 2 && *depth < min_depth =>
                {
                    break;
                },
                m::Node::Heading(node) => {
                    iter.next();
                    let title = self.parse_heading(node);
                    let children = self.parse_block(iter, node.depth)?;
                    res.push(Block::section(title, children));
                },
                m::Node::BlockQuote(node) => {
                    iter.next();
                    res.push(self.parse_block_quote(node));
                },
                node => {
                    iter.next();
                    res.push(Block::Node(node.clone()));
                },
            }
        }

        Ok(res)
    }

    fn parse_block_quote(&self, block_quote: &m::BlockQuote) -> Block {
        Block::card(
            NoteKind::default(),
            block_quote
                .children
                .iter()
                .map(|node| Block::Node(node.clone()))
                .collect(),
        )
    }

    fn parse_heading(&self, heading: &m::Heading) -> String {
        heading
            .children
            .iter()
            .flat_map(|node| match node {
                m::Node::Text(m::Text { value, .. }) => Some(value.clone()),
                _ => None,
            })
            .collect()
    }
}

pub fn from_ast(node: &m::Node) -> Result<Note> {
    let parser = Parser {};
    parser.parse(node)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::builder::*;

    #[test]
    fn text_to_invalid() {
        let err = from_ast(&text("foo")).unwrap_err();
        assert_eq!(format!("{err}"), "invalid");
    }

    #[test]
    fn text_to_note() -> Result<()> {
        assert_eq!(
            from_ast(&root(vec![text("foo")]))?,
            Note::new(None, vec![Block::Node(text("foo"))], vec![]),
        );
        Ok(())
    }

    #[test]
    fn heading_2_to_note() -> Result<()> {
        assert_eq!(
            from_ast(&root(vec![heading(2, vec![text("foo")])]))?,
            Note::new(None, vec![Block::section("foo", vec![])], vec![]),
        );
        Ok(())
    }

    #[test]
    fn heading_1_to_note() -> Result<()> {
        assert_eq!(
            from_ast(&root(vec![heading(1, vec![text("foo")])]))?,
            Note::new(None, vec![], vec![Section::new("foo", vec![])]),
        );
        Ok(())
    }

    #[test]
    fn heading_1_2_to_note() -> Result<()> {
        assert_eq!(
            from_ast(&root(vec![
                heading(1, vec![text("foo")]),
                heading(2, vec![text("bar")])
            ]))?,
            Note::new(
                None,
                vec![],
                vec![Section::new("foo", vec![Block::section("bar", vec![])])]
            ),
        );
        Ok(())
    }

    #[test]
    fn heading_2_1_to_note() -> Result<()> {
        assert_eq!(
            from_ast(&root(vec![
                heading(2, vec![text("foo")]),
                heading(1, vec![text("bar")]),
            ]))?,
            Note::new(
                None,
                vec![Block::section("foo", vec![])],
                vec![Section::new("bar", vec![])]
            ),
        );
        Ok(())
    }

    #[test]
    fn block_quote_to_note() -> Result<()> {
        assert_eq!(
            from_ast(&root(vec![block_quote(vec![text("foo")])]))?,
            Note::new(
                None,
                vec![Block::card(
                    NoteKind::default(),
                    vec![Block::Node(text("foo"))]
                )],
                vec![]
            ),
        );
        Ok(())
    }

    #[test]
    fn yaml_to_note() -> Result<()> {
        assert_eq!(
            from_ast(&root(vec![yaml("title: foo")]))?,
            Note::new(
                Some(Metadata {
                    title: Some("foo".into()),
                    ..Default::default()
                }),
                vec![],
                vec![]
            )
        );
        Ok(())
    }

    #[test]
    fn root_to_note() -> Result<()> {
        assert_eq!(
            from_ast(&root(vec![text("foo")]))?,
            Note::new(None, vec![Block::Node(text("foo"))], vec![])
        );
        Ok(())
    }
}
