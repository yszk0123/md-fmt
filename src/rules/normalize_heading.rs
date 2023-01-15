use crate::formatter::rule::Rule;

pub struct NormalizeHeadingRule {}

impl Rule for NormalizeHeadingRule {
    fn format(&self, content: String) -> String {
        format!("{}\n", content.trim())
    }
}
