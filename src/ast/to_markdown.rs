#![allow(unstable_name_collisions)]
use anyhow::{anyhow, Result};
use itertools::Itertools;
use markdown::mdast::Node;

const INDENT: &str = "    ";
const NEWLINE: &str = "\n";

struct Context {
    pub depth: usize,
    pub order: Option<usize>,
}

pub fn to_markdown(node: &Node) -> Result<String> {
    let mut context = Context {
        depth: 0,
        order: None,
    };
    to_md(node, &mut context)
}

fn to_md(node: &Node, context: &mut Context) -> Result<String> {
    match node {
        // Parent
        Node::Root(node) => map_children(&node.children, context, Some(NEWLINE)),
        Node::BlockQuote(node) => {
            let s = map_children(&node.children, context, Some(NEWLINE))?;
            Ok(quote(s, context))
        },
        Node::List(node) => node
            .children
            .iter()
            .enumerate()
            .map(|(i, child)| {
                context.depth += 1;
                let order = context.order;
                context.order = if node.ordered { Some(i + 1) } else { None };
                let res = to_md(child, context);
                context.order = order;
                context.depth -= 1;
                res
            })
            .collect::<Result<String>>(),
        Node::Heading(node) => {
            let s = map_children(&node.children, context, None)?;
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
            indent(context.depth - 1),
            if let Some(order) = context.order {
                format!("{order}.")
            } else {
                "-".to_owned()
            },
            map_children(&node.children, context, None)?
        )),
        Node::Paragraph(node) => {
            let s = node
                .children
                .iter()
                .map(|node| to_md(node, context).map(|v| v.trim().to_string()))
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
            let s = map_children(&node.children, context, None)?;
            Ok(format!("*{s}*"))
        },
        Node::Strong(node) => {
            let s = map_children(&node.children, context, None)?;
            Ok(format!("**{s}**"))
        },
        Node::Break(_) => Ok("\n".into()),

        // Literal
        Node::Text(text) => Ok(text.value.to_owned()),
        Node::Yaml(node) => Ok(format!("---\n{}---\n", node.value)),
        Node::InlineCode(node) => Ok(format!("`{}`", node.value)),
        Node::ThematicBreak(_) => Ok("---\n".to_owned()),

        node => Err(anyhow!("{:?} not supported syntax", node)),
    }
}

fn map_children(children: &[Node], context: &mut Context, sep: Option<&str>) -> Result<String> {
    Ok(children
        .iter()
        .map(|node| to_md(node, context))
        .collect::<Result<Vec<String>>>()?
        .join(if let Some(sep) = sep { sep } else { "" }))
}

fn quote(text: String, context: &mut Context) -> String {
    text.trim_end()
        .split('\n')
        .map(|line| {
            if line.is_empty() {
                format!("{}>", indent(context.depth))
            } else {
                format!("{}> {}", indent(context.depth), line)
            }
        })
        .collect_vec()
        .join("\n")
        + "\n"
}

fn indent(n: usize) -> String {
    INDENT.repeat(n)
}
