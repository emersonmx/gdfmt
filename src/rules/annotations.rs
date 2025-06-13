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
