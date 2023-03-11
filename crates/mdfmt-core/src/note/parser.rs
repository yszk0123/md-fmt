use std::iter::Peekable;
use std::slice::Iter;

use anyhow::{anyhow, Ok, Result};
use itertools::Itertools;
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
            return Ok(Some(
                Metadata::from_str(&node.value)
                    .unwrap_or_else(|_| Metadata::Raw(node.value.to_owned())),
            ));
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
                    let title = self.parse_heading(h)?;
                    let children = self.parse_block(iter, *depth)?;
                    res.push(Section::new(&title, children));
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
                    let title = self.parse_heading(node)?;
                    let children = self.parse_block(iter, node.depth)?;
                    res.push(Block::section(&title, children));
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
        let (kind, node) = self
            .parse_card(first)
            .unwrap_or_else(|| (NoteKind::default(), Some(first.clone())));
        let lines = if let Some(node) = node {
            [&[node], rest].concat()
        } else {
            rest.to_vec()
        }
        .iter()
        .map(ast::AstPrinter::print)
        .collect::<Result<Vec<String>>>()?;

        match kind {
            NoteKind::Toc => {
                let lines = lines
                    .iter()
                    .flat_map(|v| v.split('\n'))
                    .map(String::from)
                    .collect::<Vec<String>>();
                Ok(Block::toc(Toc::parse_lines(lines)?.flatten_ref()))
            },
            _ => Ok(Block::card(kind, vec![Block::text(&lines.join("\n\n"))])),
        }
    }

    fn parse_card(&self, node: &m::Node) -> Option<(NoteKind, Option<m::Node>)> {
        if let m::Node::Paragraph(Paragraph { children, .. }) = node {
            if let Some((m::Node::Text(m::Text { value, .. }), rest)) = children.split_first() {
                match self.parse_card_paragraph(value) {
                    Some((kind, s)) => Some((
                        kind,
                        if s.is_empty() && rest.is_empty() {
                            None
                        } else {
                            Some(m::Node::Paragraph(Paragraph {
                                children: [
                                    &[m::Node::Text(m::Text {
                                        value: s,
                                        position: None,
                                    })],
                                    rest,
                                ]
                                .concat(),
                                position: None,
                            }))
                        },
                    )),
                    _ => None,
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    // Example:
    // > [!note]
    // > content
    fn parse_card_paragraph(&self, value: &str) -> Option<(NoteKind, String)> {
        let mut lines = value.lines();
        if let Some(v) = lines.next() {
            match v {
                "[!note]" => Some((NoteKind::Note, lines.join("\n"))),
                "[!question]" => Some((NoteKind::Question, lines.join("\n"))),
                "[!quote]" => Some((NoteKind::Quote, lines.join("\n"))),
                "[!summary]" => Some((NoteKind::Summary, lines.join("\n"))),
                "[!toc]" => Some((NoteKind::Toc, lines.join("\n"))),
                _ => None,
            }
        } else {
            None
        }
    }

    fn parse_heading(&self, heading: &m::Heading) -> Result<String> {
        let mut res: Vec<String> = vec![];

        for node in heading.children.iter() {
            res.push(ast::AstPrinter::print(node)?);
        }

        Ok(res.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::{ast::builder::*, note::metadata::Meta, toc::FlattenNode};

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
                block_quote(vec![paragraph(vec![text("[!note]\nfoo")])]),
                block_quote(vec![paragraph(vec![text("[!summary]\nfoo")])]),
                block_quote(vec![paragraph(vec![text("[!quote]\nfoo")])]),
                block_quote(vec![paragraph(vec![text("[!question]\nfoo")])]),
                block_quote(vec![paragraph(vec![text("[!toc]\n- foo")])]),
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
                Some(Metadata::Meta(Meta {
                    title: Some("foo".into()),
                    ..Default::default()
                })),
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
