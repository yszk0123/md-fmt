use anyhow::Result;
use markdown::mdast as m;

use crate::md::builder::*;
use crate::note::metadata::Metadata;
use crate::note::model::*;

pub fn from_note(note: &Note) -> Result<m::Node> {
    let mut children = vec![];

    children.extend(from_yaml(&note.metadata)?);
    children.extend(from_block(&note.head, 2)?);
    children.extend(from_block(&note.body, 1)?);

    Ok(root(children))
}

fn from_yaml(metadata: &Option<Metadata>) -> Result<Vec<m::Node>> {
    if let Some(metadata) = metadata {
        Ok(vec![yaml(metadata.to_md()?)])
    } else {
        Ok(vec![])
    }
}

fn from_block(block: &Block, depth: u8) -> Result<Vec<m::Node>> {
    let mut nodes: Vec<m::Node> = vec![];

    if let Some(title) = &block.title {
        nodes.push(heading(depth, vec![text(title)]));
    }

    for node in &block.children {
        nodes.extend(from_node(node, depth + 1)?);
    }

    Ok(nodes)
}

fn from_node(node: &NoteNode, depth: u8) -> Result<Vec<m::Node>> {
    match node {
        NoteNode::Empty => Ok(vec![]),

        NoteNode::Block(block) => from_block(block, depth),

        NoteNode::Card(Card { kind, children }) => {
            let children = concat(children.iter().map(|v| from_node(v, depth + 1)).collect())?;
            let children = Some(text(format!("[!{kind}]")))
                .into_iter()
                .chain(children)
                .collect();
            Ok(vec![block_quote(children)])
        },

        NoteNode::Node(node) => Ok(vec![node.clone()]),
    }
}

fn concat<T>(nodes_list: Vec<Result<Vec<T>>>) -> Result<Vec<T>> {
    let mut result: Vec<T> = vec![];
    for nodes in nodes_list {
        result.extend(nodes?);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata_to_node() -> Result<()> {
        assert_eq!(
            from_note(&Note::new(
                Some(Metadata {
                    title: Some("foo".into()),
                    ..Default::default()
                }),
                Block::default(),
                Block::default(),
            ))?,
            root(vec![yaml("title: foo\n")])
        );
        Ok(())
    }

    #[test]
    fn head_text_to_node() -> Result<()> {
        assert_eq!(
            from_note(&Note::new(
                None,
                Block::new(None, vec![NoteNode::Node(text("foo"))]),
                Block::default(),
            ))?,
            root(vec![text("foo")]),
        );
        Ok(())
    }

    #[test]
    fn head_heading_to_node() -> Result<()> {
        assert_eq!(
            from_note(&Note::new(
                None,
                Block::new(Some("heading".into()), vec![NoteNode::Node(text("foo"))]),
                Block::default(),
            ))?,
            root(vec![heading(2, vec![text("heading")]), text("foo")])
        );
        Ok(())
    }

    #[test]
    fn body_heading_to_node() -> Result<()> {
        assert_eq!(
            from_note(&Note::new(
                None,
                Block::default(),
                Block::new(Some("heading".into()), vec![NoteNode::Node(text("foo"))]),
            ))?,
            root(vec![heading(1, vec![text("heading")]), text("foo")])
        );
        Ok(())
    }

    #[test]
    fn body_text_to_node() -> Result<()> {
        assert_eq!(
            from_note(&Note::new(
                None,
                Block::default(),
                Block::new(None, vec![NoteNode::Node(text("foo"))]),
            ))?,
            root(vec![text("foo")])
        );
        Ok(())
    }
}
