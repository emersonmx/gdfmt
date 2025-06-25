use crate::node::get_gap_lines;
use crate::text::indent_by;
use tree_sitter::Node;

pub fn apply(node: Node, source: &str, indent_level: usize) -> String {
    let gap_lines = get_gap_lines(node, source);
    let mut output = String::new();

    output.push_str(&gap_lines);
    indent_by(&mut output, indent_level);

    for child in node.children(&mut node.walk()) {
        let child_apply_fn = || super::apply(child, source, indent_level + 1);
        let (text, space): (&str, &str) = match child.kind() {
            _ if child.prev_sibling().is_none() => (&child_apply_fn(), ""),
            ":" | "setget" => (&child_apply_fn(), ""),
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
    #[case("var   a   =  0  ", "var a = 0\n")]
    #[case("var\tb\t=\t0\t", "var b = 0\n")]
    #[case("var \tc \t= \t0 \t", "var c = 0\n")]
    #[case("var d = 0 \n\t", "var d = 0\n")]
    #[case("var a = 0\nvar b = 0", "var a = 0\nvar b = 0\n")]
    #[case("var b = 0\n\nvar b = 0", "var b = 0\n\nvar b = 0\n")]
    #[case("var c = 0\n\n\n\nvar b = 0", "var c = 0\n\nvar b = 0\n")]
    fn enforce_spacing_rules(#[case] source_input: &str, #[case] expected_output: &str) {
        let formatted = format_code(source_input).unwrap();

        assert_eq!(formatted, expected_output);
    }
}
