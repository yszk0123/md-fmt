use anyhow::Result;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct Toc(Vec<Node>);

#[serde_as]
#[derive(PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    value: String,
    children: Vec<Node>,
}

#[derive(PartialEq, Debug)]
pub struct FlattenNode(usize, String);

impl Node {
    pub fn flatten(self) -> Vec<FlattenNode> {
        let mut values: Vec<FlattenNode> = vec![];
        self.flatten_inner(1, &mut values);
        values
    }

    fn flatten_inner(self, indent: usize, values: &mut Vec<FlattenNode>) {
        values.push(FlattenNode(indent, self.value));

        for child in self.children.into_iter() {
            child.flatten_inner(indent + 1, values);
        }
    }
}

enum Line {
    Unknown,
    Block(usize, String),
    List(usize, String),
}

impl Toc {
    pub fn new(children: Vec<Node>) -> Self {
        Self(children)
    }

    pub fn parse(s: String) -> Result<Self> {
        let lines: Vec<Line> = s.lines().map(Line::parse).collect();
        let (_, nodes) = Self::parse_line(0, 0, &lines, 0);
        Ok(Self(nodes))
    }

    fn parse_line(
        base: usize,
        parent: usize,
        lines: &Vec<Line>,
        index: usize,
    ) -> (usize, Vec<Node>) {
        let mut res: Vec<Node> = vec![];
        let mut i = index;

        while let Some(line) = lines.get(i) {
            match line {
                Line::Block(indent, value) => {
                    if *indent <= parent {
                        break;
                    }

                    let (next_index, children) = Self::parse_line(*indent, *indent, lines, i + 1);
                    i = next_index;
                    res.push(Node::new(value, children));
                },
                Line::List(indent, value) => {
                    let indent = base + *indent;
                    if indent <= parent {
                        break;
                    }

                    let (next_index, children) = Self::parse_line(parent, indent, lines, i + 1);
                    i = next_index;
                    res.push(Node::new(value, children));
                },
                Line::Unknown => {
                    i += 1;
                },
            }
        }

        (i, res)
    }
}

impl Line {
    pub fn parse(s: &str) -> Line {
        if let Some(line) = Self::parse_block(s) {
            return line;
        }
        if let Some(line) = Self::parse_bullet_list(s) {
            return line;
        }
        if let Some(line) = Self::parse_order_list(s) {
            return line;
        }
        Line::Unknown
    }

    fn parse_block(s: &str) -> Option<Line> {
        let (indent, skip, end) = s
            .chars()
            .fold_while((1, 0, ' '), |(indent, skip, prev), next| {
                match (prev, next) {
                    (' ', '#') => Continue((indent + 1, skip + 1, next)),
                    ('#', '#') => Continue((indent + 1, skip + 1, next)),
                    ('#', ' ') => Done((indent, skip + 1, '#')),
                    _ => Done((0, 0, ' ')),
                }
            })
            .into_inner();
        if end == '#' {
            Some(Line::Block(indent, s.chars().skip(skip).collect()))
        } else {
            None
        }
    }

    fn parse_bullet_list(s: &str) -> Option<Line> {
        let (indent, skip, end) = s
            .chars()
            .fold_while((1, 0, ' '), |(indent, skip, prev), next| {
                match (prev, next) {
                    (' ', ' ') => Continue((indent + 1, skip + 1, next)),
                    (' ', '-') => Continue((indent, skip + 1, next)),
                    ('-', ' ') => Done((indent, skip + 1, '-')),
                    _ => Done((0, 0, ' ')),
                }
            })
            .into_inner();
        if end == '-' {
            Some(Line::List(indent, s.chars().skip(skip).collect()))
        } else {
            None
        }
    }

    fn parse_order_list(s: &str) -> Option<Line> {
        let (indent, skip, end) = s
            .chars()
            .fold_while((1, 0, ' '), |(indent, skip, prev), next| {
                match (prev, next) {
                    (' ', ' ') => Continue((indent + 1, skip + 1, next)),
                    (' ', '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9') => {
                        Continue((indent, skip + 1, 'n'))
                    },
                    ('n', '.') => Continue((indent, skip + 1, '.')),
                    ('.', ' ') => Done((indent, skip + 1, '.')),
                    _ => Done((0, 0, ' ')),
                }
            })
            .into_inner();
        if end == '.' {
            Some(Line::List(indent, s.chars().skip(skip).collect()))
        } else {
            None
        }
    }
}

impl Node {
    pub fn new(value: impl ToString, children: Vec<Node>) -> Self {
        Self {
            value: value.to_string(),
            children,
        }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn sharp_style_heading() -> Result<()> {
        let toc = Toc::parse(
            indoc! {"
                # aaa
                ## bbb
                ### ccc
                # ddd
            "}
            .to_string(),
        )?;
        assert_eq!(
            toc,
            Toc::new(vec![
                Node::new(
                    "aaa",
                    vec![Node::new("bbb", vec![Node::new("ccc", vec![])]),]
                ),
                Node::new("ddd", vec![])
            ])
        );
        Ok(())
    }

    #[test]
    fn bullet_style_heading() -> Result<()> {
        let toc = Toc::parse(
            indoc! {"
                - aaa
                  - bbb
                    - ccc
                - ddd
            "}
            .to_string(),
        )?;
        assert_eq!(
            toc,
            Toc::new(vec![
                Node::new(
                    "aaa",
                    vec![Node::new("bbb", vec![Node::new("ccc", vec![])]),]
                ),
                Node::new("ddd", vec![])
            ])
        );
        Ok(())
    }

    #[test]
    fn number_list_style_heading() -> Result<()> {
        let toc = Toc::parse(
            indoc! {"
                1. aaa
                  2. bbb
                    3. ccc
                1. ddd
            "}
            .to_string(),
        )?;
        assert_eq!(
            toc,
            Toc::new(vec![
                Node::new(
                    "aaa",
                    vec![Node::new("bbb", vec![Node::new("ccc", vec![])]),]
                ),
                Node::new("ddd", vec![])
            ])
        );
        Ok(())
    }

    #[test]
    fn mixed_list_style_heading() -> Result<()> {
        let toc = Toc::parse(
            indoc! {"
                # aaa
                ## bbb
                - ccc
                  - ddd
                # eee
            "}
            .to_string(),
        )?;
        assert_eq!(
            toc,
            Toc::new(vec![
                Node::new(
                    "aaa",
                    vec![Node::new(
                        "bbb",
                        vec![Node::new("ccc", vec![Node::new("ddd", vec![])])]
                    ),]
                ),
                Node::new("eee", vec![])
            ])
        );
        Ok(())
    }

    #[test]
    fn node_flatten() -> Result<()> {
        let node = Node::new(
            "aaa",
            vec![
                Node::new("bbb", vec![Node::new("ccc", vec![])]),
                Node::new("ddd", vec![]),
            ],
        );
        assert_eq!(
            node.flatten(),
            vec![
                FlattenNode(1, String::from("aaa")),
                FlattenNode(2, String::from("bbb")),
                FlattenNode(3, String::from("ccc")),
                FlattenNode(2, String::from("ddd")),
            ]
        );
        Ok(())
    }
}
