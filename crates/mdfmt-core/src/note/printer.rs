use anyhow::Result;

use super::{Block, Card, FlattenNode, Note, Section};
use crate::chunk::{Chunk, ChunkPrinter};
use crate::note::builder::*;
use crate::note::metadata::Metadata;
use crate::printer::Printer;

const INDENT: &str = "    ";

impl Printer for Note {
    type Options = ();

    fn print(&self, _options: Self::Options) -> Result<String> {
        let mut chunks = ChunkPrinter::new();

        from_yaml(&self.metadata, &mut chunks)?;
        from_body(&self.body, &mut chunks)?;

        Ok(chunks.print() + "\n")
    }
}

pub struct BlockPrinterOptions {
    pub depth: u8,
}

impl Printer for Block {
    type Options = BlockPrinterOptions;

    fn print(&self, options: Self::Options) -> Result<String> {
        let mut chunks = ChunkPrinter::new();

        from_block(self, options.depth, &mut chunks)?;

        Ok(chunks.print())
    }
}

fn from_yaml(metadata: &Option<Metadata>, chunks: &mut ChunkPrinter) -> Result<()> {
    if let Some(metadata) = metadata {
        chunks.push(Chunk::Single(format!("---\n{}---", metadata.to_md()?)));
    }
    Ok(())
}

fn from_body(blocks: &Vec<Block>, chunks: &mut ChunkPrinter) -> Result<()> {
    for block in blocks {
        from_block(block, 1, chunks)?;
    }
    Ok(())
}

fn from_block(block: &Block, depth: u8, chunks: &mut ChunkPrinter) -> Result<()> {
    match block {
        Block::Empty => Ok(()),

        Block::AnonymousSection(children) => {
            for child in children {
                from_block(child, depth + 1, chunks)?;
            }
            Ok(())
        },

        Block::Section(Section { title, children }) => {
            chunks.push(Chunk::Single(heading(depth, title)));
            for child in children {
                from_block(child, depth + 1, chunks)?;
            }
            Ok(())
        },

        Block::Card(Card {
            kind,
            title,
            children,
        }) => {
            let mut subchunks = ChunkPrinter::new();

            let kind_line = if let Some(title) = title {
                format!("[!{kind}] {title}")
            } else {
                format!("[!{kind}]")
            };
            subchunks.push(Chunk::Single(kind_line));

            for child in children {
                from_block(child, depth + 1, &mut subchunks)?;
            }
            chunks.push(Chunk::Single(block_quote(&subchunks.print())));
            Ok(())
        },

        Block::Text(node) => {
            chunks.push(Chunk::Double(node.clone()));
            Ok(())
        },

        Block::Single(node) => {
            chunks.push(Chunk::Single(node.clone()));
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
            chunks.push(Chunk::Double(format!("> [!toc]\n{s}")));
            Ok(())
        },
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
