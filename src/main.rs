use anyhow::{Context, Result};
use gdfmt::format_code;
use std::io::Read;

fn main() -> Result<()> {
    let mut source = String::new();
    std::io::stdin()
        .read_to_string(&mut source)
        .context("Unable to read from stdin.")?;

    let output = format_code(&source)?;
    print!("{}", output);

    Ok(())
}
