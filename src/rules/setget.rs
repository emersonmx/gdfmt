use crate::text::make_indent;
use tree_sitter::Node;

pub fn apply(node: Node, source: &str, indent_level: usize) -> String {
    let indent = make_indent(indent_level);
    let mut output = String::new();

    output.push(':');
    output.push('\n');
    output.push_str(&indent);
    for (i, child) in node.children(&mut node.walk()).enumerate() {
        let text = match child.kind() {
            _ if i == 0 => "",
            "=" => " = ",
            _ => &super::apply(child, source, indent_level),
        };

        output.push_str(text);
    }

    output
}
