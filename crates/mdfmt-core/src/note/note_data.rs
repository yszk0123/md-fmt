use anyhow::Result;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

use super::{
    block::Block,
    metadata::{Meta, Metadata},
    visitor::{Visitor, VisitorContext},
};
use crate::{debug_printer::DebugPrinter, printer::Printer};

#[derive(PartialEq, Debug, Default, Serialize, Deserialize, Tsify)]
pub struct Note {
    pub metadata: Option<Metadata>,
    pub body: Vec<Block>,
}

impl Note {
    pub fn new(metadata: Option<Metadata>, body: Vec<Block>) -> Self {
        Self { metadata, body }
    }

    pub fn normalize(self) -> Result<Self> {
        let mut body = self.get_toc()?;
        body.extend(self.body);

        Ok(Self {
            metadata: self.metadata.and_then(Metadata::normalize),
            body,
        })
    }

    fn get_toc(&self) -> Result<Vec<Block>> {
        match &self.metadata {
            Some(Metadata::Meta(Meta {
                bookmark: Some(b), ..
            })) => {
                if let Some(toc) = b.parse_toc()? {
                    Ok(vec![Block::toc(toc.flatten_ref())])
                } else {
                    Ok(vec![])
                }
            },
            Some(Metadata::Meta(m)) => {
                if let Some(toc) = m.parse_toc()? {
                    Ok(vec![Block::toc(toc.flatten_ref())])
                } else {
                    Ok(vec![])
                }
            },
            _ => Ok(vec![]),
        }
    }
}

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

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::super::{
        block::Block,
        metadata::{Bookmark, Meta, Metadata},
        note_kind::NoteKind,
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
                    Block::card(NoteKind::Todo, None, vec![]),
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

                > [!todo]
            "}
        );
        Ok(())
    }
}

impl DebugPrinter for Note {
    type Options = ();

    fn debug_print(&self, _options: Self::Options) -> String {
        let mut s = String::new();

        s.push_str(&format!("{:?}\n---\n", self.metadata));
        for block in self.body.iter() {
            s.push_str(&block.debug_print(0));
        }

        s
    }
}
