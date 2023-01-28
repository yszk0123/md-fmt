// pub fn yaml(value: impl ToString) -> String {
//     Node::Yaml(Yaml {
//         value: value.to_string(),
//         position: None,
//     })
// }

pub fn heading(depth: u8, value: &String) -> String {
    format!("{} {}", "#".repeat(depth.into()), value)
}

pub fn block_quote(text: String) -> String {
    text.trim_end()
        .lines()
        .map(|line| {
            if line.is_empty() {
                ">".to_string()
            } else {
                format!("> {line}")
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
        + "\n"
}

// pub fn block_quote(children: Vec<Node>) -> String {
//     Node::BlockQuote(BlockQuote {
//         children,
//         position: None,
//     })
// }

// pub fn text(value: impl ToString) -> String {
//     Node::Text(Text {
//         value: value.to_string(),
//         position: None,
//     })
// }

// pub fn paragraph(children: Vec<Node>) -> String {
//     Node::Paragraph(Paragraph {
//         children,
//         position: None,
//     })
// }
