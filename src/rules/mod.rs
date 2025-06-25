mod annotations;
mod array;
mod binary_operator;
mod body;
mod class;
mod dictionary;
mod enum_definition;
mod float;
mod function;
mod integer;
mod parameters;
mod parenthesized_expression;
mod setget;
mod source;
mod string;
mod unary_operator;
mod variable;

use crate::node::{get_gap_lines, get_node_text};
use crate::text::indent_by;
use tree_sitter::Node;

pub fn apply(node: Node, source: &str, indent_level: usize) -> String {
    match node.kind() {
        // call inner nodes
        "source" => source::apply(node, source, indent_level),
        "body" => body::apply(node, source, indent_level),

        // with trailing line
        "variable_statement" => variable::apply(node, source, indent_level),
        "function_definition" | "constructor_definition" => {
            function::apply(node, source, indent_level)
        }
        "class_definition" => class::apply(node, source, indent_level),
        "enum_definition" | "enumerator_list" | "enumerator" => {
            enum_definition::apply(node, source, indent_level)
        }

        "class_name_statement"
        | "extends_statement"
        | "comment"
        | "signal_statement"
        | "expression_statement"
        | "pass_statement"
        | "return_statement"
        | "if_statement"
        | "for_statement" => apply_fallback_rules(node, source, indent_level),

        // without leading/trailing whitespace
        // "setget" => setget::apply(node, source, indent_level),
        "parameters" | "default_parameter" => parameters::apply(node, source, indent_level),
        "parenthesized_expression" => parenthesized_expression::apply(node, source, indent_level),
        "annotations" => annotations::apply(node, source, indent_level),
        "unary_operator" => unary_operator::apply(node, source, indent_level),
        "binary_operator" => binary_operator::apply(node, source, indent_level),
        "integer" => integer::apply(node, source, indent_level),
        "float" => float::apply(node, source, indent_level),
        "string" => string::apply(node, source, indent_level),
        "array" => array::apply(node, source, indent_level),
        "dictionary" | "pair" => dictionary::apply(node, source, indent_level),

        _ => get_node_text(node, source).to_string(),
    }
}

fn apply_fallback_rules(node: Node, source: &str, indent_level: usize) -> String {
    let text = get_node_text(node, source);
    let gap_lines = get_gap_lines(node, source);
    let mut output = String::new();

    output.push_str(&gap_lines);
    indent_by(&mut output, indent_level);
    output.push_str(text.trim());
    output.push('\n');

    output
}

#[cfg(test)]
mod tests {
    use crate::format_code;
    use rstest::*;

    #[rstest]
    #[case("# comment\n# comment", "# comment\n# comment\n")]
    #[case("# comment\n\n# comment", "# comment\n\n# comment\n")]
    #[case("# comment\n\n\n# comment", "# comment\n\n# comment\n")]
    #[case("# comment\n  \n  \n# comment", "# comment\n\n# comment\n")]
    fn keep_line(#[case] source_input: &str, #[case] expected_output: &str) {
        let formatted = format_code(source_input).unwrap();

        assert_eq!(formatted, expected_output);
    }

    #[rstest]
    #[case("print(42)", "print(42)\n")]
    #[case("print( 42 )", "print( 42 )\n")]
    fn keep_text(#[case] source_input: &str, #[case] expected_output: &str) {
        let formatted = format_code(source_input).unwrap();

        assert_eq!(formatted, expected_output);
    }
}
