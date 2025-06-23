use tree_sitter::Node;

pub fn apply(node: Node, source: &str, indent_level: usize) -> String {
    let mut output = String::new();

    for child in node.children(&mut node.walk()) {
        let child_apply_fn = || super::apply(child, source, indent_level);
        let (text, space): (&str, &str) = match child.kind() {
            "!" | "not" => ("not ", ""),
            "+" => ("", ""),
            _ => (&child_apply_fn(), ""),
        };
        output.push_str(space);
        output.push_str(text);
    }

    output
}

#[cfg(test)]
mod tests {
    use crate::format_code;
    use rstest::*;

    #[rstest]
    #[case("var a=-1", "var a = -1\n")]
    fn enforce_spacing_rules(#[case] source_input: &str, #[case] expected_output: &str) {
        let formatted = format_code(source_input).unwrap();

        assert_eq!(formatted, expected_output);
    }

    #[rstest]
    #[case("var a=+1", "var a = 1\n")]
    fn remove_useless_operator(#[case] source_input: &str, #[case] expected_output: &str) {
        let formatted = format_code(source_input).unwrap();

        assert_eq!(formatted, expected_output);
    }

    #[rstest]
    #[case("var a = !true", "var a = not true\n")]
    fn prefer_the_plain_english_versions_of_boolean_operators(
        #[case] source_input: &str,
        #[case] expected_output: &str,
    ) {
        let formatted = format_code(source_input).unwrap();

        assert_eq!(formatted, expected_output);
    }
}
