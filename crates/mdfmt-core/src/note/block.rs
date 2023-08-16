use anyhow::Result;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

use super::{
    builder::*,
    card::Card,
    note_kind::NoteKind,
    section::Section,
    toc::FlattenNode,
    visitor::{Visitor, VisitorContext},
};
use crate::{chunk::Chunk, printer::Printer};

const INDENT: &str = "    ";

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize, Tsify)]
#[serde(tag = "type", content = "value")]
pub enum Block {
    #[default]
    Empty,
    AnonymousSection(Vec<Block>),
    Section(Section),
    Card(Card),
    Text(String),
    Single(String),
    Toc(Vec<FlattenNode>),
}

impl Block {
    pub fn anonymous_section(children: Vec<Block>) -> Self {
        Self::AnonymousSection(children)
    }

    pub fn section(title: &str, children: Vec<Block>) -> Self {
        Self::Section(Section {
            title: title.to_string(),
            children,
        })
    }

    pub fn toc(children: Vec<FlattenNode>) -> Self {
        Self::Toc(children)
    }

    pub fn card(kind: NoteKind, title: Option<String>, children: Vec<Block>) -> Self {
        Self::Card(Card {
            kind,
            title,
            children,
        })
    }

    pub fn single(text: &str) -> Self {
        Self::Single(text.to_string())
    }

    pub fn text(text: &str) -> Self {
        Self::Text(text.to_string())
    }
}

impl Visitor for Block {
    fn visit(&self, context: &mut VisitorContext) -> Result<()> {
        match self {
            Block::Empty => Ok(()),

            Block::AnonymousSection(children) => context.dive(|c| {
                for child in children {
                    child.visit(c)?;
                }
                Ok(())
            }),

            Block::Section(Section { title, children }) => {
                context.push(Chunk::Single(heading(context.get_depth(), title)));
                context.dive(|c| {
                    for child in children {
                        child.visit(c)?;
                    }
                    Ok(())
                })
            },

            Block::Card(Card {
                kind,
                title,
                children,
            }) => {
                let sub_context = &mut context.sub();
                // let sub_context = &mut context.sub();

                let kind_line = if let Some(title) = title {
                    format!("[!{kind}] {title}")
                } else {
                    format!("[!{kind}]")
                };
                sub_context.push(Chunk::Single(kind_line));

                sub_context.dive(|c| {
                    for child in children {
                        child.visit(c)?;
                    }
                    Ok(())
                })?;

                context.push(Chunk::Single(block_quote(&sub_context.print())));
                Ok(())
            },

            Block::Text(node) => {
                context.push(Chunk::Double(node.clone()));
                Ok(())
            },

            Block::Single(node) => {
                context.push(Chunk::Single(node.clone()));
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
                context.push(Chunk::Double(format!("> [!toc]\n{s}")));
                Ok(())
            },
        }
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
