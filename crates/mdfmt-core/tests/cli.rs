use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use mdfmt_core::{format, generate_index};
use pretty_assertions::assert_eq;

#[test]
fn markdown() -> Result<()> {
    let entries = fs::read_dir("fixtures/format")?
        .map(|res| res.map(|e| e.path()))
        .collect::<std::result::Result<Vec<_>, std::io::Error>>()?;
    let paths = entries.iter().filter(|v| v.is_dir()).collect::<Vec<_>>();

    for path in paths {
        if let Some(name) = path.file_name() {
            let dir = Path::new("fixtures/format").join(name);
            let input_path = dir.join("input.md");
            let output_path = dir.join("output.md");

            let input = fs::read_to_string(&input_path)
                .with_context(|| format!("could not find input file `{}`", path.display()))?;
            let actual = format(&input)?;

            let expected = fs::read_to_string(&output_path).with_context(|| {
                format!("could not find output file `{}`", output_path.display())
            })?;

            assert_eq!(
                actual,
                expected,
                "test \"{}\" does not match",
                output_path.display()
            );
        }
    }
    Ok(())
}

#[test]
fn index() -> Result<()> {
    let entries = fs::read_dir("fixtures/index/input")?
        .map(|res| res.map(|e| e.path()))
        .collect::<std::result::Result<Vec<_>, std::io::Error>>()?;
    let paths = entries
        .into_iter()
        .filter(|v| v.is_file())
        .collect::<Vec<PathBuf>>();
    let output_path = Path::new("fixtures/index/output.json");

    let actual = generate_index(&paths)?;

    let expected = fs::read_to_string(output_path)
        .with_context(|| format!("could not find output file `{}`", output_path.display()))?;

    assert_eq!(
        actual,
        expected,
        "test \"{}\" does not match",
        output_path.display()
    );
    Ok(())
}
