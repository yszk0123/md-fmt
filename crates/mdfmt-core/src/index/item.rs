use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::note::{Meta, Metadata, Note};

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct Item {
    pub file: String,
    pub path: PathBuf,
    pub meta: Option<Meta>,
}

impl Item {
    pub fn new(path: &PathBuf, note: &Note) -> Self {
        let file = path
            .file_name()
            .and_then(|v| v.to_str())
            .unwrap_or("")
            .to_string();

        Self {
            file,
            path: path.to_owned(),
            meta: match &note.metadata {
                Some(Metadata::Meta(v)) => Some(v.clone()),
                _ => None,
            },
        }
    }

    pub fn to_json(&self) -> Result<Option<String>> {
        if self.meta.is_none() {
            Ok(None)
        } else {
            let res = serde_json::to_string(&self)
                .with_context(|| format!("could not print file `{}`", self.path.display()))?;
            Ok(Some(res))
        }
    }
}
