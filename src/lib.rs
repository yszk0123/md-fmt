use clap::Parser;
use std::error::Error;
use std::fs;

/// Simple Markdown Formatter
#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// Source
    #[arg(short, long)]
    input: String,

    /// Overwrite
    #[arg(short, long)]
    write: bool,
}

pub struct Config {
    pub input: String,
    pub write: bool,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let args = Args::parse();

        Ok(Config {
            input: args.input.clone(),
            write: args.write,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.input.clone())?;

    if config.write {
        fs::write(config.input.clone(), contents)?;
    } else {
        println!("{contents}");
    }

    Ok(())
}
