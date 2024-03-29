use std::iter::Peekable;
use std::slice::Iter;
use std::str::FromStr;

use anyhow::{anyhow, Ok, Result};
use itertools::Itertools;
use markdown::mdast::{self as m, Paragraph};

use super::{block::Block, metadata::Metadata, note_data::Note, note_kind::NoteKind, toc::Toc};
use crate::printer::Printer;

pub struct NoteParser {}

const KIND_LIST: &[(&str, &NoteKind)] = &[
    ("[!note]", &NoteKind::Note),
    ("[!question]", &NoteKind::Question),
    ("[!quote]", &NoteKind::Quote),
    ("[!summary]", &NoteKind::Summary),
    ("[!toc]", &NoteKind::Toc),
    ("[!todo]", &NoteKind::Todo),
];

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
                let mut body = self.parse_head_block(&mut iter)?;
                let rest = self.parse_block(&mut iter, 0)?;
                body.extend(rest);

                Ok(Note { metadata, body })
            },
            _ => Err(anyhow!("invalid")),
        }
    }

    fn parse_metadata(&self, iter: &mut Peekable<Iter<m::Node>>) -> Result<Option<Metadata>> {
        let Some(m::Node::Yaml(node)) = iter.peek() else {
            return Ok(None);
        };

        iter.next();
        Ok(Some(
            Metadata::from_str(&node.value)
                .unwrap_or_else(|_| Metadata::Raw(node.value.to_owned())),
        ))
    }

    fn parse_head_block(&self, iter: &mut Peekable<Iter<m::Node>>) -> Result<Vec<Block>> {
        let res: Vec<Block> = self.parse_block(iter, 1)?;

        Ok(if res.is_empty() {
            vec![]
        } else {
            vec![Block::anonymous_section(res)]
        })
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
                    let s = node.print(())?;
                    res.push(Block::Single(s.trim().to_string()));
                },
                node => {
                    iter.next();
                    let s = node.print(())?;
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
        let (kind, title, node) = self
            .parse_card(first)
            .unwrap_or_else(|| (NoteKind::default(), None, Some(first.clone())));
        let lines = if let Some(node) = node {
            [&[node], rest].concat()
        } else {
            rest.to_vec()
        }
        .iter()
        .map(|v| v.print(()))
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
            _ => Ok(Block::card(
                kind,
                title,
                vec![Block::text(&lines.join("\n\n"))],
            )),
        }
    }

    fn parse_card(&self, node: &m::Node) -> Option<(NoteKind, Option<String>, Option<m::Node>)> {
        let m::Node::Paragraph(Paragraph { children, .. }) = node else {
            return None;
        };

        let Some((m::Node::Text(m::Text { value, .. }), rest)) = children.split_first() else {
            return None;
        };

        let Some((kind, title, s)) = self.parse_card_paragraph(value) else {
            return None;
        };

        if s.is_empty() && rest.is_empty() {
            return Some((kind, title, None));
        };

        Some((
            kind,
            title,
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
            })),
        ))
    }

    // Example:
    // > [!note]
    // > content
    fn parse_card_paragraph(&self, value: &str) -> Option<(NoteKind, Option<String>, String)> {
        let mut lines = value.lines();
        let (kind, title) = self.parse_card_kind(lines.next()?)?;
        Some((kind.clone(), title, lines.join("\n")))
    }

    fn parse_card_kind(&self, line: &str) -> Option<(&NoteKind, Option<String>)> {
        for (kind_str, kind) in KIND_LIST {
            if line.starts_with(kind_str) {
                let title = line.strip_prefix(kind_str)?.trim_start();
                let title = if title.is_empty() {
                    None
                } else {
                    Some(title.to_string())
                };
                return Some((kind, title));
            }
        }
        None
    }

    fn parse_heading(&self, heading: &m::Heading) -> Result<String> {
        let mut res: Vec<String> = vec![];

        for node in heading.children.iter() {
            res.push(node.print(())?);
        }

        Ok(res.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::{ast::builder::*, note::Meta, FlattenNode};

    #[test]
    fn text_to_invalid() {
        let err = NoteParser::parse(&text("foo")).unwrap_err();
        assert_eq!(format!("{err}"), "invalid");
    }

    #[test]
    fn text_to_note() -> Result<()> {
        assert_eq!(
            NoteParser::parse(&root(vec![text("foo")]))?,
            Note::new(
                None,
                vec![Block::anonymous_section(vec![Block::text("foo")])]
            ),
        );
        Ok(())
    }

    #[test]
    fn heading_2_to_note() -> Result<()> {
        assert_eq!(
            NoteParser::parse(&root(vec![heading(2, vec![text("foo")])]))?,
            Note::new(
                None,
                vec![Block::anonymous_section(vec![Block::section(
                    "foo",
                    vec![]
                )])]
            ),
        );
        Ok(())
    }

    #[test]
    fn heading_1_to_note() -> Result<()> {
        assert_eq!(
            NoteParser::parse(&root(vec![heading(1, vec![text("foo")])]))?,
            Note::new(None, vec![Block::section("foo", vec![])]),
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
                vec![Block::section("foo", vec![Block::section("bar", vec![])])]
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
                vec![
                    Block::section("aaa", vec![Block::section("bbb", vec![])]),
                    Block::section("ccc", vec![Block::section("ddd", vec![])])
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
                vec![
                    Block::anonymous_section(vec![Block::section("foo", vec![])]),
                    Block::section("bar", vec![])
                ],
            ),
        );
        Ok(())
    }

    #[test]
    fn block_quote_paragraph_to_note() -> Result<()> {
        assert_eq!(
            NoteParser::parse(&root(vec![
                block_quote(vec![paragraph(vec![text("foo")])]),
                block_quote(vec![paragraph(vec![text("[!note] title\nfoo")])]),
                block_quote(vec![paragraph(vec![text("[!note]\nfoo")])]),
                block_quote(vec![paragraph(vec![text("[!summary]\nfoo")])]),
                block_quote(vec![paragraph(vec![text("[!quote]\nfoo")])]),
                block_quote(vec![paragraph(vec![text("[!question]\nfoo")])]),
                block_quote(vec![paragraph(vec![text("[!toc]\n- foo")])]),
                block_quote(vec![paragraph(vec![text("[!todo]\nfoo")])]),
            ]))?,
            Note::new(
                None,
                vec![Block::anonymous_section(vec![
                    Block::card(NoteKind::Note, None, vec![Block::text("foo")]),
                    Block::card(
                        NoteKind::Note,
                        Some("title".into()),
                        vec![Block::text("foo")]
                    ),
                    Block::card(NoteKind::Note, None, vec![Block::text("foo")]),
                    Block::card(NoteKind::Summary, None, vec![Block::text("foo")]),
                    Block::card(NoteKind::Quote, None, vec![Block::text("foo")]),
                    Block::card(NoteKind::Question, None, vec![Block::text("foo")]),
                    Block::toc(vec![FlattenNode(1, String::from("foo"))]),
                    Block::card(NoteKind::Todo, None, vec![Block::text("foo")]),
                ])],
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
                vec![]
            )
        );
        Ok(())
    }

    #[test]
    fn root_to_note() -> Result<()> {
        assert_eq!(
            NoteParser::parse(&root(vec![text("foo")]))?,
            Note::new(
                None,
                vec![Block::anonymous_section(vec![Block::text("foo")])]
            )
        );
        Ok(())
    }
}
