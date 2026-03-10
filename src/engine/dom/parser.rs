use anyhow::anyhow;

use super::{
    node::{Document, Node},
    tokenizer::{tokenize_html, HtmlToken},
};

pub fn parse_html(input: &str) -> anyhow::Result<Document> {
    let tokens = tokenize_html(input);
    let mut root = Node::document(vec![]);
    let mut stack: Vec<Node> = vec![];

    for token in tokens {
        match token {
            HtmlToken::StartTag(tag) => stack.push(Node::element(tag, vec![])),
            HtmlToken::Text(text) => {
                if let Some(top) = stack.last_mut() {
                    top.children.push(Node::text(text));
                } else {
                    root.children.push(Node::text(text));
                }
            }
            HtmlToken::EndTag(tag) => {
                let Some(node) = stack.pop() else {
                    return Err(anyhow!("unexpected end tag: {tag}"));
                };
                if !matches!(&node.node_type, crate::engine::dom::node::NodeType::Element(name) if name == &tag)
                {
                    return Err(anyhow!("mismatched end tag: {tag}"));
                }
                if let Some(parent) = stack.last_mut() {
                    parent.children.push(node);
                } else {
                    root.children.push(node);
                }
            }
        }
    }

    while let Some(node) = stack.pop() {
        if let Some(parent) = stack.last_mut() {
            parent.children.push(node);
        } else {
            root.children.push(node);
        }
    }

    Ok(Document::new(root.children))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_and_extract_title() {
        let doc =
            parse_html("<html><head><title>Demo</title></head><body><p>Hello</p></body></html>")
                .unwrap();
        assert_eq!(doc.find_title().as_deref(), Some("Demo"));
        assert!(doc.visible_text(100).contains("Hello"));
    }
}
