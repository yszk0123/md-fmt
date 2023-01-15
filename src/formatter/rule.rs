pub trait Rule {
    fn format(&self, content: String) -> String;
}
