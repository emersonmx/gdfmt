use tree_sitter::Node;

pub fn apply(node: Node, source: &str, indent_level: usize) -> String {
    let mut output = String::new();

    for child in node.children(&mut node.walk()) {
        let next_kind = child.next_sibling().map(|ns| ns.kind());
        let prev_kind = child.prev_sibling().map(|ps| ps.kind());
        let text = &super::apply(child, source, indent_level);
        let (text, space): (&str, &str) = match child.kind() {
            "[" => (text, ""),
            "]" if prev_kind == Some("[") => (text, ""),
            "," if next_kind == Some("]") => ("", ""),
            "," => (text, ""),
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
    #[case("var a = [ 1, 2, 3 ]", "var a = [ 1, 2, 3 ]\n")]
    #[case("var a=[1,2,3]", "var a = [ 1, 2, 3 ]\n")]
    #[case("var  a  =  [  1  ,  2  ,  3  ]", "var a = [ 1, 2, 3 ]\n")]
    #[case("var a=[]", "var a = []\n")]
    #[case("var a=[1]", "var a = [ 1 ]\n")]
    #[case("var a=[ 1 ]", "var a = [ 1 ]\n")]
    #[case("var a=[ 1, ]", "var a = [ 1 ]\n")]
    fn trim_whitespaces(#[case] source_input: &str, #[case] expected_output: &str) {
        let formatted = format_code(source_input).unwrap();

        assert_eq!(
            formatted, expected_output,
            "Failed for input: {:?}",
            source_input
        );
    }
}
