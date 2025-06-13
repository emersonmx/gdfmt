use anyhow::{Context, Result};
use gdfmt::format_code;
use std::{fs::read_to_string, io::Read};

fn main() -> Result<()> {
    let mut source = String::new();
    let args: Vec<String> = std::env::args().collect();

    if let Some(file_path) = args.get(1) {
        let data = read_to_string(file_path)
            .with_context(|| format!("Unable to read from file: {}", file_path))?;
        source.push_str(&data);
    } else {
        std::io::stdin()
            .read_to_string(&mut source)
            .context("Unable to read from stdin.")?;
    }

    let output = format_code(&source)?;
    print!("{}", output);

    Ok(())
}
