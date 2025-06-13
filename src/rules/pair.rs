use tree_sitter::Node;

pub fn apply(node: Node, source: &str, indent_level: usize) -> String {
    let mut output = String::new();

    for (i, child) in node.children(&mut node.walk()).enumerate() {
        let text = &super::apply(child, source, indent_level);
        let (text, space): (&str, &str) = match child.kind() {
            _ if i == 0 => (text, ""),
            ":" => (text, ""),
            _ => (text, " "),
        };
        output.push_str(space);
        output.push_str(text);
    }

    output
}
