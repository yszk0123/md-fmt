use crate::formatter::rule::Rule;

pub struct TrimRule {}

impl Rule for TrimRule {
    fn format(&self, content: String) -> String {
        format!("{}\n", content.trim())
    }
}
