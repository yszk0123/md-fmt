use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use serde_with::{json::JsonString, serde_as, skip_serializing_none, DisplayFromStr, OneOrMany};
use std::collections::HashMap;

#[serde_as]
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
pub struct Metadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub path: Option<String>,
    pub bookmark: Option<Bookmark>,

    #[serde_as(as = "Option<OneOrMany<_>>")]
    pub tags: Option<Vec<String>>,

    #[serde(flatten)]
    #[serde_as(as = "HashMap<DisplayFromStr, JsonString>")]
    others: HashMap<String, Box<RawValue>>,
}

impl Metadata {
    pub fn from_str(s: &str) -> Result<Self> {
        serde_yaml::from_str(s).with_context(|| "could not stringify front matter".to_string())
    }

    pub fn to_md(&self) -> Result<String> {
        serde_yaml::to_string(self).with_context(|| "could not stringify front matter".to_string())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BookmarkId(String);

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Bookmark {
    pub id: Option<BookmarkId>,
    pub image: Option<String>,
    pub journal_date: Option<DateTime<Local>>,

    #[serde(flatten)]
    #[serde_as(as = "HashMap<DisplayFromStr, JsonString>")]
    others: HashMap<String, Box<RawValue>>,
}
