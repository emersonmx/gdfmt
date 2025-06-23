use crate::text::indent_by;
use tree_sitter::Node;

pub fn apply(node: Node, source: &str, indent_level: usize) -> String {
    let mut output = String::new();

    for child in node.children(&mut node.walk()) {
        let prev_kind = child.prev_sibling().map(|ps| ps.kind());
        let child_apply_fn = || super::apply(child, source, indent_level);
        let (text, space, indent_offset): (&str, &str, Option<usize>) = match child.kind() {
            "{" => (&child_apply_fn(), "", None),
            "}" if prev_kind == Some("{") => (&child_apply_fn(), "", None),
            "}" if prev_kind == Some("enumerator") => (&child_apply_fn(), ",\n", Some(0)),
            "}" => (&child_apply_fn(), "\n", Some(0)),
            "," => (&child_apply_fn(), "", None),
            _ => (&child_apply_fn(), "\n", Some(1)),
        };
        output.push_str(space);
        if let Some(offset) = indent_offset {
            indent_by(&mut output, indent_level + offset);
        }
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
    #[case("enum B{ONE,}", "enum B {\n\tONE,\n}\n")]
    #[case("enum C{F = 0}", "enum C {\n\tF = 0,\n}\n")]
    #[case("enum D{F = 0,}", "enum D {\n\tF = 0,\n}\n")]
    #[case(
        "class A:\n\tenum D{F = 0,}",
        "class A:\n\tenum D {\n\t\tF = 0,\n\t}\n"
    )]
    fn force_spaces_rules(#[case] source_input: &str, #[case] expected_output: &str) {
        let formatted = format_code(source_input).unwrap();

        assert_eq!(formatted, expected_output);
    }
}
