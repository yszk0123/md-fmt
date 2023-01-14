use std::error::Error;
use std::fs;

pub struct Config {
    pub input: String,
    pub write: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let input = match args.next() {
            Some(arg) => arg,
            None => return Err("required input file path"),
        };
        let write = match args.next() {
            Some(arg) => arg == "--write",
            None => false,
        };
        Ok(Config { input, write })
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
