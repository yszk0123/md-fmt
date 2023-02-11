use std::env;

use anyhow::Result;
use mdfmt_core::cli::config::Config;

fn main() -> Result<()> {
    let config = Config::build(env::args())?;

    mdfmt_core::run(&config)?;

    Ok(())
}
