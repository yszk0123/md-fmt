use crate::note::model::{Block, Note};
use anyhow::Result;
use markdown::mdast::{BlockQuote, Break, Node, Root, Yaml};

pub fn from_note(note: &Note) -> Result<Node> {
    let yaml = if let Some(m) = &note.metadata {
        Some(Node::Yaml(Yaml {
            value: m.to_md()?,
            position: None,
        }))
    } else {
        None
    };
    let mut children = note
        .blocks
        .iter()
        .map(from_block)
        .collect::<Result<Vec<Node>>>()?;
    children.splice(0..0, yaml);
    Ok(Node::Root(Root {
        children,
        position: None,
    }))
}

fn from_block(node: &Block) -> Result<Node> {
    match node {
        Block::Empty => Ok(Node::Break(Break { position: None })),

        Block::Container { children } => {
            let children = children.iter().map(from_block).collect::<Result<_>>()?;
            Ok(Node::Root(Root {
                children,
                position: None,
            }))
        }

        Block::Card { kind: _, block } => {
            let children = if let Some(block) = block {
                vec![from_block(block)?]
            } else {
                vec![]
            };
            Ok(Node::BlockQuote(BlockQuote {
                children,
                position: None,
            }))
        }

        Block::Node(node) => Ok(node.clone()),
    }
}
