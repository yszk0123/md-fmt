use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser as ClaspParser;

/// Simple Markdown Formatter
#[derive(ClaspParser, Debug)]
#[command(version)]
struct Args {
    files: Option<Vec<PathBuf>>,

    /// Source
    #[arg(short, long)]
    file: Vec<PathBuf>,

    #[arg(short, long)]
    glob: Option<String>,

    /// Overwrite
    #[arg(short, long, default_value = "false")]
    write: bool,

    #[arg(long)]
    md: bool,

    #[arg(long)]
    note: bool,

    #[arg(long)]
    check: bool,
}

pub struct Config {
    pub files: Vec<PathBuf>,
    pub glob: Option<String>,
    pub write: bool,
    pub md: bool,
    pub note: bool,
    pub check: bool,
}

impl Config {
    pub fn build(args: impl Iterator<Item = String>) -> Result<Config> {
        let args =
            Args::try_parse_from(args).with_context(|| "could not parse arguments".to_string())?;

        Ok(Config {
            files: [args.files.unwrap_or_default(), args.file].concat(),
            glob: args.glob,
            write: args.write,
            md: args.md,
            note: args.note,
            check: args.check,
        })
    }
}
