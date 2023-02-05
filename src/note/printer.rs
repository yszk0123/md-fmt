use anyhow::Result;

use crate::note::builder::*;
use crate::note::metadata::Metadata;
use crate::note::model::*;
use crate::toc::FlattenNode;

const INDENT: &str = "    ";

pub struct NotePrinter {}

impl NotePrinter {
    pub fn print(note: &Note) -> Result<String> {
        let mut res = String::new();

        res.push_str(&from_yaml(&note.metadata)?);
        res.push_str(&from_head(&note.head)?);
        res.push_str(&from_body(&note.body)?);

        Ok(res.trim().to_string() + "\n")
    }
}

fn from_yaml(metadata: &Option<Metadata>) -> Result<String> {
    if let Some(metadata) = metadata {
        Ok(format!("---\n{}---\n", metadata.to_md()?))
    } else {
        Ok(String::from(""))
    }
}

fn from_head(children: &Vec<Block>) -> Result<String> {
    let mut res = String::new();

    for node in children {
        res.push_str(&from_node(node, 2)?);
        res.push('\n');
    }

    Ok(res)
}

fn from_body(children: &Vec<Section>) -> Result<String> {
    let mut res = String::new();

    for block in children {
        res.push_str(&heading(1, &block.title));
        res.push('\n');
        for node in &block.children {
            res.push_str(&from_node(node, 2)?);
        }
    }

    Ok(res)
}

fn from_node(node: &Block, depth: u8) -> Result<String> {
    match node {
        Block::Empty => Ok(String::from("")),

        Block::Section(Section { title, children }) => {
            let mut res = heading(depth, title);
            res.push('\n');
            for child in children {
                res.push_str(&from_node(child, depth + 1)?);
                res.push('\n');
            }
            Ok(res)
        },

        Block::Card(Card { kind, children }) => {
            let mut res = format!("[!{kind}]\n");
            for child in children {
                res.push_str(&from_node(child, depth + 1)?);
                res.push('\n');
            }
            Ok(block_quote(res))
        },

        Block::Text(node) => Ok(node.clone()),

        Block::Toc(nodes) => {
            let s = nodes
                .iter()
                .map(|FlattenNode(indent, value)| {
                    format!("> {}- {}", INDENT.repeat(indent - 1), value)
                })
                .collect::<Vec<String>>()
                .join("\n");

            Ok(format!("> [!toc]\n{s}\n"))
        },
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::note::metadata::Bookmark;

    #[test]
    fn convert_metadata() -> Result<()> {
        assert_eq!(
            NotePrinter::print(&Note::new(
                Some(Metadata {
                    title: Some("foo".into()),
                    ..Default::default()
                }),
                vec![],
                vec![]
            ))?,
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
            Some(Metadata {
                bookmark: Some(Bookmark::toc(indoc! {"
                    # aaa
                    ## bbb
                "})),
                ..Default::default()
            }),
            vec![],
            vec![],
        )
        .normalize()?;
        assert_eq!(
            NotePrinter::print(&note)?,
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
            NotePrinter::print(&Note::new(None, vec![Block::text("foo")], vec![]))?,
            indoc! {"
                foo
            "},
        );
        Ok(())
    }

    #[test]
    fn convert_head_heading() -> Result<()> {
        assert_eq!(
            NotePrinter::print(&Note::new(
                None,
                vec![Block::section("heading", vec![Block::text("foo")])],
                vec![]
            ))?,
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
            NotePrinter::print(&Note::new(
                None,
                vec![],
                vec![Section::new("heading", vec![Block::text("foo")])],
            ))?,
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
            NotePrinter::print(&Note::new(None, vec![], vec![Section::new("foo", vec![])]))?,
            indoc! {"
                # foo
            "}
        );
        Ok(())
    }

    #[test]
    fn convert_card() -> Result<()> {
        assert_eq!(
            NotePrinter::print(&Note::new(
                None,
                vec![
                    Block::card(NoteKind::default(), vec![]),
                    Block::card(NoteKind::Note, vec![]),
                    Block::card(NoteKind::Summary, vec![]),
                    Block::card(NoteKind::Quote, vec![]),
                    Block::card(NoteKind::Question, vec![]),
                ],
                vec![]
            ))?,
            indoc! {"
                > [!note]

                > [!note]

                > [!summary]

                > [!quote]

                > [!question]
            "}
        );
        Ok(())
    }
}
