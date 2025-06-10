use thiserror::Error;
use tree_sitter::Node;

#[derive(Error, Debug)]
pub enum Error {
    #[error("mixed indentation")]
    MixedIndentation,
}

pub fn format_node(node: Node, source: &str, indent_level: usize) -> Result<String, Error> {
    let indent = "\t".repeat(indent_level);
    let kind = node.kind();

    match kind {
        "source" => {
            let mut result = String::new();
            for child in node.children(&mut node.walk()) {
                result += &format_node(child, source, indent_level)?;
            }
            Ok(result)
        }
        _ => {
            let text = &source[node.byte_range()];
            let formatted_text = format!("{}{}\n", indent, text.trim());
            Ok(formatted_text)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter::{Parser, Tree};
    use tree_sitter_gdscript::LANGUAGE as gdscript_language;

    fn parse_gscript(source: &str) -> (Tree, String) {
        let mut parser = Parser::new();
        parser
            .set_language(&gdscript_language.into())
            .expect("Error loading GDScript grammar");
        let tree = parser.parse(source, None).expect("Error parsing code");
        (tree, source.to_string())
    }

    #[test]
    fn test_trim_trailing_spaces() {
        let source = "var i = 0    ";
        let (tree, source_str) = parse_gscript(source);
        let root_node = tree.root_node();

        let formatted = format_node(root_node, &source_str, 0).unwrap();

        let expected = "var i = 0\n";
        assert_eq!(formatted, expected);
    }

    #[test]
    fn test_keep_one_newline_at_end() {
        let source = "var i = 0\n\n";
        let (tree, source_str) = parse_gscript(source);
        let root_node = tree.root_node();

        let formatted = format_node(root_node, &source_str, 0).unwrap();
        println!("{}", formatted);

        let expected = "var i = 0\n";
        assert_eq!(formatted, expected);
    }
}
