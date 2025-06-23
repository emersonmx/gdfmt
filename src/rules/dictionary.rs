use tree_sitter::Node;

pub fn apply(node: Node, source: &str, indent_level: usize) -> String {
    match node.kind() {
        "dictionary" => apply_dictionary_rules(node, source, indent_level),
        "pair" => apply_pair_rules(node, source, indent_level),
        _ => unreachable!(),
    }
}

fn apply_dictionary_rules(node: Node, source: &str, indent_level: usize) -> String {
    let mut output = String::new();

    for child in node.children(&mut node.walk()) {
        let next_kind = child.next_sibling().map(|ns| ns.kind());
        let prev_kind = child.prev_sibling().map(|ps| ps.kind());
        let child_apply_fn = || super::apply(child, source, indent_level);
        let (text, space): (&str, &str) = match child.kind() {
            "{" => (&child_apply_fn(), ""),
            "}" if prev_kind == Some("{") => (&child_apply_fn(), ""),
            "}" => (&child_apply_fn(), " "),
            "," if next_kind == Some("}") => ("", ""),
            "," => (&child_apply_fn(), ""),
            _ => (&child_apply_fn(), " "),
        };
        output.push_str(space);
        output.push_str(text);
    }

    output
}

fn apply_pair_rules(node: Node, source: &str, indent_level: usize) -> String {
    let mut output = String::new();

    for child in node.children(&mut node.walk()) {
        let child_apply_fn = || super::apply(child, source, indent_level);
        let (text, space): (&str, &str) = match child.kind() {
            _ if child.prev_sibling().is_none() => (&child_apply_fn(), ""),
            ":" => (&child_apply_fn(), ""),
            _ => (&child_apply_fn(), " "),
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
    #[case(
        r#"var a = { "one": 1, "two": 2, "three": 3 }"#,
        "var a = { \"one\": 1, \"two\": 2, \"three\": 3 }\n"
    )]
    #[case(
        r#"var b={"one":1,"two":2,"three":3}"#,
        "var b = { \"one\": 1, \"two\": 2, \"three\": 3 }\n"
    )]
    #[case(
        r#"var  c  =  {  "one"  :  1  ,  "two"  :  2  ,  "three"  :  3  }"#,
        "var c = { \"one\": 1, \"two\": 2, \"three\": 3 }\n"
    )]
    #[case("var d={}", "var d = {}\n")]
    #[case(r#"var e={"one":1}"#, "var e = { \"one\": 1 }\n")]
    #[case(r#"var f={ "one":1 }"#, "var f = { \"one\": 1 }\n")]
    #[case(r#"var g={ "one":1, }"#, "var g = { \"one\": 1 }\n")]
    #[case("var a = { 1: 1 }", "var a = { 1: 1 }\n")]
    #[case("var b = {1:1}", "var b = { 1: 1 }\n")]
    #[case("var c = {1:1,2:2}", "var c = { 1: 1, 2: 2 }\n")]
    #[case("var d = {1:1,2:2,}", "var d = { 1: 1, 2: 2 }\n")]
    fn force_spaces_rules(#[case] source_input: &str, #[case] expected_output: &str) {
        let formatted = format_code(source_input).unwrap();

        assert_eq!(formatted, expected_output);
    }
}
