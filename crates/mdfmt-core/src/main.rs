use std::env;

use anyhow::Result;
use mdfmt_core::{run, Config};

fn main() -> Result<()> {
    let config = Config::build(env::args())?;

    run(&config)?;

    Ok(())
}
