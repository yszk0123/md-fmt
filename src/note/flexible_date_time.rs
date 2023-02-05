use std::fmt;

use anyhow::{anyhow, Result};
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, Utc};
use serde::{de, Deserializer, Serialize, Serializer};
use serde_with::{DeserializeAs, SerializeAs};

pub struct FlexibleDateTime;

impl SerializeAs<DateTime<Utc>> for FlexibleDateTime {
    fn serialize_as<S>(value: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        value.serialize(serializer)
    }
}

impl<'de> DeserializeAs<'de, DateTime<Utc>> for FlexibleDateTime {
    fn deserialize_as<D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(DateTimeVisitor)
    }
}

struct DateTimeVisitor;

impl<'de> de::Visitor<'de> for DateTimeVisitor {
    type Value = DateTime<Utc>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a formatted date and time string or a unix timestamp")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if let Ok(t) =
            DateTime::<FixedOffset>::parse_from_rfc3339(value).map(|dt| dt.with_timezone(&Utc))
        {
            return Ok(t);
        }
        if let Ok(t) = NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M:%S") {
            return Ok(DateTime::<Utc>::from_utc(t, Utc));
        }
        if let Ok(t) = NaiveDate::parse_from_str(value, "%Y-%m-%d") {
            let t = t
                .and_hms_opt(0, 0, 0)
                .map(|t| DateTime::<Utc>::from_utc(t, Utc));
            if let Some(t) = t {
                return Ok(t);
            }
        }
        if let Ok(t) = parse_yyyy_mm(value) {
            let t = t
                .and_hms_opt(0, 0, 0)
                .map(|t| DateTime::<Utc>::from_utc(t, Utc));
            if let Some(t) = t {
                return Ok(t);
            }
        }

        Err(E::custom(""))
    }
}

fn parse_yyyy_mm(s: &str) -> Result<NaiveDate> {
    let mut ss = s.split('-');
    let year = ss.next().ok_or_else(|| anyhow!(""))?.parse::<i32>()?;
    let month = ss.next().ok_or_else(|| anyhow!(""))?.parse::<u32>()?;
    NaiveDate::from_ymd_opt(year, month, 1).ok_or_else(|| anyhow!(""))
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use serde::Deserialize;
    use serde_json::json;
    use serde_with::serde_as;

    use super::*;

    #[serde_as]
    #[derive(PartialEq, Serialize, Deserialize, Debug)]
    struct Foo {
        #[serde_as(as = "Option<FlexibleDateTime>")]
        date: Option<DateTime<Utc>>,
    }

    #[test]
    fn it_should_parse_yyyy_mm() -> Result<()> {
        let expected = Foo {
            date: Some(Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap()),
        };
        let actual: Foo = serde_json::from_value(json!({ "date": "2000-01" }))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn it_should_parse_naive_date() -> Result<()> {
        let expected = Foo {
            date: Some(Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap()),
        };
        let actual: Foo = serde_json::from_value(json!({ "date": "2000-01-01" }))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn it_should_parse_naive_date_time() -> Result<()> {
        let expected = Foo {
            date: Some(Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap()),
        };
        let actual: Foo = serde_json::from_value(json!({ "date": "2000-01-01T00:00:00" }))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn it_should_parse_date_time() -> Result<()> {
        let expected = Foo {
            date: Some(Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap()),
        };
        let actual: Foo = serde_json::from_value(json!({ "date": "2000-01-01T00:00:00+00:00" }))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
