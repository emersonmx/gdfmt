use crate::error::Error;
use crate::node::{get_gap_lines, get_node_text, get_root_gap_lines};
use crate::text::{force_end_line, make_indent};
use tree_sitter::Node;
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

    Ok(format_node(root_node, source, 0))
}

fn format_node(node: Node, source: &str, indent_level: usize) -> String {
    match node.kind() {
        // with trailing line
        "source" => format_source_node(node, source, indent_level),
        "function_definition" | "constructor_definition" => {
            format_function_node(node, source, indent_level)
        }
        "class_definition" => format_class_definition_node(node, source, indent_level),
        "variable_statement" => format_variable_statement_node(node, source, indent_level),
        "class_name_statement"
        | "extends_statement"
        | "comment"
        | "signal_statement"
        | "expression_statement"
        | "pass_statement"
        | "if_statement" => formatted_text(node, source, indent_level),
        "body" => format_body_node(node, source, indent_level),
        // without trailing whitespace
        "setget" => format_setget_node(node, source, indent_level),
        "parameters" => format_parameters_node(node, source, indent_level),
        "default_parameter" => format_default_parameter_node(node, source, indent_level),
        "annotations" => format_annotations_node(node, source, indent_level),
        "annotation" => format_annotation_node(node, source, indent_level),
        "array" => format_array_node(node, source, indent_level),
        _ => get_node_text(node, source).to_string(),
    }
}

fn format_source_node(node: Node, source: &str, indent_level: usize) -> String {
    let mut output = String::new();
    for child in node.children(&mut node.walk()) {
        let child_output = format_node(child, source, indent_level);
        output.push_str(&child_output);
    }

    force_end_line(&mut output);
    output
}

fn format_function_node(node: Node, source: &str, indent_level: usize) -> String {
    let indent = make_indent(indent_level);
    let parent_kind = node.parent().map(|n| n.kind());
    let gap_lines = match parent_kind {
        Some("source") => get_root_gap_lines(node, source),
        _ => get_gap_lines(node, source),
    };
    let mut output = String::new();

    output.push_str(&gap_lines);
    output.push_str(&indent);

    for (i, child) in node.children(&mut node.walk()).enumerate() {
        let text = &format_node(child, source, indent_level);
        let (text, space): (&str, &str) = match child.kind() {
            _ if i == 0 => (text, ""),
            "parameters" => (text, ""),
            ":" => (text, ""),
            "body" => (
                &format!("\n{}", &format_node(child, source, indent_level + 1)),
                "",
            ),
            _ => (text, " "),
        };
        output.push_str(space);
        output.push_str(text);
    }

    output
}

fn format_class_definition_node(node: Node, source: &str, indent_level: usize) -> String {
    let parent_kind = node.parent().map(|n| n.kind());
    let gap_lines = match parent_kind {
        Some("source") => get_root_gap_lines(node, source),
        _ => get_gap_lines(node, source),
    };
    let mut output = String::new();

    output.push_str(&gap_lines);

    for (i, child) in node.children(&mut node.walk()).enumerate() {
        let text = &format_node(child, source, indent_level);
        let (text, space): (&str, &str) = match child.kind() {
            _ if i == 0 => (text, ""),
            ":" => (text, ""),
            "body" => (
                &format!("\n{}", &format_node(child, source, indent_level + 1)),
                "",
            ),
            _ => (text, " "),
        };
        output.push_str(space);
        output.push_str(text);
    }

    output
}

fn format_variable_statement_node(node: Node, source: &str, indent_level: usize) -> String {
    let indent = make_indent(indent_level);
    let gap_lines = get_gap_lines(node, source);
    let mut output = String::new();

    output.push_str(&gap_lines);
    output.push_str(&indent);

    for (i, child) in node.children(&mut node.walk()).enumerate() {
        let text = &format_node(child, source, indent_level + 1);
        let (text, space): (&str, &str) = match child.kind() {
            _ if i == 0 => (text, ""),
            ":" | "setget" => (text, ""),
            _ => (text, " "),
        };
        output.push_str(space);
        output.push_str(text);
    }
    output.push('\n');

    output
}

fn format_setget_node(node: Node, source: &str, indent_level: usize) -> String {
    let indent = make_indent(indent_level);
    let mut output = String::new();

    output.push(':');
    output.push('\n');
    output.push_str(&indent);
    for (i, child) in node.children(&mut node.walk()).enumerate() {
        let text = match child.kind() {
            _ if i == 0 => "",
            "=" => " = ",
            _ => &format_node(child, source, indent_level),
        };

        output.push_str(text);
    }

    output
}

fn format_parameters_node(node: Node, source: &str, indent_level: usize) -> String {
    let mut output = String::new();

    for child in node.children(&mut node.walk()) {
        let prev_kind = child.prev_sibling().map(|ps| ps.kind());
        let text = &format_node(child, source, indent_level);
        let (text, space): (&str, &str) = match child.kind() {
            "(" | ")" | "=" | "," => (text, ""),
            "identifier" | "default_parameter" if prev_kind == Some("(") => (text, ""),
            "identifier" => (text, " "),
            _ => (text, " "),
        };
        output.push_str(space);
        output.push_str(text);
    }

    output
}

fn format_body_node(node: Node, source: &str, indent_level: usize) -> String {
    let mut output = String::new();

    for child in node.children(&mut node.walk()) {
        let text = &format_node(child, source, indent_level);
        output.push_str(text);
    }

    output
}

fn format_default_parameter_node(node: Node, source: &str, indent_level: usize) -> String {
    let mut output = String::new();

    for child in node.children(&mut node.walk()) {
        let text = &format_node(child, source, indent_level);
        let (text, space): (&str, &str) = match child.kind() {
            "identifier" => (text, ""),
            _ => (text, ""),
        };
        output.push_str(space);
        output.push_str(text);
    }

    output
}

fn format_annotations_node(node: Node, source: &str, indent_level: usize) -> String {
    let mut output = String::new();

    for (i, child) in node.children(&mut node.walk()).enumerate() {
        let text = &format_node(child, source, indent_level);
        let (text, space): (&str, &str) = match child.kind() {
            _ if i == 0 => (text, ""),
            "annotation" => (text, " "),
            _ => (text, ""),
        };
        output.push_str(space);
        output.push_str(text);
    }

    output
}

fn format_annotation_node(node: Node, source: &str, indent_level: usize) -> String {
    let mut output = String::new();

    for child in node.children(&mut node.walk()) {
        let text = &format_node(child, source, indent_level);
        let (text, space): (&str, &str) = match child.kind() {
            "@" => (text, ""),
            "annotation" => (text, " "),
            _ => (text, ""),
        };
        output.push_str(space);
        output.push_str(text);
    }

    output
}

fn format_array_node(node: Node, source: &str, indent_level: usize) -> String {
    let mut output = String::new();

    for child in node.children(&mut node.walk()) {
        let text = &format_node(child, source, indent_level);
        let (text, space): (&str, &str) = match child.kind() {
            "[" => (text, ""),
            "," => (text, ""),
            _ => (text, " "),
        };
        output.push_str(space);
        output.push_str(text);
    }

    output
}

fn formatted_text(node: Node, source: &str, indent_level: usize) -> String {
    let indent = make_indent(indent_level);
    let text = get_node_text(node, source); // TODO: try format_node
    let gap_lines = get_gap_lines(node, source);
    let mut output = String::new();

    output.push_str(&gap_lines);
    output.push_str(&indent);
    output.push_str(text.trim());
    output.push('\n');

    output
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
    #[case("func     a  (   b    ,    c)    :    pass", "func a(b, c):\n\tpass\n")]
    #[case("func a( b = {} ,  c  =  42 ):pass", "func a(b={}, c=42):\n\tpass\n")]
    #[case("func a( b ,  c  =  42 ):pass", "func a(b, c=42):\n\tpass\n")]
    #[case("class     MyClass    :    pass", "class MyClass:\n\tpass\n")]
    #[case(
        "class MyClass:\n\tfunc a( b = 24 ,  c  =  42 ):pass",
        "class MyClass:\n\tfunc a(b=24, c=42):\n\t\tpass\n"
    )]
    fn trim_whitespaces(#[case] source_input: &str, #[case] expected_output: &str) {
        let formatted = format_code(source_input).unwrap();

        assert_eq!(
            formatted, expected_output,
            "Failed for input: {:?}",
            source_input
        );
    }

    #[rstest]
    #[case("var a = 0\nvar b = 0", "var a = 0\nvar b = 0\n")]
    #[case("var a = 0\n\nvar b = 0", "var a = 0\n\nvar b = 0\n")]
    #[case("var a = 0\n\n\n\nvar b = 0", "var a = 0\n\nvar b = 0\n")]
    fn keep_lines_between(#[case] source_input: &str, #[case] expected_output: &str) {
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
    #[case(
        "# a comment\n\nfunc a():\n\tpass\nfunc b():\n\tpass",
        "# a comment\n\nfunc a():\n\tpass\n\n\nfunc b():\n\tpass\n"
    )]
    #[case(
        "# a comment\n\nclass A:\n\tpass\nclass B:\n\tpass",
        "# a comment\n\nclass A:\n\tpass\n\n\nclass B:\n\tpass\n"
    )]
    #[case(
        "# a comment\n\nfunc _init():\n\tpass\nfunc b():\n\tpass",
        "# a comment\n\nfunc _init():\n\tpass\n\n\nfunc b():\n\tpass\n"
    )]
    #[case(
        "func _init():\n\tpass\nfunc b():\n\tpass",
        "func _init():\n\tpass\n\n\nfunc b():\n\tpass\n"
    )]
    #[case(
        "func a():\n\tpass\nfunc _init():\n\tpass",
        "func a():\n\tpass\n\n\nfunc _init():\n\tpass\n"
    )]
    #[case(
        "class A:\n\tpass\nfunc b():\n\tpass",
        "class A:\n\tpass\n\n\nfunc b():\n\tpass\n"
    )]
    #[case(
        "func a():\n\tpass\nclass B:\n\tpass",
        "func a():\n\tpass\n\n\nclass B:\n\tpass\n"
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
