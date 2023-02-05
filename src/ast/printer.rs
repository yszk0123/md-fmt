#![allow(unstable_name_collisions)]
use anyhow::{anyhow, Result};
use itertools::Itertools;
use markdown::mdast::{
    AlignKind, Delete, FootnoteDefinition, FootnoteReference, Html, Image, Link, Node, Table,
    TableCell, TableRow, Text,
};

const INDENT: &str = "    ";
const NEWLINE: &str = "\n";

pub struct AstPrinter {
    depth: u8,
    order: Option<usize>,
}

impl AstPrinter {
    pub fn print(node: &Node) -> Result<String> {
        let mut printer = AstPrinter {
            depth: 0,
            order: None,
        };
        printer.print_root(node)
    }

    fn print_root(&mut self, node: &Node) -> Result<String> {
        match node {
            // Parent
            Node::Root(node) => self.map_children(&node.children, Some(NEWLINE)),
            Node::BlockQuote(node) => {
                let s = self.map_children(&node.children, Some(NEWLINE))?;
                Ok(quote(s, self.depth))
            },
            Node::List(node) => node
                .children
                .iter()
                .enumerate()
                .map(|(i, child)| {
                    self.depth += 1;
                    let order = self.order;
                    self.order = if node.ordered { Some(i + 1) } else { None };
                    let res = self.print_root(child);
                    self.order = order;
                    self.depth -= 1;
                    res
                })
                .collect::<Result<String>>(),
            Node::Heading(node) => {
                let s = self.map_children(&node.children, None)?;
                let d: usize = node.depth.into();
                Ok(format!("{} {}", "#".repeat(d), s))
            },
            Node::Code(node) => {
                let lang = node.lang.clone().unwrap_or_else(|| "".into());
                let meta = node
                    .meta
                    .clone()
                    .map(|v| format!(" {v}"))
                    .unwrap_or_else(|| "".into());
                Ok(format!("```{}{}\n{}\n```\n", lang, meta, node.value))
            },
            Node::ListItem(node) => Ok(format!(
                "{}{} {}",
                indent(self.depth - 1),
                if let Some(order) = self.order {
                    format!("{order}.")
                } else {
                    "-".to_owned()
                },
                self.map_children(&node.children, None)?
            )),
            Node::Paragraph(node) => {
                let s = node
                    .children
                    .iter()
                    .map(|node| self.print_root(node).map(|v| v.trim().to_string()))
                    .enumerate()
                    .map(|(i, r)| {
                        if i > 0 {
                            r.map(|v| {
                                if v.is_empty() {
                                    String::from("")
                                } else {
                                    format!(" {v}")
                                }
                            })
                        } else {
                            r
                        }
                    })
                    .collect::<Result<String>>()?;
                Ok(format!("{}\n", s.trim()))
            },
            Node::Emphasis(node) => {
                let s = self.map_children(&node.children, None)?;
                Ok(format!("*{s}*"))
            },
            Node::Strong(node) => {
                let s = self.map_children(&node.children, None)?;
                Ok(format!("**{s}**"))
            },
            Node::Delete(Delete { children, .. }) => {
                let s = self.map_children(children, None)?;
                Ok(format!("~~{s}~~"))
            },
            Node::FootnoteDefinition(FootnoteDefinition {
                children,
                identifier,
                ..
            }) => {
                let s = self.map_children(children, None)?;
                Ok(format!("[^{identifier}]: {s}"))
            },
            Node::Break(_) => Ok("\n".into()),
            Node::Link(Link { children, url, .. }) => {
                let text = self.map_children(children, None)?;
                Ok(format!("[{text}]({url})"))
            },
            Node::Table(Table {
                align, children, ..
            }) => {
                let sep = align
                    .iter()
                    .map(|kind| match kind {
                        AlignKind::Left => ":--",
                        AlignKind::Right => "--:",
                        AlignKind::Center => ":-:",
                        AlignKind::None => "---",
                    })
                    .collect::<Vec<&str>>()
                    .join(" | ");
                let mut rows: Vec<String> = vec![];
                for (i, child) in children.iter().enumerate() {
                    if let Node::TableRow(TableRow { children, .. }) = child {
                        let s = children
                            .iter()
                            .map(|v| self.print_root(v))
                            .collect::<Result<Vec<String>>>()?
                            .join(" | ");
                        rows.push(format!("| {s} |"));
                    };
                    if i == 0 {
                        rows.push(format!("| {sep} |"));
                    }
                }
                Ok(rows.join("\n"))
            },
            Node::TableRow(TableRow { children, .. }) => {
                let s = children
                    .iter()
                    .map(|v| self.print_root(v))
                    .collect::<Result<Vec<String>>>()?
                    .join(" | ");
                Ok(format!("| {s} |\n"))
            },
            Node::TableCell(TableCell { children, .. }) => self.map_children(children, None),

            // Literal
            Node::Html(Html { value, .. }) => Ok(value.to_owned()),
            Node::Text(Text { value, .. }) => Ok(value.to_owned()),
            Node::Yaml(node) => Ok(format!("---\n{}---\n", node.value)),
            Node::FootnoteReference(FootnoteReference { identifier, .. }) => {
                Ok(format!("[^{identifier}]"))
            },
            Node::InlineCode(node) => Ok(format!("`{}`", node.value)),
            Node::ThematicBreak(_) => Ok("---\n".to_owned()),
            Node::Image(Image { alt, url, .. }) => Ok(format!("![{alt}]({url})")),

            node => Err(anyhow!("{:?} not supported syntax", node)),
        }
    }

    fn map_children(&mut self, children: &[Node], sep: Option<&str>) -> Result<String> {
        Ok(children
            .iter()
            .map(|node| self.print_root(node))
            .collect::<Result<Vec<String>>>()?
            .join(if let Some(sep) = sep { sep } else { "" }))
    }
}

fn quote(text: String, depth: u8) -> String {
    text.trim_end()
        .split('\n')
        .map(|line| {
            if line.is_empty() {
                format!("{}>", indent(depth))
            } else {
                format!("{}> {}", indent(depth), line)
            }
        })
        .collect_vec()
        .join("\n")
        + "\n"
}

fn indent(n: u8) -> String {
    INDENT.repeat(n.into())
}
