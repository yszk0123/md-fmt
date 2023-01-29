use std::collections::BTreeMap;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, DisplayFromStr, OneOrMany};
use yaml_rust::{YamlEmitter, YamlLoader};

use super::model::NoteKind;
use crate::note::flexible_date_time::FlexibleDateTime;

#[serde_as]
#[skip_serializing_none]
#[derive(Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub path: Option<String>,
    pub bookmark: Option<Bookmark>,
    pub link: Option<String>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    pub status: Option<NoteStatus>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    pub kind: Option<NoteKind>,

    #[serde_as(as = "Option<FlexibleDateTime>")]
    pub journal_date: Option<DateTime<Utc>>,

    #[serde_as(as = "Option<FlexibleDateTime>")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde_as(as = "Option<FlexibleDateTime>")]
    pub updated_at: Option<DateTime<Utc>>,

    #[serde_as(as = "Option<OneOrMany<_>>")]
    pub author: Option<Vec<String>>,

    #[serde_as(as = "Option<OneOrMany<_>>")]
    pub tags: Option<Vec<String>>,

    #[serde(flatten)]
    #[serde_as(as = "BTreeMap<DisplayFromStr, _>")]
    pub others: BTreeMap<String, serde_yaml::Value>,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Default, PartialEq, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Bookmark {
    pub id: Option<BookmarkId>,
    pub image: Option<String>,

    title: Option<String>,

    toc: Option<String>,

    #[serde_as(as = "Option<FlexibleDateTime>")]
    journal_date: Option<DateTime<Utc>>,

    #[serde_as(as = "Option<FlexibleDateTime>")]
    created_at: Option<DateTime<Utc>>,

    #[serde_as(as = "Option<FlexibleDateTime>")]
    updated_at: Option<DateTime<Utc>>,

    url: Option<String>,

    #[serde(flatten)]
    #[serde_as(as = "BTreeMap<DisplayFromStr, _>")]
    others: BTreeMap<String, serde_yaml::Value>,
}

#[derive(Default, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct BookmarkId(String);

impl Metadata {
    pub fn from_str(s: &str) -> Result<Self> {
        serde_yaml::from_str(s).with_context(|| "could not stringify front matter".to_string())
    }

    pub fn to_md(&self) -> Result<String> {
        let s = serde_yaml::to_string(self)
            .with_context(|| "could not stringify front matter".to_string())?;
        let s = self.fix_indent(s);
        Ok(s)
    }

    fn fix_indent(&self, input: String) -> String {
        // Workaround
        // https://github.com/dtolnay/serde-yaml/issues/337
        let docs = YamlLoader::load_from_str(&input).unwrap();
        let mut out_str = String::new();
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(&docs[0]).unwrap();
        out_str[4..].to_string() + "\n"
    }

    pub fn normalize(self) -> Self {
        let bookmark = self.bookmark.as_ref();

        Self {
            title: self.title.or_else(|| bookmark?.title.clone()),
            description: self.description,
            path: self.path,
            author: self.author,
            tags: self.tags,
            status: self.status,
            kind: self.kind,
            journal_date: self.journal_date.or_else(|| bookmark?.journal_date),
            created_at: self.created_at.or_else(|| bookmark?.created_at),
            updated_at: self.updated_at.or_else(|| bookmark?.updated_at),
            link: self.link.or_else(|| bookmark?.url.clone()),
            bookmark: self.bookmark.map(|v| v.normalize()),
            others: self.others,
        }
    }
}

impl Bookmark {
    pub fn normalize(self) -> Self {
        Self {
            id: self.id,
            image: self.image,
            journal_date: None,
            url: None,
            created_at: None,
            updated_at: None,
            title: None,
            toc: None,
            others: self.others,
        }
    }
}

#[derive(PartialEq, Debug, Default, Serialize, Deserialize)]
pub enum NoteStatus {
    #[default]
    Todo,
    InProgress,
    Done,
    NotPlanned,
    Archived,
}

impl std::fmt::Display for NoteStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Todo => write!(f, "todo"),
            Self::InProgress => write!(f, "in progress"),
            Self::Done => write!(f, "done"),
            Self::NotPlanned => write!(f, "not planned"),
            Self::Archived => write!(f, "archived"),
        }
    }
}

impl std::str::FromStr for NoteStatus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "todo" => Ok(Self::Todo),
            "in progress" => Ok(Self::InProgress),
            "done" => Ok(Self::Done),
            "not planned" => Ok(Self::NotPlanned),
            "archived" => Ok(Self::Archived),
            _ => Ok(Self::Todo),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn normalize_title() {
        let metadata = Metadata {
            title: None,
            bookmark: Some(Bookmark {
                title: Some("foo".into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(
            metadata.normalize(),
            Metadata {
                title: Some("foo".into()),
                bookmark: Some(Bookmark {
                    title: None,
                    ..Default::default()
                }),
                ..Default::default()
            }
        );
    }

    #[test]
    fn normalize_link() {
        let metadata = Metadata {
            link: None,
            bookmark: Some(Bookmark {
                url: Some("foo".into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(
            metadata.normalize(),
            Metadata {
                link: Some("foo".into()),
                bookmark: Some(Bookmark {
                    url: None,
                    ..Default::default()
                }),
                ..Default::default()
            }
        );
    }

    #[test]
    fn normalize_journal_date() {
        let metadata = Metadata {
            journal_date: None,
            bookmark: Some(Bookmark {
                journal_date: Some(Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap()),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(
            metadata.normalize(),
            Metadata {
                journal_date: Some(Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap()),
                bookmark: Some(Bookmark {
                    journal_date: None,
                    ..Default::default()
                }),
                ..Default::default()
            }
        );
    }

    #[test]
    fn serialize() -> Result<()> {
        assert_eq!(
            serde_json::to_value(&Metadata {
                kind: Some(NoteKind::Quote),
                status: Some(NoteStatus::InProgress),
                ..Default::default()
            })?,
            json!({
                "kind": "quote",
                "status": "in progress"
            })
        );
        Ok(())
    }

    #[test]
    fn deserialize() -> Result<()> {
        assert_eq!(
            serde_json::from_value::<Metadata>(json!({
                "kind": "quote",
                "status": "in progress"
            }))?,
            Metadata {
                kind: Some(NoteKind::Quote),
                status: Some(NoteStatus::InProgress),
                ..Default::default()
            }
        );
        Ok(())
    }
}
