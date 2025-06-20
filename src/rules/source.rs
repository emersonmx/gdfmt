use crate::text::force_end_line;
use tree_sitter::Node;

pub fn apply(node: Node, source: &str, indent_level: usize) -> String {
    let mut output = String::new();

    for child in node.children(&mut node.walk()) {
        let child_output = super::apply(child, source, indent_level);
        output.push_str(&child_output);
    }

    force_end_line(&mut output);

    output
}
