use anyhow::Result;
use markdown::mdast as m;

use crate::ast::builder::*;
use crate::note::metadata::Metadata;
use crate::note::model::*;

pub fn to_ast(note: &Note) -> Result<m::Node> {
    let mut children = vec![];

    children.extend(from_yaml(&note.metadata)?);
    children.extend(from_head(&note.head)?);
    children.extend(from_body(&note.body)?);

    Ok(root(children))
}

fn from_yaml(metadata: &Option<Metadata>) -> Result<Vec<m::Node>> {
    if let Some(metadata) = metadata {
        Ok(vec![yaml(metadata.to_md()?)])
    } else {
        Ok(vec![])
    }
}

fn from_head(children: &Vec<Block>) -> Result<Vec<m::Node>> {
    let mut nodes: Vec<m::Node> = vec![];

    for node in children {
        nodes.extend(from_node(node, 2)?);
    }

    Ok(nodes)
}

fn from_body(children: &Vec<Section>) -> Result<Vec<m::Node>> {
    let mut nodes: Vec<m::Node> = vec![];

    for block in children {
        nodes.push(heading(1, vec![text(&block.title)]));
        for node in &block.children {
            nodes.extend(from_node(node, 2)?);
        }
    }

    Ok(nodes)
}

fn from_node(node: &Block, depth: u8) -> Result<Vec<m::Node>> {
    match node {
        Block::Empty => Ok(vec![]),

        Block::Section(Section { title, children }) => {
            let mut res = vec![heading(depth, vec![text(title)])];
            for child in children {
                res.extend(from_node(child, depth + 1)?);
            }
            Ok(res)
        },

        Block::Card(Card { kind, children }) => {
            let mut res = vec![text(format!("[!{kind}]"))];
            for child in children {
                res.extend(from_node(child, depth + 1)?);
            }
            Ok(vec![block_quote(res)])
        },

        Block::Node(node) => Ok(vec![node.clone()]),
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn metadata_to_node() -> Result<()> {
        assert_eq!(
            to_ast(&Note::new(
                Some(Metadata {
                    title: Some("foo".into()),
                    ..Default::default()
                }),
                vec![],
                vec![]
            ))?,
            root(vec![yaml("title: foo\n")])
        );
        Ok(())
    }

    #[test]
    fn head_text_to_node() -> Result<()> {
        assert_eq!(
            to_ast(&Note::new(None, vec![Block::Node(text("foo"))], vec![]))?,
            root(vec![text("foo")]),
        );
        Ok(())
    }

    #[test]
    fn head_heading_to_node() -> Result<()> {
        assert_eq!(
            to_ast(&Note::new(
                None,
                vec![Block::section("heading", vec![Block::Node(text("foo"))])],
                vec![]
            ))?,
            root(vec![heading(2, vec![text("heading")]), text("foo")])
        );
        Ok(())
    }

    #[test]
    fn body_heading_to_node() -> Result<()> {
        assert_eq!(
            to_ast(&Note::new(
                None,
                vec![],
                vec![Section::new("heading", vec![Block::Node(text("foo"))])],
            ))?,
            root(vec![heading(1, vec![text("heading")]), text("foo")])
        );
        Ok(())
    }

    #[test]
    fn body_text_to_node() -> Result<()> {
        assert_eq!(
            to_ast(&Note::new(None, vec![], vec![Section::new("foo", vec![])]))?,
            root(vec![heading(1, vec![text("foo")])])
        );
        Ok(())
    }
}
