use tree_sitter::Node;

pub fn apply(node: Node, source: &str, indent_level: usize) -> String {
    let mut output = String::new();

    for child in node.children(&mut node.walk()) {
        let child_apply_fn = || super::apply(child, source, indent_level);
        let (text, space): (&str, &str) = match child.kind() {
            _ if child.prev_sibling().is_none() => (&child_apply_fn(), ""),
            "=" => (&child_apply_fn(), " "),
            _ => (&child_apply_fn(), " "),
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
    #[case("enum A{ONE}", "enum A {\n\tONE,\n}\n")]
    #[case("enum B{F=0}", "enum B {\n\tF = 0,\n}\n")]
    fn force_spaces_rules(#[case] source_input: &str, #[case] expected_output: &str) {
        let formatted = format_code(source_input).unwrap();

        assert_eq!(formatted, expected_output);
    }
}
