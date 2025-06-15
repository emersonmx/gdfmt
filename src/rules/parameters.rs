use tree_sitter::Node;

pub fn apply(node: Node, source: &str, indent_level: usize) -> String {
    match node.kind() {
        "parameters" => apply_parameters_rules(node, source, indent_level),
        "default_parameter" => apply_default_parameter_rules(node, source, indent_level),
        _ => unreachable!(),
    }
}

fn apply_parameters_rules(node: Node, source: &str, indent_level: usize) -> String {
    let mut output = String::new();

    for child in node.children(&mut node.walk()) {
        let prev_kind = child.prev_sibling().map(|ps| ps.kind());
        let text = &super::apply(child, source, indent_level);
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

fn apply_default_parameter_rules(node: Node, source: &str, indent_level: usize) -> String {
    let mut output = String::new();

    for child in node.children(&mut node.walk()) {
        let text = &super::apply(child, source, indent_level);
        let (text, space): (&str, &str) = match child.kind() {
            "identifier" => (text, ""),
            _ => (text, ""),
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
    #[case("func a(): pass", "func a():\n\tpass\n")]
    #[case("func a(p1): pass", "func a(p1):\n\tpass\n")]
    #[case("func a(p1,p2): pass", "func a(p1, p2):\n\tpass\n")]
    #[case("func a(p1,p2 = 42): pass", "func a(p1, p2=42):\n\tpass\n")]
    #[case("func a(p1 = 24,p2): pass", "func a(p1=24, p2):\n\tpass\n")]
    #[case("func a(p1 = 24,p2 = 42): pass", "func a(p1=24, p2=42):\n\tpass\n")]
    fn force_spaces_rules(#[case] source_input: &str, #[case] expected_output: &str) {
        let formatted = format_code(source_input).unwrap();

        assert_eq!(formatted, expected_output);
    }
}
