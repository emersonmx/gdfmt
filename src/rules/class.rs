use crate::node::{get_gap_lines, get_root_gap_lines};
use tree_sitter::Node;

pub fn apply(node: Node, source: &str, indent_level: usize) -> String {
    let parent_kind = node.parent().map(|n| n.kind());
    let gap_lines = match parent_kind {
        Some("source") => get_root_gap_lines(node, source),
        _ => get_gap_lines(node, source),
    };
    let mut output = String::new();

    output.push_str(&gap_lines);

    for (i, child) in node.children(&mut node.walk()).enumerate() {
        let text = &super::apply(child, source, indent_level);
        let (text, space): (&str, &str) = match child.kind() {
            _ if i == 0 => (text, ""),
            ":" => (text, ""),
            "body" => (
                &format!("\n{}", &super::apply(child, source, indent_level + 1)),
                "",
            ),
            _ => (text, " "),
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
    #[case("class     MyClass    :    pass", "class MyClass:\n\tpass\n")]
    #[case(
        "class MyClass:\n\tfunc a( b = 24 ,  c  =  42 ):pass",
        "class MyClass:\n\tfunc a(b=24, c=42):\n\t\tpass\n"
    )]
    fn trim_whitespaces(#[case] source_input: &str, #[case] expected_output: &str) {
        let formatted = format_code(source_input).unwrap();

        assert_eq!(formatted, expected_output);
    }

    #[rstest]
    #[case(
        "class A:\n\tpass\nfunc b():\n\tpass",
        "class A:\n\tpass\n\n\nfunc b():\n\tpass\n"
    )]
    #[case(
        "func a():\n\tpass\nclass B:\n\tpass",
        "func a():\n\tpass\n\n\nclass B:\n\tpass\n"
    )]
    #[case(
        "# a comment\n\nclass A:\n\tpass\nclass B:\n\tpass",
        "# a comment\n\nclass A:\n\tpass\n\n\nclass B:\n\tpass\n"
    )]
    fn keep_two_lines_between(#[case] source_input: &str, #[case] expected_output: &str) {
        let formatted = format_code(source_input).unwrap();

        assert_eq!(formatted, expected_output);
    }
}
