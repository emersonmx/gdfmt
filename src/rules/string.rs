use crate::node::get_node_text;
use tree_sitter::Node;

const SINGLE_QUOTE: &str = "\'";
const ESCAPED_SINGLE_QUOTE: &str = "\\\'";
const DOUBLE_QUOTE: &str = "\"";
const ESCAPED_DOUBLE_QUOTE: &str = "\\\"";

pub fn apply(node: Node, source: &str, _indent_level: usize) -> String {
    let raw_text = get_node_text(node, source);
    let surround_quote = raw_text.chars().next().unwrap_or('"');
    let text = raw_text.trim_matches(surround_quote);
    let has_single_quote = text.contains(SINGLE_QUOTE);
    let has_double_quote = text.contains(DOUBLE_QUOTE);

    let (surround_quote, text): (&str, &str) = match (
        surround_quote.to_string().as_ref(),
        has_single_quote,
        has_double_quote,
    ) {
        (SINGLE_QUOTE, true, _) => (
            DOUBLE_QUOTE,
            &text
                .replace(ESCAPED_SINGLE_QUOTE, SINGLE_QUOTE)
                .replace(ESCAPED_DOUBLE_QUOTE, DOUBLE_QUOTE)
                .replace(DOUBLE_QUOTE, ESCAPED_DOUBLE_QUOTE),
        ),
        (SINGLE_QUOTE, _, true) => (
            SINGLE_QUOTE,
            &text.replace(ESCAPED_DOUBLE_QUOTE, DOUBLE_QUOTE),
        ),
        (SINGLE_QUOTE, _, _) => (DOUBLE_QUOTE, text),

        (DOUBLE_QUOTE, true, _) => (
            DOUBLE_QUOTE,
            &text
                .replace(ESCAPED_SINGLE_QUOTE, SINGLE_QUOTE)
                .replace(ESCAPED_DOUBLE_QUOTE, DOUBLE_QUOTE)
                .replace(DOUBLE_QUOTE, ESCAPED_DOUBLE_QUOTE),
        ),
        (DOUBLE_QUOTE, _, true) => (
            SINGLE_QUOTE,
            &text.replace(ESCAPED_DOUBLE_QUOTE, DOUBLE_QUOTE),
        ),
        _ => (DOUBLE_QUOTE, text),
    };

    let mut output = String::new();

    output.push_str(surround_quote);
    output.push_str(text);
    output.push_str(surround_quote);

    output
}

#[cfg(test)]
mod tests {
    use crate::format_code;
    use rstest::*;

    #[rstest]
    #[case(r#"var a = "hello world""#, "var a = \"hello world\"\n")]
    #[case("var b = 'hello world'", "var b = \"hello world\"\n")]
    #[case(r#"var c = 'hello" "world'"#, "var c = 'hello\" \"world'\n")]
    #[case("var d = 'hello\\\' \\\'world'", "var d = \"hello' 'world\"\n")]
    #[case(r#"var e = "hello\" \"world""#, "var e = 'hello\" \"world'\n")]
    #[case(
        r#"var f = 'hello\"\' \'\"world'"#,
        "var f = \"hello\\\"' '\\\"world\"\n"
    )]
    #[case(
        r#"var g = "hello\"\' \'\"world""#,
        "var g = \"hello\\\"' '\\\"world\"\n"
    )]
    #[case(
        r#"var h = 'hello\"\'\"\' \'\"\'\"world'"#,
        "var h = \"hello\\\"'\\\"' '\\\"'\\\"world\"\n"
    )]
    #[case(
        r#"var i = "hello\"\'\"\' \'\"\'\"world""#,
        "var i = \"hello\\\"'\\\"' '\\\"'\\\"world\"\n"
    )]
    fn fix_string_quotes(#[case] source_input: &str, #[case] expected_output: &str) {
        let formatted = format_code(source_input).unwrap();

        assert_eq!(formatted, expected_output);
    }
}
