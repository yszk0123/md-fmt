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
                let body = self.parse_body(&mut iter, 1)?;

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

    fn parse_head(&self, iter: &mut Peekable<Iter<m::Node>>) -> Result<Block> {
        self.parse_body(iter, 2)
    }

    fn parse_body(&self, iter: &mut Peekable<Iter<m::Node>>, min_depth: u8) -> Result<Block> {
        let mut res: Vec<NoteNode> = vec![];
        while let Some(node) = iter.peek() {
            match *node {
                m::Node::Heading(m::Heading { depth, .. })
                    if min_depth >= 2 && *depth < min_depth =>
                {
                    break;
                },
                m::Node::Heading(node) => {
                    iter.next();
                    let mut block = self.parse_body(iter, node.depth)?;
                    block.title = Some(self.parse_heading(node.clone()));
                    res.push(NoteNode::Block(block));
                },
                m::Node::BlockQuote(node) => {
                    iter.next();
                    res.push(self.parse_block_quote(node));
                },
                node => {
                    iter.next();
                    res.push(NoteNode::Node(node.clone()));
                },
            }
        }

        Ok(Block::new(None, res))
    }

    fn parse_block_quote(&self, block_quote: &m::BlockQuote) -> NoteNode {
        NoteNode::card(
            NoteKind::default(),
            block_quote
                .children
                .iter()
                .map(|node| NoteNode::Node(node.clone()))
                .collect(),
        )
    }

    fn parse_heading(&self, heading: m::Heading) -> String {
        heading
            .children
            .into_iter()
            .flat_map(|node| match node {
                m::Node::Text(m::Text { value, .. }) => Some(value),
                _ => None,
            })
            .collect()
    }
}

pub fn to_note(node: &m::Node) -> Result<Note> {
    let parser = Parser {};
    parser.parse(node)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::md::builder::*;

    #[test]
    fn text_to_invalid() {
        let err = to_note(&text("foo")).unwrap_err();
        assert_eq!(format!("{err}"), "invalid");
    }

    #[test]
    fn text_to_note() -> Result<()> {
        assert_eq!(
            to_note(&root(vec![text("foo")]))?,
            Note::new(
                None,
                Block::new(None, vec![NoteNode::Node(text("foo"))]),
                Block::default()
            ),
        );
        Ok(())
    }

    #[test]
    fn heading_2_to_note() -> Result<()> {
        assert_eq!(
            to_note(&root(vec![heading(2, vec![text("foo")])]))?,
            Note::new(
                None,
                Block::children(vec![NoteNode::block("foo", vec![])]),
                Block::default()
            ),
        );
        Ok(())
    }

    #[test]
    fn heading_1_to_note() -> Result<()> {
        assert_eq!(
            to_note(&root(vec![heading(1, vec![text("foo")])]))?,
            Note::new(
                None,
                Block::default(),
                Block::children(vec![NoteNode::block("foo", vec![])])
            ),
        );
        Ok(())
    }

    #[test]
    fn heading_1_2_to_note() -> Result<()> {
        assert_eq!(
            to_note(&root(vec![
                heading(1, vec![text("foo")]),
                heading(2, vec![text("bar")])
            ]))?,
            Note::new(
                None,
                Block::default(),
                Block::children(vec![NoteNode::block(
                    "foo",
                    vec![NoteNode::block("bar", vec![])]
                )])
            ),
        );
        Ok(())
    }

    #[test]
    fn heading_2_1_to_note() -> Result<()> {
        assert_eq!(
            to_note(&root(vec![
                heading(2, vec![text("foo")]),
                heading(1, vec![text("bar")]),
            ]))?,
            Note::new(
                None,
                Block::children(vec![NoteNode::block("foo", vec![])]),
                Block::children(vec![NoteNode::block("bar", vec![])])
            ),
        );
        Ok(())
    }

    #[test]
    fn block_quote_to_note() -> Result<()> {
        assert_eq!(
            to_note(&root(vec![block_quote(vec![text("foo")])]))?,
            Note::new(
                None,
                Block::new(
                    None,
                    vec![NoteNode::card(
                        NoteKind::default(),
                        vec![NoteNode::Node(text("foo"))]
                    )]
                ),
                Block::default()
            ),
        );
        Ok(())
    }

    #[test]
    fn yaml_to_note() -> Result<()> {
        assert_eq!(
            to_note(&root(vec![yaml("title: foo")]))?,
            Note::new(
                Some(Metadata {
                    title: Some("foo".into()),
                    ..Default::default()
                }),
                Block::default(),
                Block::default()
            )
        );
        Ok(())
    }

    #[test]
    fn root_to_note() -> Result<()> {
        assert_eq!(
            to_note(&root(vec![text("foo")]))?,
            Note {
                head: Block::new(None, vec![NoteNode::Node(text("foo"))]),
                ..Default::default()
            }
        );
        Ok(())
    }
}
