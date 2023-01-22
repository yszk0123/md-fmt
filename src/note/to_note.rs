use crate::note::model::{Block, Note};
use crate::Metadata;
use anyhow::Result;
use markdown::mdast::Node;

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

#[cfg(test)]
mod tests {
    use markdown::mdast::{Root, Text, Yaml};

    use super::*;

    #[test]
    fn text_to_note() -> Result<()> {
        assert_eq!(
            to_note(&Node::Text(Text {
                value: "foo".into(),
                position: None
            }))?,
            Note {
                metadata: None,
                summary: None,
                blocks: vec![Block::Node(Node::Text(Text {
                    value: "foo".into(),
                    position: None
                }))]
            },
        );
        Ok(())
    }

    #[test]
    fn yaml_to_note() -> Result<()> {
        assert_eq!(
            to_note(&Node::Yaml(Yaml {
                value: "title: foo".into(),
                position: None
            }))?,
            Note {
                metadata: Some(Metadata {
                    title: Some("foo".into()),
                    ..Default::default()
                }),
                summary: None,
                blocks: vec![Block::Empty]
            },
        );
        Ok(())
    }

    #[test]
    fn root_to_note() -> Result<()> {
        assert_eq!(
            to_note(&Node::Root(Root {
                children: vec![],
                position: None
            }))?,
            Note {
                metadata: None,
                summary: None,
                blocks: vec![Block::Container { children: vec![] }]
            },
        );
        Ok(())
    }
}
