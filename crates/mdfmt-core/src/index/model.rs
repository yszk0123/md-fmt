use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::note::{
    metadata::{Meta, Metadata},
    model::Note,
};

#[derive(PartialEq, Serialize, Deserialize, Debug)]
struct Data {
    pub file: String,
    pub path: PathBuf,
    pub meta: Option<Meta>,
}

pub struct Index {
    data: Data,
}

impl Index {
    pub fn new(path: &PathBuf, note: &Note) -> Self {
        let file = path
            .file_name()
            .and_then(|v| v.to_str())
            .unwrap_or("")
            .to_string();

        Self {
            data: Data {
                file,
                path: path.to_owned(),
                meta: match &note.metadata {
                    Some(Metadata::Meta(v)) => Some(v.clone()),
                    _ => None,
                },
            },
        }
    }

    pub fn to_json(&self) -> Result<Option<String>> {
        if self.data.meta.is_none() {
            Ok(None)
        } else {
            let res = serde_json::to_string(&self.data)
                .with_context(|| format!("could not print file `{}`", self.data.path.display()))?;
            Ok(Some(res))
        }
    }
}
