use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser as ClaspParser;

/// Simple Markdown Formatter
#[derive(ClaspParser, Debug)]
#[command(version)]
struct Args {
    /// Source
    #[arg(short, long)]
    file: Vec<PathBuf>,

    /// Overwrite
    #[arg(short, long, default_value = "false")]
    write: bool,

    #[arg(long)]
    md: bool,

    #[arg(long)]
    note: bool,
}

pub struct Config {
    pub files: Vec<PathBuf>,
    pub write: bool,
    pub md: bool,
    pub note: bool,
}

impl Config {
    pub fn build(args: impl Iterator<Item = String>) -> Result<Config> {
        let args =
            Args::try_parse_from(args).with_context(|| "could not parse arguments".to_string())?;

        Ok(Config {
            files: args.file.clone(),
            write: args.write,
            md: args.md,
            note: args.note,
        })
    }
}
