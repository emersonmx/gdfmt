use anyhow::{Context, Result};
use gdfmt::formatter::format_code;

fn main() -> Result<()> {
    let file_path = std::env::args()
        .nth(1)
        .context("Please provide a file path as the first argument.")?;

    let source = std::fs::read_to_string(&file_path)
        .with_context(|| format!("Failed to load code from {}", file_path))?;

    let output = format_code(&source)?;
    print!("{}", output);

    Ok(())
}
