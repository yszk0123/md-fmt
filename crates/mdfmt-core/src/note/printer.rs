use anyhow::Result;

use super::visitor::{Visitor, VisitorContext};
use super::{Block, Card, FlattenNode, Note, Section};
use crate::chunk::Chunk;
use crate::note::builder::*;
use crate::note::metadata::Metadata;
use crate::printer::Printer;

const INDENT: &str = "    ";

impl Printer for Note {
    type Options = ();

    fn print(&self, _options: Self::Options) -> Result<String> {
        let context = &mut VisitorContext::new(1);

        if let Some(metadata) = &self.metadata {
            metadata.visit(context)?;
        }
        for block in &self.body {
            block.visit(context)?;
        }

        Ok(context.print() + "\n")
    }
}

pub struct BlockPrinterOptions {
    pub depth: u8,
}

impl Printer for Block {
    type Options = BlockPrinterOptions;

    fn print(&self, options: Self::Options) -> Result<String> {
        let context = &mut VisitorContext::new(options.depth);

        self.visit(context)?;

        Ok(context.print())
    }
}

impl Visitor for Metadata {
    fn visit(&self, context: &mut VisitorContext) -> Result<()> {
        context.push(Chunk::Single(format!("---\n{}---", self.to_md()?)));
        Ok(())
    }
}

impl Visitor for Block {
    fn visit(&self, context: &mut VisitorContext) -> Result<()> {
        match self {
            Block::Empty => Ok(()),

            Block::AnonymousSection(children) => context.dive(|c| {
                for child in children {
                    child.visit(c)?;
                }
                Ok(())
            }),

            Block::Section(Section { title, children }) => {
                context.push(Chunk::Single(heading(context.get_depth(), title)));
                context.dive(|c| {
                    for child in children {
                        child.visit(c)?;
                    }
                    Ok(())
                })
            },

            Block::Card(Card {
                kind,
                title,
                children,
            }) => {
                let sub_context = &mut context.sub();
                // let sub_context = &mut context.sub();

                let kind_line = if let Some(title) = title {
                    format!("[!{kind}] {title}")
                } else {
                    format!("[!{kind}]")
                };
                sub_context.push(Chunk::Single(kind_line));

                sub_context.dive(|c| {
                    for child in children {
                        child.visit(c)?;
                    }
                    Ok(())
                })?;

                context.push(Chunk::Single(block_quote(&sub_context.print())));
                Ok(())
            },

            Block::Text(node) => {
                context.push(Chunk::Double(node.clone()));
                Ok(())
            },

            Block::Single(node) => {
                context.push(Chunk::Single(node.clone()));
                Ok(())
            },

            Block::Toc(nodes) => {
                let s = nodes
                    .iter()
                    .map(|FlattenNode(indent, value)| {
                        format!("> {}- {}", INDENT.repeat(indent - 1), value)
                    })
                    .collect::<Vec<String>>()
                    .join("\n");
                context.push(Chunk::Double(format!("> [!toc]\n{s}")));
                Ok(())
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::super::NoteKind;
    use super::*;
    use crate::note::metadata::{Bookmark, Meta};

    #[test]
    fn convert_metadata() -> Result<()> {
        assert_eq!(
            Note::new(
                Some(Metadata::Meta(Meta {
                    title: Some("foo".into()),
                    ..Default::default()
                })),
                vec![]
            )
            .print(())?,
            indoc! {"
                ---
                title: foo
                ---
            "}
        );
        Ok(())
    }

    #[test]
    fn convert_toc() -> Result<()> {
        let note = Note::new(
            Some(Metadata::Meta(Meta {
                bookmark: Some(Bookmark::toc(indoc! {"
                    # aaa
                    ## bbb
                "})),
                ..Default::default()
            })),
            vec![],
        )
        .normalize()?;
        assert_eq!(
            &note.print(())?,
            indoc! {"
                > [!toc]
                > - aaa
                >     - bbb
            "}
        );
        Ok(())
    }

    #[test]
    fn convert_head_text() -> Result<()> {
        assert_eq!(
            Note::new(None, vec![Block::text("foo")]).print(())?,
            indoc! {"
                foo
            "},
        );
        Ok(())
    }

    #[test]
    fn convert_head_heading() -> Result<()> {
        assert_eq!(
            Note::new(
                None,
                vec![Block::anonymous_section(vec![Block::section(
                    "heading",
                    vec![Block::text("foo")]
                )])],
            )
            .print(())?,
            indoc! {"
                ## heading
                foo
            "}
        );
        Ok(())
    }

    #[test]
    fn convert_body_heading() -> Result<()> {
        assert_eq!(
            Note::new(
                None,
                vec![Block::section("heading", vec![Block::text("foo")])],
            )
            .print(())?,
            indoc! {"
                # heading
                foo
            "}
        );
        Ok(())
    }

    #[test]
    fn convert_body_text() -> Result<()> {
        assert_eq!(
            Note::new(None, vec![Block::section("foo", vec![])]).print(())?,
            indoc! {"
                # foo
            "}
        );
        Ok(())
    }

    #[test]
    fn convert_card() -> Result<()> {
        assert_eq!(
            Note::new(
                None,
                vec![
                    Block::card(NoteKind::default(), None, vec![]),
                    Block::card(NoteKind::Note, Some("title".into()), vec![]),
                    Block::card(NoteKind::Note, None, vec![]),
                    Block::card(NoteKind::Summary, None, vec![]),
                    Block::card(NoteKind::Quote, None, vec![]),
                    Block::card(NoteKind::Question, None, vec![]),
                ],
            )
            .print(())?,
            indoc! {"
                > [!note]

                > [!note] title

                > [!note]

                > [!summary]

                > [!quote]

                > [!question]
            "}
        );
        Ok(())
    }
}
