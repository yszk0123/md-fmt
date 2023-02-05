use std::iter::Peekable;
use std::slice::Iter;

use anyhow::{anyhow, Ok, Result};
use markdown::mdast::{self as m, Paragraph};

use crate::ast;
use crate::note::model::*;
use crate::toc::Toc;
use crate::Metadata;

pub struct NoteParser {}

impl NoteParser {
    pub fn parse(node: &m::Node) -> Result<Note> {
        let parser = Self {};
        parser.parse_root(node)
    }

    fn parse_root(&self, node: &m::Node) -> Result<Note> {
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
        self.parse_block(iter, 1)
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
                m::Node::Heading(m::Heading { depth, .. }) if *depth <= min_depth => {
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
                    res.push(self.parse_block_quote(node)?);
                },
                node @ m::Node::FootnoteDefinition(_) => {
                    iter.next();
                    let s = ast::AstPrinter::print(node)?;
                    res.push(Block::Single(s.trim().to_string()));
                },
                node => {
                    iter.next();
                    let s = ast::AstPrinter::print(node)?;
                    res.push(Block::Text(s.trim().to_string()));
                },
            }
        }

        Ok(res)
    }

    fn parse_block_quote(&self, block_quote: &m::BlockQuote) -> Result<Block> {
        if block_quote.children.is_empty() {
            return Ok(Block::Empty);
        }

        let (first, rest) = block_quote.children.split_first().unwrap();
        let (node, children, kind) = self.find_note_kind(first);
        let lines = node
            .into_iter()
            .chain(children)
            .map(ast::AstPrinter::print)
            .chain(rest.iter().map(ast::AstPrinter::print))
            .collect::<Result<Vec<String>>>()?;

        match kind {
            Some(NoteKind::Toc) => Ok(Block::toc(Toc::parse_lines(lines)?.flatten_ref())),
            _ => Ok(Block::card(
                kind.unwrap_or(NoteKind::Note),
                vec![Block::text(lines.join("\n"))],
            )),
        }
    }

    fn find_note_kind<'a>(
        &self,
        node: &'a m::Node,
    ) -> (Option<&'a m::Node>, &'a [m::Node], Option<NoteKind>) {
        if let m::Node::Paragraph(Paragraph { children, .. }) = node {
            if let Some(m::Node::Text(m::Text { value, .. })) = children.first() {
                let (_, rest) = children.split_first().unwrap();
                match &value[..] {
                    "[!note]" => (None, rest, Some(NoteKind::Note)),
                    "[!question]" => (None, rest, Some(NoteKind::Question)),
                    "[!quote]" => (None, rest, Some(NoteKind::Quote)),
                    "[!summary]" => (None, rest, Some(NoteKind::Summary)),
                    "[!toc]" => (None, rest, Some(NoteKind::Toc)),
                    _ => (None, children, None),
                }
            } else {
                (None, children, None)
            }
        } else {
            (Some(node), &[], None)
        }
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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::{ast::builder::*, toc::FlattenNode};

    #[test]
    fn text_to_invalid() {
        let err = NoteParser::parse(&text("foo")).unwrap_err();
        assert_eq!(format!("{err}"), "invalid");
    }

    #[test]
    fn text_to_note() -> Result<()> {
        assert_eq!(
            NoteParser::parse(&root(vec![text("foo")]))?,
            Note::new(None, vec![Block::text("foo")], vec![]),
        );
        Ok(())
    }

    #[test]
    fn heading_2_to_note() -> Result<()> {
        assert_eq!(
            NoteParser::parse(&root(vec![heading(2, vec![text("foo")])]))?,
            Note::new(None, vec![Block::section("foo", vec![])], vec![]),
        );
        Ok(())
    }

    #[test]
    fn heading_1_to_note() -> Result<()> {
        assert_eq!(
            NoteParser::parse(&root(vec![heading(1, vec![text("foo")])]))?,
            Note::new(None, vec![], vec![Section::new("foo", vec![])]),
        );
        Ok(())
    }

    #[test]
    fn heading_1_2_to_note() -> Result<()> {
        assert_eq!(
            NoteParser::parse(&root(vec![
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
    fn heading_1_2_1_2_to_note() -> Result<()> {
        assert_eq!(
            NoteParser::parse(&root(vec![
                heading(1, vec![text("aaa")]),
                heading(2, vec![text("bbb")]),
                heading(1, vec![text("ccc")]),
                heading(2, vec![text("ddd")])
            ]))?,
            Note::new(
                None,
                vec![],
                vec![
                    Section::new("aaa", vec![Block::section("bbb", vec![])]),
                    Section::new("ccc", vec![Block::section("ddd", vec![])])
                ]
            ),
        );
        Ok(())
    }

    #[test]
    fn heading_2_1_to_note() -> Result<()> {
        assert_eq!(
            NoteParser::parse(&root(vec![
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
    fn block_quote_paragraph_to_note() -> Result<()> {
        assert_eq!(
            NoteParser::parse(&root(vec![
                block_quote(vec![paragraph(vec![text("foo")])]),
                block_quote(vec![paragraph(vec![text("[!note]"), text("foo")])]),
                block_quote(vec![paragraph(vec![text("[!summary]"), text("foo")])]),
                block_quote(vec![paragraph(vec![text("[!quote]"), text("foo")])]),
                block_quote(vec![paragraph(vec![text("[!question]"), text("foo")])]),
                block_quote(vec![paragraph(vec![text("[!toc]"), text("- foo")])]),
            ]))?,
            Note::new(
                None,
                vec![
                    Block::card(NoteKind::Note, vec![Block::text("foo")]),
                    Block::card(NoteKind::Note, vec![Block::text("foo")]),
                    Block::card(NoteKind::Summary, vec![Block::text("foo")]),
                    Block::card(NoteKind::Quote, vec![Block::text("foo")]),
                    Block::card(NoteKind::Question, vec![Block::text("foo")]),
                    Block::toc(vec![FlattenNode(1, String::from("foo"))]),
                ],
                vec![]
            )
        );
        Ok(())
    }

    #[test]
    fn yaml_to_note() -> Result<()> {
        assert_eq!(
            NoteParser::parse(&root(vec![yaml("title: foo")]))?,
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
            NoteParser::parse(&root(vec![text("foo")]))?,
            Note::new(None, vec![Block::text("foo")], vec![])
        );
        Ok(())
    }
}
