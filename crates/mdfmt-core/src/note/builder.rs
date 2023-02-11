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
