use tree_sitter::Node;

pub fn apply(node: Node, source: &str, indent_level: usize) -> String {
    match node.kind() {
        "annotations" => apply_annotations_rules(node, source, indent_level),
        "annotation" => apply_annotation_rules(node, source, indent_level),
        _ => unreachable!(),
    }
}

fn apply_annotations_rules(node: Node, source: &str, indent_level: usize) -> String {
    let mut output = String::new();

    for (i, child) in node.children(&mut node.walk()).enumerate() {
        let text = &super::apply(child, source, indent_level);
        let (text, space): (&str, &str) = match child.kind() {
            _ if i == 0 => (text, ""),
            "annotation" => (text, " "),
            _ => (text, ""),
        };
        output.push_str(space);
        output.push_str(text);
    }

    output
}

fn apply_annotation_rules(node: Node, source: &str, indent_level: usize) -> String {
    let mut output = String::new();

    for child in node.children(&mut node.walk()) {
        let text = &super::apply(child, source, indent_level);
        let (text, space): (&str, &str) = match child.kind() {
            "@" => (text, ""),
            "annotation" => (text, " "),
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
    #[case("@export var a = 0", "@export var a = 0\n")]
    #[case("@export @onready var a = 0", "@export @onready var a = 0\n")]
    #[case("  @export  var a = 0", "@export var a = 0\n")]
    #[case("  @onready  @export  var a = 0", "@onready @export var a = 0\n")]
    #[case("  @  onready  @  export  var a = 0", "@onready @export var a = 0\n")]
    fn trim_whitespaces(#[case] source_input: &str, #[case] expected_output: &str) {
        let formatted = format_code(source_input).unwrap();

        assert_eq!(
            formatted, expected_output,
            "Failed for input: {:?}",
            source_input
        );
    }
}
