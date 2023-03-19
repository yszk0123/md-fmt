use std::{io, path::PathBuf};

use anyhow::{Context, Result};
use clap::Parser as ClaspParser;

use super::quoted_args::parse_quoted_args;

/// Simple Markdown Formatter
#[derive(ClaspParser, Debug)]
#[command(version)]
struct Args {
    /// Files
    files: Option<Vec<PathBuf>>,

    /// Files
    #[arg(short, long)]
    file: Vec<PathBuf>,

    #[arg(short, long)]
    glob: Option<String>,

    #[arg(long)]
    index: Option<String>,

    /// Overwrite
    #[arg(short, long, default_value = "false")]
    write: bool,

    #[arg(long)]
    md: bool,

    #[arg(long)]
    note: bool,

    #[arg(long)]
    check: bool,

    /// Read files from stdin
    #[arg(long, default_value = "false")]
    stdin: bool,
}

pub struct Config {
    pub files: Vec<PathBuf>,
    pub glob: Option<String>,
    pub index: Option<String>,
    pub write: bool,
    pub md: bool,
    pub note: bool,
    pub check: bool,
}

impl Config {
    pub fn build(args: impl Iterator<Item = String>) -> Result<Config> {
        let args =
            Args::try_parse_from(args).with_context(|| "could not parse arguments".to_string())?;

        let files = if args.stdin {
            io::stdin()
                .lines()
                .map(|v| v.unwrap())
                .flat_map(|line| {
                    parse_quoted_args(&line)
                        .iter()
                        .map(PathBuf::from)
                        .collect::<Vec<_>>()
                })
                .collect()
        } else {
            vec![]
        };

        Ok(Config {
            files: [args.files.unwrap_or_default(), args.file, files].concat(),
            glob: args.glob,
            index: args.index,
            write: args.write,
            md: args.md,
            note: args.note,
            check: args.check,
        })
    }
}
