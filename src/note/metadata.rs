use std::collections::BTreeMap;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, DisplayFromStr, OneOrMany};

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

    #[serde_as(as = "Option<FlexibleDateTime>")]
    pub journal_date: Option<DateTime<Utc>>,

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

    #[serde_as(as = "Option<FlexibleDateTime>")]
    journal_date: Option<DateTime<Utc>>,

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
        serde_yaml::to_string(self).with_context(|| "could not stringify front matter".to_string())
    }

    pub fn normalize(self) -> Self {
        let bookmark = self.bookmark.as_ref();

        Self {
            title: self.title,
            description: self.description,
            path: self.path,
            author: self.author,
            tags: self.tags,
            journal_date: bookmark
                .map(|v| v.journal_date)
                .unwrap_or_else(|| self.journal_date),
            link: bookmark.map(|v| v.url.clone()).unwrap_or_else(|| self.link),
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
            others: self.others,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn metadata_normalize_link() {
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
    fn metadata_normalize_journal_date() {
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
}
