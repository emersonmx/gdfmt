use crate::text::indent_by;
use tree_sitter::Node;

pub fn apply(node: Node, source: &str, indent_level: usize) -> String {
    let mut output = String::new();

    output.push(':');
    output.push('\n');
    indent_by(&mut output, indent_level);

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
