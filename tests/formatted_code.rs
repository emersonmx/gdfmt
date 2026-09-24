use insta::assert_snapshot;
use rstest::rstest;

#[rstest]
fn formatted_code_should_match_styleguide_sample() {
    let source = std::fs::read_to_string("samples/styleguide.gd").unwrap();
    let formatted = gdfmt::format_code(&source).unwrap();

    assert_snapshot!(
        "formatted_code_should_match_styleguide_sample",
        formatted
    );
}
