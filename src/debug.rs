use tree_sitter::Node;

#[allow(dead_code)]
pub fn node_to_string(node: Node, source: &str, indent_level: usize) -> String {
    let indent = "    ".repeat(indent_level);
    let text = &source[node.byte_range()];
    let kind = node.kind();
    let content_display = if !text.contains('\n') {
        format!(" `{}`", text.trim())
    } else {
        String::new()
    };

    let mut output = String::new();

    output.push_str(&indent);
    output.push('(');
    output.push_str(kind);
    output.push_str(&content_display);
    output.push(')');
    output.push('\n');

    for child in node.children(&mut node.walk()) {
        let child_output = node_to_string(child, source, indent_level + 1);
        output.push_str(&child_output);
    }

    output
}
