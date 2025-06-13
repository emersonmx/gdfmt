use crate::error::Error;
use crate::rules;
use tree_sitter::Parser;
use tree_sitter_gdscript::LANGUAGE as gdscript_language;

pub fn format_code(source: &str) -> Result<String, Error> {
    let mut parser = Parser::new();
    parser.set_language(&gdscript_language.into())?;

    let tree = parser.parse(source, None).ok_or_else(|| {
        Error::UnableToParse("Internal parser error: Failed to produce syntax tree.".to_string())
    })?;
    let root_node = tree.root_node();
    if root_node.has_error() {
        return Err(Error::UnableToParse(
            "Source code contains syntax errors.".to_string(),
        ));
    }

    println!("{}", "-".repeat(80));
    print!("{}", crate::debug::print_tree(root_node, source, 0));
    println!("{}", "-".repeat(80));

    Ok(rules::apply(root_node, source, 0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn returns_error_on_syntax_errors() {
        let result = format_code(".");
        assert!(matches!(result, Err(Error::UnableToParse(_))));
    }

    #[rstest]
    fn compare_with_sample_styleguide() {
        let expected = std::fs::read_to_string("samples/styleguide.gd").unwrap();

        let formatted = format_code(&expected).unwrap();

        assert_eq!(formatted, expected);
    }
}
