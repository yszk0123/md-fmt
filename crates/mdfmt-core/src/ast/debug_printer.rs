use itertools::Itertools;
use markdown::mdast::*;

use crate::debug_printer::DebugPrinter;

impl DebugPrinter for Node {
    type Options = ();

    fn debug_print(&self, _options: Self::Options) -> String {
        pretty_inner(self, 0)
    }
}

fn pretty_inner(node: &Node, depth: usize) -> String {
    match node {
        // Parents.
        Node::Root(x) => children_to_string("Root", &x.children, depth),
        Node::BlockQuote(x) => children_to_string("BlockQuote", &x.children, depth),
        Node::FootnoteDefinition(x) => children_to_string("FootnoteDefinition", &x.children, depth),
        Node::MdxJsxFlowElement(x) => children_to_string("MdxJsxFlowElement", &x.children, depth),
        Node::List(x) => children_to_string("List", &x.children, depth),
        Node::Delete(x) => children_to_string("Delete", &x.children, depth),
        Node::Emphasis(x) => children_to_string("Emphasis", &x.children, depth),
        Node::MdxJsxTextElement(x) => children_to_string("MdxJsxTextElement", &x.children, depth),
        Node::Link(x) => children_to_string("Link", &x.children, depth),
        Node::LinkReference(x) => children_to_string("LinkReference", &x.children, depth),
        Node::Strong(x) => children_to_string("Strong", &x.children, depth),
        Node::Heading(x) => children_to_string("Heading", &x.children, depth),
        Node::Table(x) => children_to_string("Table", &x.children, depth),
        Node::TableRow(x) => children_to_string("TableRow", &x.children, depth),
        Node::TableCell(x) => children_to_string("TableCell", &x.children, depth),
        Node::ListItem(x) => children_to_string("ListItem", &x.children, depth),
        Node::Paragraph(x) => children_to_string("Paragraph", &x.children, depth),

        // Literals.
        Node::MdxjsEsm(x) => literal_to_string("MdxjsEsm", &x.value, depth),
        Node::Toml(x) => literal_to_string("Toml", &x.value, depth),
        Node::Yaml(x) => literal_to_string("Yaml", &x.value, depth),
        Node::InlineCode(x) => literal_to_string("InlineCode", &x.value, depth),
        Node::InlineMath(x) => literal_to_string("InlineMath", &x.value, depth),
        Node::MdxTextExpression(x) => literal_to_string("MdxTextExpression", &x.value, depth),
        Node::Html(x) => literal_to_string("Html", &x.value, depth),
        Node::Text(x) => literal_to_string("Text", &x.value, depth),
        Node::Code(x) => literal_to_string("Code", &x.value, depth),
        Node::Math(x) => literal_to_string("Math", &x.value, depth),
        Node::MdxFlowExpression(x) => literal_to_string("MdxFlowExpression", &x.value, depth),

        // Voids.
        Node::Break(_)
        | Node::FootnoteReference(_)
        | Node::Image(_)
        | Node::ImageReference(_)
        | Node::ThematicBreak(_)
        | Node::Definition(_) => String::new(),
    }
}

fn children_to_string(name: &str, children: &[Node], depth: usize) -> String {
    format!(
        "{}[{}]\n{}",
        "  ".repeat(depth),
        name,
        children
            .iter()
            .map(|v| pretty_inner(v, depth + 1))
            .collect_vec()
            .join("\n")
    )
}

fn literal_to_string(name: &str, value: &String, depth: usize) -> String {
    format!("{}[{}]{}", "  ".repeat(depth), name, value)
}
