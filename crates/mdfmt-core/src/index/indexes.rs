use std::path::PathBuf;

use anyhow::Result;

use super::item::Item;
use crate::note::Note;
use crate::printer::Printer;

pub struct Indexes {
    pub(super) data: Vec<Item>,
}

impl Indexes {
    pub fn new(data: Vec<Item>) -> Self {
        Self { data }
    }

    pub fn push(&mut self, path: &PathBuf, note: &Note) {
        self.data.push(Item::new(path, note));
    }
}

impl Printer for Indexes {
    type Options = ();

    fn print(&self, _options: Self::Options) -> Result<String> {
        let mut res = String::from("[");
        let mut first = true;

        for item in &self.data {
            if let Some(json) = item.to_json()? {
                if first {
                    first = false;
                } else {
                    res.push(',');
                }
                res.push_str(&json);
            }
        }
        res.push(']');

        Ok(res)
    }
}
