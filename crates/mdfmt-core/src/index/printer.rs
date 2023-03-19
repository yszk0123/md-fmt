use anyhow::Result;

use super::model::Index;

pub struct IndexPrinter {}

impl IndexPrinter {
    pub fn print(indexes: &[Index]) -> Result<String> {
        let mut res = String::from("[");
        let mut first = true;

        for item in indexes {
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
