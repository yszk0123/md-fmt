use markdown::mdast::*;

pub fn root(children: Vec<Node>) -> Node {
    Node::Root(Root {
        children,
        position: None,
    })
}

pub fn yaml(value: &str) -> Node {
    Node::Yaml(Yaml {
        value: value.to_string(),
        position: None,
    })
}

pub fn heading(depth: u8, children: Vec<Node>) -> Node {
    Node::Heading(Heading {
        depth,
        children,
        position: None,
    })
}

pub fn block_quote(children: Vec<Node>) -> Node {
    Node::BlockQuote(BlockQuote {
        children,
        position: None,
    })
}

pub fn text(value: &str) -> Node {
    Node::Text(Text {
        value: value.to_string(),
        position: None,
    })
}

pub fn paragraph(children: Vec<Node>) -> Node {
    Node::Paragraph(Paragraph {
        children,
        position: None,
    })
}
