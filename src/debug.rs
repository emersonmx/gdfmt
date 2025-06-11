use tree_sitter::Node;

pub fn print_tree(node: Node, source: &str, indent_level: usize) -> String {
    let mut output = String::new();
    let indent = "    ".repeat(indent_level);
    let text = &source[node.byte_range()];
    let kind = node.kind();

    let content_display = if !text.contains('\n') {
        format!(" `{}`", text.trim())
    } else {
        String::new()
    };

    output.push_str(&format!("{}({}{})", indent, kind, content_display));
    output.push('\n');

    for child in node.children(&mut node.walk()) {
        let child_output = print_tree(child, source, indent_level + 1);
        output.push_str(&child_output);
    }

    output
}
