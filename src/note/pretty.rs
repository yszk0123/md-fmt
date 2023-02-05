use super::model::{Block, Note};
use crate::toc::FlattenNode;

const INDENT: &str = "  ";

pub fn pretty(note: &Note) -> String {
    let mut s = String::new();

    s.push_str(&format!("{:?}\n---\n", note.metadata));
    for block in &note.head {
        s.push_str(&pretty_inner(block, 0));
    }
    for section in note.body.iter() {
        s.push_str(&pretty_inner(&Block::Section(section.clone()), 0));
    }

    s
}

fn pretty_inner(node: &Block, depth: usize) -> String {
    match node {
        Block::Empty => String::new(),

        Block::Section(x) => {
            let children = children_to_string(&x.children, depth);
            format!("{}[Section] {}\n{children}\n", indent(depth), x.title)
        },

        Block::Card(x) => format!(
            "{}[card] {}\n{}\n",
            indent(depth),
            &x.kind.to_string(),
            children_to_string(&x.children, depth)
        ),

        Block::Toc(x) => format!(
            "{}[toc]\n{}",
            depth,
            &x.iter()
                .map(|FlattenNode(i, text)| line(depth + i, text))
                .collect::<Vec<String>>()
                .join("\n"),
        ),

        // Literals.
        Block::Text(x) => format!("{}\n", literal_to_string("Text", x, depth)),
    }
}

fn children_to_string(children: &[Block], depth: usize) -> String {
    line(
        depth,
        &children
            .iter()
            .map(|v| pretty_inner(v, depth + 1))
            .collect::<Vec<String>>()
            .join("\n"),
    )
}

fn literal_to_string(name: &str, value: &String, depth: usize) -> String {
    format!("{}[{}] {}", indent(depth), name, value)
}

fn line(n: usize, s: &String) -> String {
    format!("{}{s}", indent(n))
}

fn indent(n: usize) -> String {
    INDENT.repeat(n)
}
