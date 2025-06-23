use crate::{node::get_gap_lines, text::indent_by};
use tree_sitter::Node;

pub fn apply(node: Node, source: &str, indent_level: usize) -> String {
    let gap_lines = get_gap_lines(node, source);
    let mut output = String::new();

    output.push_str(&gap_lines);
    indent_by(&mut output, indent_level);

    for child in node.children(&mut node.walk()) {
        let child_apply_fn = || super::apply(child, source, indent_level);
        let (text, space): (&str, &str) = match child.kind() {
            "enum" => (&child_apply_fn(), ""),
            _ => (&child_apply_fn(), " "),
        };
        output.push_str(space);
        output.push_str(text);
    }

    output.push('\n');

    output
}

#[cfg(test)]
mod tests {
    use crate::format_code;
    use rstest::*;

    #[rstest]
    #[case("enum {ONE}", "enum {\n\tONE,\n}\n")]
    #[case("enum A{ONE}", "enum A {\n\tONE,\n}\n")]
    fn force_spaces_rules(#[case] source_input: &str, #[case] expected_output: &str) {
        let formatted = format_code(source_input).unwrap();

        assert_eq!(formatted, expected_output);
    }
}
