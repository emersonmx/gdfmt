use tree_sitter::Node;

pub fn apply(node: Node, source: &str, indent_level: usize) -> String {
    match node.kind() {
        "annotations" => apply_annotations_rules(node, source, indent_level),
        "annotation" => apply_annotation_rules(node, source, indent_level),
        _ => super::apply(node, source, indent_level),
    }
}

fn apply_annotations_rules(node: Node, source: &str, indent_level: usize) -> String {
    let mut output = String::new();

    for child in node.children(&mut node.walk()) {
        let child_apply_fn = || apply(child, source, indent_level);
        let (text, space): (&str, &str) = match child.kind() {
            _ if child.prev_sibling().is_none() => (&child_apply_fn(), ""),
            "annotation" => (&child_apply_fn(), " "),
            _ => (&child_apply_fn(), ""),
        };
        output.push_str(space);
        output.push_str(text);
    }

    output
}

fn apply_annotation_rules(node: Node, source: &str, indent_level: usize) -> String {
    let mut output = String::new();

    for child in node.children(&mut node.walk()) {
        let child_apply_fn = || apply(child, source, indent_level);
        let (text, space): (&str, &str) = match child.kind() {
            "@" => (&child_apply_fn(), ""),
            "annotation" => (&child_apply_fn(), " "),
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
    #[case("@export var a = 0", "@export var a = 0\n")]
    #[case("@export @onready var b = 0", "@export @onready var b = 0\n")]
    #[case("  @export  var c = 0", "@export var c = 0\n")]
    #[case("  @onready  @export  var d = 0", "@onready @export var d = 0\n")]
    #[case("  @  onready  @  export  var e = 0", "@onready @export var e = 0\n")]
    fn enforce_spacing_rules(#[case] source_input: &str, #[case] expected_output: &str) {
        let formatted = format_code(source_input).unwrap();

        assert_eq!(formatted, expected_output);
    }
}
