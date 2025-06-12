use tree_sitter::Node;

const KINDS_WITH_TWO_LINES_BETWEEN: [&str; 3] = [
    "function_definition",
    "class_definition",
    "constructor_definition",
];

pub fn get_node_text<'a>(node: Node<'a>, source: &'a str) -> &'a str {
    &source[node.byte_range()]
}

pub fn get_root_gap_lines(node: Node, source: &str) -> String {
    let prev_node = node.prev_sibling();
    let lines = match (
        KINDS_WITH_TWO_LINES_BETWEEN.contains(&node.kind()),
        prev_node,
    ) {
        (true, Some(prev)) => {
            if prev.kind() == "comment" {
                &get_gap_lines(node, source)
            } else {
                "\n\n"
            }
        }
        _ => &get_gap_lines(node, source),
    };
    lines.to_string()
}

pub fn get_gap_lines(node: Node, source: &str) -> String {
    let previous = node.prev_sibling();
    let gap_start_byte = if let Some(prev_node) = previous {
        prev_node.end_byte()
    } else {
        node.start_byte()
    };
    let gap_end_byte = node.start_byte();
    let gap_str = &source[gap_start_byte..gap_end_byte];
    let gap_lines: String = gap_str.chars().filter(|c| *c == '\n').collect();
    let lines = if gap_lines.len() > 1 { "\n" } else { "" };
    lines.to_string()
}
