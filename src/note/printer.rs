use anyhow::Result;

use crate::note::builder::*;
use crate::note::metadata::Metadata;
use crate::note::model::*;

pub struct Printer {}

impl Printer {
    pub fn to_markdown(note: &Note) -> Result<String> {
        let mut children = String::new();

        children.push_str(&from_yaml(&note.metadata)?);
        children.push_str(&from_head(&note.head)?);
        children.push_str(&from_body(&note.body)?);

        Ok(children.trim().to_string() + "\n")
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
    let mut nodes = String::new();

    for node in children {
        nodes.push_str(&from_node(node, 2)?);
        nodes.push('\n');
    }

    Ok(nodes)
}

fn from_body(children: &Vec<Section>) -> Result<String> {
    let mut nodes = String::new();

    for block in children {
        nodes.push_str(&heading(1, &block.title));
        nodes.push('\n');
        for node in &block.children {
            nodes.push_str(&from_node(node, 2)?);
        }
    }

    Ok(nodes)
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
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn convert_metadata() -> Result<()> {
        assert_eq!(
            Printer::to_markdown(&Note::new(
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
    fn convert_head_text() -> Result<()> {
        assert_eq!(
            Printer::to_markdown(&Note::new(None, vec![Block::text("foo")], vec![]))?,
            indoc! {"
                foo
            "},
        );
        Ok(())
    }

    #[test]
    fn convert_head_heading() -> Result<()> {
        assert_eq!(
            Printer::to_markdown(&Note::new(
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
            Printer::to_markdown(&Note::new(
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
            Printer::to_markdown(&Note::new(None, vec![], vec![Section::new("foo", vec![])]))?,
            indoc! {"
                # foo
            "}
        );
        Ok(())
    }

    #[test]
    fn convert_card() -> Result<()> {
        assert_eq!(
            Printer::to_markdown(&Note::new(
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
