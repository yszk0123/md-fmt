use anyhow::Result;

pub trait Printer {
    fn print(&self) -> Result<String>;
}
