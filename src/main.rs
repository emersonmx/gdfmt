use anyhow::{Context, Result};
use gdfmt::formatter::format_node;
use tree_sitter::Parser;
use tree_sitter_gdscript::LANGUAGE as gdscript_language;

fn main() -> Result<()> {
    let file_path = std::env::args()
        .nth(1)
        .context("Please provide a file path as the first argument.")?;

    let source = std::fs::read_to_string(&file_path)
        .with_context(|| format!("Failed to load code from {}", file_path))?;

    let mut parser = Parser::new();
    parser
        .set_language(&gdscript_language.into())
        .context("Unable to load gdscript language")?;

    let tree = parser
        .parse(&source, None)
        .context("Unable to parse the source code")?;
    let root_node = tree.root_node();

    let output = format_node(root_node, &source, 0)?;
    print!("{}", output);

    Ok(())
}
