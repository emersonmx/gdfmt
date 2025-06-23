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
        let child_apply_fn = || super::apply(child, source, indent_level);
        let (text, space): (&str, &str) = match child.kind() {
            "(" | ")" | "=" | "," => (&child_apply_fn(), ""),
            "identifier" | "default_parameter" if prev_kind == Some("(") => (&child_apply_fn(), ""),
            "identifier" => (&child_apply_fn(), " "),
            _ => (&child_apply_fn(), " "),
        };
        output.push_str(space);
        output.push_str(text);
    }

    output
}

fn apply_default_parameter_rules(node: Node, source: &str, indent_level: usize) -> String {
    let mut output = String::new();

    for child in node.children(&mut node.walk()) {
        let child_apply_fn = || super::apply(child, source, indent_level);
        let (text, space): (&str, &str) = match child.kind() {
            "identifier" => (&child_apply_fn(), ""),
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
    #[case("func a(): pass", "func a():\n\tpass\n")]
    #[case("func b(p1): pass", "func b(p1):\n\tpass\n")]
    #[case("func c(p1,p2): pass", "func c(p1, p2):\n\tpass\n")]
    #[case("func d(p1,p2 = 42): pass", "func d(p1, p2=42):\n\tpass\n")]
    #[case("func e(p1 = 24,p2): pass", "func e(p1=24, p2):\n\tpass\n")]
    #[case("func f(p1 = 24,p2 = 42): pass", "func f(p1=24, p2=42):\n\tpass\n")]
    fn force_spaces_rules(#[case] source_input: &str, #[case] expected_output: &str) {
        let formatted = format_code(source_input).unwrap();

        assert_eq!(formatted, expected_output);
    }
}
