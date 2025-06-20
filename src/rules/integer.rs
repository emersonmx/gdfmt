use crate::node::get_node_text;
use tree_sitter::Node;

pub fn apply(node: Node, source: &str, _indent_level: usize) -> String {
    let text = &get_node_text(node, source).to_lowercase();
    let mut output = String::new();

    output.push_str(text);

    output
}

#[cfg(test)]
mod tests {
    use crate::format_code;
    use rstest::*;

    #[rstest]
    #[case("var a = 0xFB8C0B", "var a = 0xfb8c0b\n")]
    fn force_lowercase_hexadecimal(#[case] source_input: &str, #[case] expected_output: &str) {
        let formatted = format_code(source_input).unwrap();

        assert_eq!(formatted, expected_output);
    }
}
