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
        "function_definition" => {
            let header = node
                .child_by_field_name("name")
                .map(|n| &source[n.byte_range()])
                .unwrap_or("func_name");

            let parameters_node = node.child_by_field_name("parameters");
            let parameters_text = parameters_node
                .map(|n| &source[n.byte_range()])
                .unwrap_or("()");

            let body = node.child_by_field_name("body");

            let mut result = format!(
                "{}func {}{}:\n",
                indent,
                header.trim(),
                parameters_text.trim()
            );

            if let Some(body_node) = body {
                for child in body_node.children(&mut body_node.walk()) {
                    result += &format_node(child, source, indent_level + 1)?;
                }
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
    fn test_format_node_converts_spaces_to_tabs() {
        let source = r#"func my_function():
        print("Hello, world!")
        pass"#;
        let (tree, source_str) = parse_gscript(source);
        let root_node = tree.root_node();

        let func_node = root_node
            .children(&mut root_node.walk())
            .find(|n| n.kind() == "function_definition")
            .expect("Could not find function_definition node");

        let formatted = format_node(func_node, &source_str, 0).unwrap();

        let expected = "func my_function():\n\tprint(\"Hello, world!\")\n\tpass\n";
        assert_eq!(formatted, expected);
    }
}
