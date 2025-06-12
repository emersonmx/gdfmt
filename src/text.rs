pub fn force_end_line(text: &mut String) {
    while text.ends_with("\n") {
        text.pop();
    }
    text.push('\n');
}

pub fn make_indent(indent_level: usize) -> String {
    "\t".repeat(indent_level)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("", "\n")]
    #[case("\n", "\n")]
    #[case("\n\n", "\n")]
    #[case("\n\n\n", "\n")]
    #[case("text\n\n\n", "text\n")]
    fn should_have_only_one_end_line(#[case] input: &str, #[case] output: &str) {
        let mut input = input.to_string();
        force_end_line(&mut input);

        assert_eq!(input, output, "Failed for input: {:?}", input);
    }

    #[rstest]
    #[case(0, "")]
    #[case(1, "\t")]
    #[case(2, "\t\t")]
    #[case(3, "\t\t\t")]
    fn make_indent_by_level(#[case] input: usize, #[case] output: &str) {
        let indent = make_indent(input);

        assert_eq!(indent, output, "Failed for input: {:?}", input);
    }
}
