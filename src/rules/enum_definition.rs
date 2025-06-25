use crate::{node::get_gap_lines, text::indent_by};
use tree_sitter::Node;

pub fn apply(node: Node, source: &str, indent_level: usize) -> String {
    match node.kind() {
        "enum_definition" => apply_enum_definition_rules(node, source, indent_level),
        "enumerator_list" => apply_enumerator_list_rules(node, source, indent_level),
        "enumerator" => apply_enumerator_rules(node, source, indent_level),
        _ => super::apply(node, source, indent_level),
    }
}

fn apply_enum_definition_rules(node: Node, source: &str, indent_level: usize) -> String {
    let gap_lines = get_gap_lines(node, source);
    let mut output = String::new();

    output.push_str(&gap_lines);
    indent_by(&mut output, indent_level);

    for child in node.children(&mut node.walk()) {
        let child_apply_fn = || apply(child, source, indent_level);
        let (text, space): (&str, &str) = match child.kind() {
            "enum" => (&child_apply_fn(), ""),
            _ => (&child_apply_fn(), " "),
        };
        output.push_str(space);
        output.push_str(text);
    }

    output.push('\n');

    output
}

fn apply_enumerator_list_rules(node: Node, source: &str, indent_level: usize) -> String {
    let mut output = String::new();

    for child in node.children(&mut node.walk()) {
        let prev_kind = child.prev_sibling().map(|ps| ps.kind());
        let child_apply_fn = || apply(child, source, indent_level);
        let (text, space): (&str, &str) = match child.kind() {
            "{" if child.prev_sibling().is_none() => (&child_apply_fn(), ""),
            "enumerator" => {
                output.push('\n');
                indent_by(&mut output, indent_level + 1);
                (&child_apply_fn(), "")
            }
            "," => (&child_apply_fn(), ""),
            "}" if prev_kind == Some("{") => (&child_apply_fn(), ""),
            "}" => {
                if prev_kind == Some("enumerator") {
                    output.push(',');
                }
                output.push('\n');
                indent_by(&mut output, indent_level);
                (&child_apply_fn(), "")
            }
            _ => (&child_apply_fn(), ""),
        };
        output.push_str(space);
        output.push_str(text);
    }

    output
}

pub fn apply_enumerator_rules(node: Node, source: &str, indent_level: usize) -> String {
    let mut output = String::new();

    for child in node.children(&mut node.walk()) {
        let child_apply_fn = || apply(child, source, indent_level);
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
    #[case("enum {ONE}", "enum {\n\tONE,\n}\n")]
    #[case("enum A{ONE}", "enum A {\n\tONE,\n}\n")]
    #[case("enum A{ONE}", "enum A {\n\tONE,\n}\n")]
    #[case("enum B{ONE,}", "enum B {\n\tONE,\n}\n")]
    #[case("enum C{F = 0}", "enum C {\n\tF = 0,\n}\n")]
    #[case("enum D{F = 0,}", "enum D {\n\tF = 0,\n}\n")]
    #[case("enum E{ONE}", "enum E {\n\tONE,\n}\n")]
    #[case("enum F{F=0}", "enum F {\n\tF = 0,\n}\n")]
    #[case(
        "class A:\n\tenum D{F = 0,}",
        "class A:\n\tenum D {\n\t\tF = 0,\n\t}\n"
    )]
    fn enforce_spacing_rules(#[case] source_input: &str, #[case] expected_output: &str) {
        let formatted = format_code(source_input).unwrap();

        assert_eq!(formatted, expected_output);
    }
}
