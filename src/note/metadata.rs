use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use serde_with::{json::JsonString, serde_as, skip_serializing_none, DisplayFromStr, OneOrMany};
use std::collections::HashMap;

#[serde_as]
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
pub struct Metadata {
    pub title: Option<String>,

    #[serde_as(as = "Option<OneOrMany<_>>")]
    pub tags: Option<Vec<String>>,

    #[serde(flatten)]
    #[serde_as(as = "HashMap<DisplayFromStr, JsonString>")]
    others: HashMap<String, Box<RawValue>>,
}

impl Metadata {
    pub fn from_str(s: &str) -> Result<Self> {
        serde_yaml::from_str(s).with_context(|| format!("could not stringify front matter"))
    }

    pub fn to_md(&self) -> Result<String> {
        serde_yaml::to_string(self).with_context(|| format!("could not stringify front matter"))
    }
}
