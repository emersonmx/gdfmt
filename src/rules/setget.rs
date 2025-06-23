use crate::text::indent_by;
use tree_sitter::Node;

#[allow(dead_code)]
pub fn apply(node: Node, source: &str, indent_level: usize) -> String {
    let mut output = String::new();

    output.push(':');
    output.push('\n');
    indent_by(&mut output, indent_level);

    for child in node.children(&mut node.walk()) {
        let text = match child.kind() {
            _ if child.prev_sibling().is_none() => "",
            "=" => " = ",
            _ => &super::apply(child, source, indent_level),
        };

        output.push_str(text);
    }

    output
}

// #[cfg(test)]
// mod tests {
//     use crate::format_code;
//     use rstest::*;
//
//     #[rstest]
//     #[case("var a = true:\n\tset=set_a", "var a = true:\n\tset = set_a\n")]
//     #[case("var a = true:\n\tget=get_a", "var a = true:\n\tget = get_a\n")]
//     fn force_spaces_rules(#[case] source_input: &str, #[case] expected_output: &str) {
//         let formatted = format_code(source_input).unwrap();
//
//         assert_eq!(formatted, expected_output);
//     }
// }
