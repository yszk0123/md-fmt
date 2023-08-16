use anyhow::Result;

pub trait Printer {
    type Options;
    fn print(&self, options: Self::Options) -> Result<String>;
}
