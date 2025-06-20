use crate::node::get_node_text;
use tree_sitter::Node;

pub fn apply(node: Node, source: &str, _indent_level: usize) -> String {
    let text = get_node_text(node, source);
    let leading_zero = if text.starts_with(".") { "0" } else { "" };
    let trailing_zero = if text.ends_with(".") { "0" } else { "" };

    let mut output = String::new();

    output.push_str(leading_zero);
    output.push_str(text);
    output.push_str(trailing_zero);

    output
}

#[cfg(test)]
mod tests {
    use crate::format_code;
    use rstest::*;

    #[rstest]
    #[case("var a = .234", "var a = 0.234\n")]
    #[case("var b = 13.", "var b = 13.0\n")]
    fn force_leading_or_trailing_zero(#[case] source_input: &str, #[case] expected_output: &str) {
        let formatted = format_code(source_input).unwrap();

        assert_eq!(formatted, expected_output);
    }
}
