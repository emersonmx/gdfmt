use crate::text::indent_by;
use tree_sitter::Node;

pub fn apply(node: Node, source: &str, indent_level: usize) -> String {
    match node.kind() {
        "setget" => apply_setget_rules(node, source, indent_level),
        "set_body" | "get_body" => apply_setget_body_rules(node, source, indent_level),
        "body" => super::apply(node, source, indent_level + 1),
        _ => super::apply(node, source, indent_level),
    }
}

fn apply_setget_rules(node: Node, source: &str, indent_level: usize) -> String {
    let mut output = String::new();

    for child in node.children(&mut node.walk()) {
        let prev_kind = child.prev_sibling().map(|ps| ps.kind());
        let child_apply_fn = || apply(child, source, indent_level);
        let (text, space): (&str, &str) = match child.kind() {
            ":" => {
                output.push_str(&child_apply_fn());
                output.push('\n');
                ("", "")
            }
            "get" | "set" => {
                if prev_kind == Some(":") {
                    indent_by(&mut output, indent_level);
                } else {
                    output.push(' ');
                }
                (&child_apply_fn(), "")
            }
            "=" | "getter" | "setter" => (&child_apply_fn(), " "),
            "," => (&child_apply_fn(), ""),
            "set_body" | "get_body" => (&child_apply_fn(), ""),
            _ => (&child_apply_fn(), ""),
        };
        output.push_str(space);
        output.push_str(text);
    }

    output
}

fn apply_setget_body_rules(node: Node, source: &str, indent_level: usize) -> String {
    let mut output = String::new();

    indent_by(&mut output, indent_level);

    for child in node.children(&mut node.walk()) {
        let child_apply_fn = || apply(child, source, indent_level);
        let (text, space): (&str, &str) = match child.kind() {
            "set" | "get" => (&child_apply_fn(), ""),
            ":" => {
                output.push_str(&child_apply_fn());
                output.push('\n');
                ("", "")
            }
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
    #[case("var a:\n\tset=set_a", "var a:\n\tset = set_a\n")]
    #[case("var a:\n\tget=get_a", "var a:\n\tget = get_a\n")]
    #[case(
        "var b:\n\tset=set_b, get=get_b",
        "var b:\n\tset = set_b, get = get_b\n"
    )]
    #[case(
        "var b:\n\tget=get_b, set=set_b",
        "var b:\n\tget = get_b, set = set_b\n"
    )]
    #[case(
        "var c:\n\tset(value):\n\t\tc = value",
        "var c:\n\tset(value):\n\t\tc = value\n"
    )]
    #[case("var c:\n\tget:\n\t\treturn 42", "var c:\n\tget:\n\t\treturn 42\n")]
    #[case(
        "var d:\n\tset(value):\n\t\td = value\n\tget:\n\t\treturn 42",
        "var d:\n\tset(value):\n\t\td = value\n\tget:\n\t\treturn 42\n"
    )]
    fn enforce_spacing_rules(#[case] source_input: &str, #[case] expected_output: &str) {
        let formatted = format_code(source_input).unwrap();

        assert_eq!(formatted, expected_output);
    }
}
