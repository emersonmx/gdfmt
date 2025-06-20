use crate::node::get_node_text;
use tree_sitter::Node;

const MIN_LENGTH: usize = 6;
const DEFAULT_GROUP_LENGTH: usize = 3;
const BINARY_PREFIX: &str = "0b";
const BINARY_GROUP_LENGTH: usize = 4;
const HEXADECIMAL_PREFIX: &str = "0x";
const HEXADECIMAL_GROUP_LENGTH: usize = 4;

pub fn apply(node: Node, source: &str, _indent_level: usize) -> String {
    let text = &get_node_text(node, source).to_lowercase();

    let (prefix, text): (&str, &str) = match (
        text.starts_with(BINARY_PREFIX),
        text.starts_with(HEXADECIMAL_PREFIX),
    ) {
        (true, _) => (
            BINARY_PREFIX,
            &format_with_underscores(text.trim_start_matches(BINARY_PREFIX), BINARY_GROUP_LENGTH),
        ),
        (_, true) => (
            HEXADECIMAL_PREFIX,
            &format_with_underscores(
                text.trim_start_matches(HEXADECIMAL_PREFIX),
                HEXADECIMAL_GROUP_LENGTH,
            ),
        ),
        (false, false) if text.len() > MIN_LENGTH => {
            ("", &format_with_underscores(text, DEFAULT_GROUP_LENGTH))
        }
        _ => ("", text),
    };

    let mut output = String::new();

    output.push_str(prefix);
    output.push_str(text);

    output
}

fn format_with_underscores(s: &str, group_length: usize) -> String {
    let len = s.len();
    if len <= group_length {
        return s.to_string();
    }

    let mut result = String::with_capacity(len + (len - 1) / group_length);

    let first_group_len = len % group_length;
    let first_group_len = if first_group_len == 0 {
        group_length
    } else {
        first_group_len
    };

    result.push_str(&s[0..first_group_len]);

    for i in (first_group_len..len).step_by(group_length) {
        result.push('_');
        result.push_str(&s[i..(i + group_length)]);
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::format_code;
    use rstest::*;

    #[rstest]
    #[case("var a = 999999", "var a = 999999\n")]
    #[case("var b = 12345", "var b = 12345\n")]
    #[case("var c = 123", "var c = 123\n")]
    #[case("var d = 0", "var d = 0\n")]
    #[case("var e = 1234567890", "var e = 1_234_567_890\n")]
    #[case("var f = 0x1a3f", "var f = 0x1a3f\n")]
    #[case("var g = 0x1a3f6d", "var g = 0x1a_3f6d\n")]
    #[case("var h = 0xfffff8f80000", "var h = 0xffff_f8f8_0000\n")]
    #[case("var i = 0b1101", "var i = 0b1101\n")]
    #[case("var j = 0b110100", "var j = 0b11_0100\n")]
    #[case("var k = 0b110100101010", "var k = 0b1101_0010_1010\n")]
    fn use_underscore_to_make_large_numbers_more_readable(
        #[case] source_input: &str,
        #[case] expected_output: &str,
    ) {
        let formatted = format_code(source_input).unwrap();

        assert_eq!(formatted, expected_output);
    }
}
