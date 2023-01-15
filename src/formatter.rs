pub mod rule;

use rule::Rule;

pub struct Formatter {
    rules: Vec<Box<dyn Rule>>,
}

impl Formatter {
    pub fn new(rules: Vec<Box<dyn Rule>>) -> Self {
        Formatter { rules }
    }

    pub fn apply(&self, content: String) -> String {
        self.rules
            .iter()
            .fold(content, |acc, rule| rule.format(acc))
    }
}
