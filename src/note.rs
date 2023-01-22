#![allow(unstable_name_collisions)]
pub mod metadata;
pub mod model;

use crate::note::model::{Block, Note};
use crate::Metadata;
use anyhow::Result;
use markdown::mdast::{BlockQuote, Break, Node, Root, Yaml};

struct Context {
    pub metadata: Option<Metadata>,
}

pub fn to_note(node: &Node) -> Result<Note> {
    let mut context = Context { metadata: None };
    let block = to_block(node, &mut context)?;
    Ok(Note {
        metadata: context.metadata,
        summary: None,
        blocks: vec![block],
    })
}

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

fn to_block(node: &Node, context: &mut Context) -> Result<Block> {
    match node {
        Node::Root(node) => {
            let children = node
                .children
                .iter()
                .map(|child| to_block(child, context))
                .collect::<Result<_>>()?;
            Ok(Block::Container { children })
        }

        Node::Yaml(node) => {
            context.metadata = Some(Metadata::from_str(&node.value)?);
            Ok(Block::Empty)
        }

        node => Ok(Block::Node(node.clone())),
    }
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
