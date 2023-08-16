use anyhow::Result;

use super::visitor::{Visitor, VisitorContext};
use super::{Block, Note};
use crate::printer::Printer;

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

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::super::{
        metadata::{Bookmark, Meta, Metadata},
        NoteKind,
    };
    use super::*;

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
