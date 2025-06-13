pub fn force_end_line(buffer: &mut String) {
    while buffer.ends_with("\n") {
        buffer.pop();
    }
    buffer.push('\n');
}

pub fn indent_by(buffer: &mut String, indent_level: usize) {
    for _ in 0..indent_level {
        buffer.push('\t');
    }
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

        assert_eq!(input, output);
    }

    #[rstest]
    #[case(0, "")]
    #[case(1, "\t")]
    #[case(2, "\t\t")]
    #[case(3, "\t\t\t")]
    fn make_indent_by_level(#[case] input: usize, #[case] output: &str) {
        let mut buf = String::new();

        indent_by(&mut buf, input);

        assert_eq!(buf, output);
    }
}
