use tree_sitter::Node;

pub fn apply(node: Node, source: &str, indent_level: usize) -> String {
    let mut output = String::new();

    for child in node.children(&mut node.walk()) {
        let (text, space): (&str, &str) = match child.kind() {
            "&&" => ("and", " "),
            "||" => ("or", " "),
            _ => {
                let space = if child.prev_sibling().is_some() {
                    " "
                } else {
                    ""
                };
                (&super::apply(child, source, indent_level), space)
            }
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
    #[case("var a = 1+1", "var a = 1 + 1\n")]
    fn force_spaces_rules(#[case] source_input: &str, #[case] expected_output: &str) {
        let formatted = format_code(source_input).unwrap();

        assert_eq!(formatted, expected_output);
    }

    #[rstest]
    #[case("var a = foo&&bar", "var a = foo and bar\n")]
    #[case("var a = foo and bar", "var a = foo and bar\n")]
    #[case("var b = foo||bar", "var b = foo or bar\n")]
    #[case("var b = foo or bar", "var b = foo or bar\n")]
    #[case("var c = foo&&bar||!baz", "var c = foo and bar or not baz\n")]
    #[case("var c = foo and bar or not baz", "var c = foo and bar or not baz\n")]
    fn prefer_the_plain_english_versions_of_boolean_operators(
        #[case] source_input: &str,
        #[case] expected_output: &str,
    ) {
        let formatted = format_code(source_input).unwrap();

        assert_eq!(formatted, expected_output);
    }
}
