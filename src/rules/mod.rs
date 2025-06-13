mod annotations;
mod array;
mod body;
mod class;
mod dictionary;
mod function;
mod node;
mod pair;
mod parameters;
mod setget;
mod source;
mod variable;

use crate::text::indent_by;
use node::{get_gap_lines, get_node_text};
use tree_sitter::Node;

pub fn apply(node: Node, source: &str, indent_level: usize) -> String {
    match node.kind() {
        // with trailing line
        "source" => source::apply(node, source, indent_level),
        "function_definition" | "constructor_definition" => {
            function::apply(node, source, indent_level)
        }
        "class_definition" => class::apply(node, source, indent_level),
        "variable_statement" => variable::apply(node, source, indent_level),
        "class_name_statement"
        | "extends_statement"
        | "comment"
        | "signal_statement"
        | "expression_statement"
        | "pass_statement"
        | "if_statement" => apply_trailing_line_rules(node, source, indent_level),
        "body" => body::apply(node, source, indent_level),
        // without trailing whitespace
        "setget" => setget::apply(node, source, indent_level),
        "parameters" | "default_parameter" => parameters::apply(node, source, indent_level),
        "annotations" | "annotation" => annotations::apply(node, source, indent_level),
        "array" => array::apply(node, source, indent_level),
        "dictionary" => dictionary::apply(node, source, indent_level),
        "pair" => pair::apply(node, source, indent_level),
        _ => get_node_text(node, source).to_string(),
    }
}

fn apply_trailing_line_rules(node: Node, source: &str, indent_level: usize) -> String {
    let text = get_node_text(node, source); // TODO: try apply
    let gap_lines = get_gap_lines(node, source);
    let mut output = String::new();

    output.push_str(&gap_lines);
    indent_by(&mut output, indent_level);
    output.push_str(text.trim());
    output.push('\n');

    output
}
