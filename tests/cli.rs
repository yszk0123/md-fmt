use std::{fs, path::Path};

use anyhow::{Context, Result};
use mdfmt::{format, to_mdast_from_str};
use pretty_assertions::assert_eq;

#[test]
fn markdown() -> Result<()> {
    let entries = fs::read_dir("fixtures/input")?
        .map(|res| res.map(|e| e.path()))
        .collect::<std::result::Result<Vec<_>, std::io::Error>>()?;
    let paths = entries.iter().filter(|v| v.is_file()).collect::<Vec<_>>();

    for path in paths {
        if let Some(name) = path.file_name() {
            let output_path = Path::new("fixtures/output").join(name);

            let input = fs::read_to_string(path)
                .with_context(|| format!("could not find input file `{}`", path.display()))?;
            let actual = format(&to_mdast_from_str(&input)?)?;

            let expected = fs::read_to_string(&output_path).with_context(|| {
                format!("could not find output file `{}`", output_path.display())
            })?;

            assert_eq!(
                expected,
                actual,
                "test \"{}\" does not match",
                output_path.display()
            );
        }
    }
    Ok(())
}
