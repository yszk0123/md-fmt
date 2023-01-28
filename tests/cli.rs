use std::fs;

use anyhow::{Context, Result};
use mdfmt::{format, to_mdast_from_str};

#[test]
fn markdown() -> Result<()> {
    for file in &["short.md", "metadata.md", "complex.md", "journal_date.md"] {
        let input = fs::read_to_string(format!("fixtures/input/{file}"))
            .with_context(|| format!("could not find input file `{file}`"))?;
        let expected = fs::read_to_string(format!("fixtures/output/{file}"))
            .with_context(|| format!("could not find output file `{file}`"))?;
        let actual = format(&to_mdast_from_str(&input)?)?;
        assert_eq!(expected, actual, "test \"{file}\" does not match");
    }
    Ok(())
}
