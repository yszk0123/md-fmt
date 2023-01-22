use std::fs;

use anyhow::{Context, Result};
use mdfmt::{to_markdown, to_mdast_from_str};

#[test]
fn markdown() -> Result<()> {
    for file in vec!["short.md", "complex.md"] {
        let input = fs::read_to_string(format!("fixtures/input/{}", file))
            .with_context(|| format!("could not find input file `{}`", file))?;
        let expected = fs::read_to_string(format!("fixtures/output/{}", file))
            .with_context(|| format!("could not find output file `{}`", file))?;
        let actual = to_markdown(&to_mdast_from_str(&input)?)?;
        assert_eq!(expected, actual);
    }
    Ok(())
}
