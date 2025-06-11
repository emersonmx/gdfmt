use thiserror::Error;
use tree_sitter::LanguageError;
use tree_sitter::Node;
use tree_sitter::Parser;
use tree_sitter_gdscript::LANGUAGE as gdscript_language;

const KINDS_WITH_TWO_LINES_BETWEEN: [&str; 3] = [
    "function_definition",
    "class_definition",
    "constructor_definition",
];

#[derive(Error, Debug)]
pub enum Error {
    #[error("unable to load language")]
    UnableToLoadLanguage(#[from] LanguageError),
    #[error("unable to parse: {0}")]
    UnableToParse(String),
}

pub fn format_code(source: &str) -> Result<String, Error> {
    let mut parser = Parser::new();
    parser.set_language(&gdscript_language.into())?;

    let tree = parser
        .parse(source, None)
        .ok_or_else(|| Error::UnableToParse("Failed to parse source code".to_string()))?;
    let root_node = tree.root_node();

    format_node_walk(root_node, source, 0)
}

fn format_node_walk(node: Node, source: &str, indent_level: usize) -> Result<String, Error> {
    let indent = "\t".repeat(indent_level);

    match node.kind() {
        "source" => format_source_kind(node, source, indent_level),
        "function_definition" => {
            format_function_definition_kind(node, source, indent_level, &indent)
        }
        _ => format_any_kind(node, source, &indent),
    }
}

fn format_source_kind(node: Node, source: &str, indent_level: usize) -> Result<String, Error> {
    let mut output = String::new();
    let mut cursor = node.walk();
    let mut prev_kind: Option<&str> = None;

    for child in node.children(&mut cursor) {
        if let Some(pk) = prev_kind {
            if KINDS_WITH_TWO_LINES_BETWEEN.contains(&pk)
                || KINDS_WITH_TWO_LINES_BETWEEN.contains(&child.kind())
            {
                output.push_str("\n\n");
            }
        }
        output += &format_node_walk(child, source, indent_level)?;
        prev_kind = Some(child.kind());
    }

    while output.ends_with("\n") {
        output.pop();
    }
    output.push('\n');
    Ok(output)
}

fn format_function_definition_kind(
    node: Node,
    source: &str,
    indent_level: usize,
    indent: &str,
) -> Result<String, Error> {
    let header = node
        .child_by_field_name("name")
        .map(|n| &source[n.byte_range()])
        .unwrap_or("func_name");

    let parameters_node = node.child_by_field_name("parameters");
    let parameters_text = parameters_node
        .map(|n| &source[n.byte_range()])
        .unwrap_or("()");

    let body = node.child_by_field_name("body");

    let mut output = format!(
        "{}func {}{}:\n",
        indent,
        header.trim(),
        parameters_text.trim()
    );

    if let Some(body_node) = body {
        for child in body_node.children(&mut body_node.walk()) {
            output += &format_node_walk(child, source, indent_level + 1)?;
        }
    }
    Ok(output)
}

fn format_any_kind(node: Node, source: &str, indent: &str) -> Result<String, Error> {
    let text = &source[node.byte_range()];
    let output = format!("{}{}\n", indent, text.trim());
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("var a = 0    ", "var a = 0\n")]
    #[case("var a = 0\t", "var a = 0\n")]
    #[case("var a = 0 \t", "var a = 0\n")]
    #[case("var a = 0  \n \t", "var a = 0\n")]
    fn trim_trailing_spaces(#[case] source_input: &str, #[case] expected_output: &str) {
        let formatted = format_code(source_input).unwrap();

        assert_eq!(
            formatted, expected_output,
            "Failed for input: {:?}",
            source_input
        );
    }

    #[rstest]
    #[case("var a = 0", "var a = 0\n")]
    #[case("var b = 1\n", "var b = 1\n")]
    #[case("var c = 2\n\n", "var c = 2\n")]
    #[case("var d = 3\n\n\n", "var d = 3\n")]
    fn keep_one_newline_at_end(#[case] source_input: &str, #[case] expected_output: &str) {
        let formatted = format_code(source_input).unwrap();

        assert_eq!(
            formatted, expected_output,
            "Failed for input: {:?}",
            source_input
        );
    }

    #[rstest]
    #[case(
        "func a():\n\tpass\nfunc b():\n\tpass",
        "func a():\n\tpass\n\n\nfunc b():\n\tpass\n"
    )]
    #[case(
        "func a():\n\tpass\n\nfunc b():\n\tpass",
        "func a():\n\tpass\n\n\nfunc b():\n\tpass\n"
    )]
    #[case(
        "func a():\n\tpass\n\n\nfunc b():\n\tpass",
        "func a():\n\tpass\n\n\nfunc b():\n\tpass\n"
    )]
    #[case(
        "func a():\n\tpass\n\n\n\nfunc b():\n\tpass",
        "func a():\n\tpass\n\n\nfunc b():\n\tpass\n"
    )]
    #[case(
        "\nfunc a():\n\tpass\nfunc b():\n\tpass",
        "func a():\n\tpass\n\n\nfunc b():\n\tpass\n"
    )]
    #[case(
        "\n\nfunc a():\n\tpass\nfunc b():\n\tpass",
        "func a():\n\tpass\n\n\nfunc b():\n\tpass\n"
    )]
    fn keep_two_lines_between(#[case] source_input: &str, #[case] expected_output: &str) {
        let formatted = format_code(source_input).unwrap();

        assert_eq!(
            formatted, expected_output,
            "Failed for input: {:?}",
            source_input
        );
    }

    #[rstest]
    fn compare_with_styleguide() {
        let expected = std::fs::read_to_string("samples/styleguide.gd").unwrap();

        let formatted = format_code(&expected).unwrap();

        assert_eq!(formatted, expected);
    }
}
