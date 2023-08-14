use std::collections::BTreeMap;

use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{formats::PreferMany, serde_as, skip_serializing_none, DisplayFromStr, OneOrMany};
use yaml_rust::{YamlEmitter, YamlLoader};

use super::model::NoteKind;
use crate::note::flexible_date::FlexibleDate;
use crate::{note::flexible_date_time::FlexibleDateTime, toc::Toc};

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub enum Metadata {
    Meta(Meta),
    Raw(String),
}

impl Metadata {
    pub fn from_str(s: &str) -> Result<Self> {
        Ok(Metadata::Meta(Meta::from_str(s)?))
    }

    pub fn to_md(&self) -> Result<String> {
        match self {
            Self::Meta(v) => v.to_md(),
            Self::Raw(v) => Ok(v.to_owned() + "\n"),
        }
    }

    pub fn normalize(self) -> Option<Self> {
        match self {
            Self::Meta(v) => v.normalize().map(Self::Meta),
            v => Some(v),
        }
    }
}

#[serde_as]
#[skip_serializing_none]
#[derive(Default, PartialEq, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub title: Option<String>,
    pub description: Option<String>,
    pub path: Option<String>,
    pub bookmark: Option<Bookmark>,
    pub link: Option<String>,
    pub toc: Option<String>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    pub status: Option<NoteStatus>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    pub kind: Option<NoteKind>,

    #[serde_as(as = "Option<FlexibleDate>")]
    pub journal_date: Option<NaiveDate>,

    #[serde_as(as = "Option<FlexibleDateTime>")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde_as(as = "Option<FlexibleDateTime>")]
    pub updated_at: Option<DateTime<Utc>>,

    #[serde_as(as = "Option<OneOrMany<_, PreferMany>>")]
    pub author: Option<Vec<String>>,

    #[serde_as(as = "Option<OneOrMany<_, PreferMany>>")]
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

    #[serde_as(as = "Option<FlexibleDate>")]
    journal_date: Option<NaiveDate>,

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

impl Meta {
    pub fn from_str(s: &str) -> Result<Self> {
        serde_yaml::from_str(s).with_context(|| "could not stringify front matter".to_string())
    }

    pub fn to_md(&self) -> Result<String> {
        let s = serde_yaml::to_string(self)
            .with_context(|| "could not stringify front matter".to_string())?;
        let s = self.fix_indent(&s);
        Ok(s)
    }

    fn fix_indent(&self, input: &str) -> String {
        // Workaround
        // https://github.com/dtolnay/serde-yaml/issues/337
        let docs = YamlLoader::load_from_str(input).unwrap();
        let mut out_str = String::new();
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(&docs[0]).unwrap();
        out_str[4..].to_string() + "\n"
    }

    pub fn normalize(self) -> Option<Self> {
        let bookmark = self.bookmark.as_ref();

        let res = Self {
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
            bookmark: self.bookmark.and_then(|v| v.normalize()),
            toc: None,
            others: self.others,
        };
        if res == Self::default() {
            None
        } else {
            Some(res)
        }
    }

    pub fn parse_toc(&self) -> Result<Option<Toc>> {
        if let Some(v) = &self.toc {
            let res = Toc::parse(v)?;
            Ok(Some(res))
        } else {
            Ok(None)
        }
    }
}

impl Bookmark {
    pub fn toc(value: &str) -> Self {
        Self {
            toc: Some(value.to_string()),
            ..Default::default()
        }
    }

    pub fn normalize(self) -> Option<Self> {
        if self.id.is_none() && self.image.is_none() && self.others.is_empty() {
            return None;
        }

        Some(Self {
            id: self.id,
            image: self.image,
            journal_date: None,
            url: None,
            created_at: None,
            updated_at: None,
            title: None,
            toc: None,
            others: self.others,
        })
    }

    pub fn parse_toc(&self) -> Result<Option<Toc>> {
        if let Some(v) = &self.toc {
            let res = Toc::parse(v)?;
            Ok(Some(res))
        } else {
            Ok(None)
        }
    }
}

#[derive(PartialEq, Debug, Default, Serialize, Deserialize, Clone)]
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
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn normalize_empty() {
        assert_eq!(Meta::default().normalize(), None);
    }

    #[test]
    fn normalize_title() {
        let metadata = Meta {
            title: None,
            bookmark: Some(Bookmark {
                title: Some("foo".into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(
            metadata.normalize(),
            Some(Meta {
                title: Some("foo".into()),
                bookmark: None,
                ..Default::default()
            })
        );
    }

    #[test]
    fn normalize_link() {
        let metadata = Meta {
            link: None,
            bookmark: Some(Bookmark {
                url: Some("foo".into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(
            metadata.normalize(),
            Some(Meta {
                link: Some("foo".into()),
                bookmark: None,
                ..Default::default()
            })
        );
    }

    #[test]
    fn normalize_journal_date() {
        let metadata = Meta {
            journal_date: None,
            bookmark: Some(Bookmark {
                journal_date: NaiveDate::from_ymd_opt(2000, 1, 1),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(
            metadata.normalize(),
            Some(Meta {
                journal_date: NaiveDate::from_ymd_opt(2000, 1, 1),
                bookmark: None,
                ..Default::default()
            })
        );
    }

    #[test]
    fn normalize_others() {
        let metadata = Meta {
            bookmark: Some(Bookmark {
                others: BTreeMap::from([("foo".into(), serde_yaml::Value::String("bar".into()))]),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(
            metadata.normalize(),
            Some(Meta {
                bookmark: Some(Bookmark {
                    others: BTreeMap::from([(
                        "foo".into(),
                        serde_yaml::Value::String("bar".into())
                    )]),
                    ..Default::default()
                }),
                ..Default::default()
            })
        );
    }

    #[test]
    fn serialize() -> Result<()> {
        assert_eq!(
            serde_json::to_value(&Meta {
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
            serde_json::from_value::<Meta>(json!({
                "kind": "quote",
                "status": "in progress"
            }))?,
            Meta {
                kind: Some(NoteKind::Quote),
                status: Some(NoteStatus::InProgress),
                ..Default::default()
            }
        );
        Ok(())
    }
}
