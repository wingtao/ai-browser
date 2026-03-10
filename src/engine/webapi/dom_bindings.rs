use std::{cell::RefCell, rc::Rc};

use crate::engine::dom::node::{Document, Node, NodeType};

#[derive(Clone)]
pub struct JsDocumentBinding {
    doc: Rc<RefCell<Document>>,
}

impl JsDocumentBinding {
    pub fn new(doc: Rc<RefCell<Document>>) -> Self {
        Self { doc }
    }

    pub fn query_selector_text(&self, selector: &str) -> Option<String> {
        let doc = self.doc.borrow();
        for node in &doc.children {
            if let Some(text) = find_text_by_tag(node, selector) {
                return Some(text);
            }
        }
        None
    }

    pub fn set_first_text_for_tag(&self, selector: &str, text: &str) -> bool {
        let mut doc = self.doc.borrow_mut();
        for node in &mut doc.children {
            if set_text_by_tag(node, selector, text) {
                return true;
            }
        }
        false
    }

    pub fn query_selector_all_text(&self, selector: &str) -> Vec<String> {
        let doc = self.doc.borrow();
        let mut result = Vec::new();
        for node in &doc.children {
            find_all_text_by_tag(node, selector, &mut result);
        }
        result
    }
}

fn find_text_by_tag(node: &Node, selector: &str) -> Option<String> {
    if let NodeType::Element(tag) = &node.node_type {
        if tag.eq_ignore_ascii_case(selector) {
            for child in &node.children {
                if let NodeType::Text(t) = &child.node_type {
                    return Some(t.trim().to_string());
                }
            }
        }
    }
    for child in &node.children {
        if let Some(v) = find_text_by_tag(child, selector) {
            return Some(v);
        }
    }
    None
}

fn set_text_by_tag(node: &mut Node, selector: &str, text: &str) -> bool {
    if let NodeType::Element(tag) = &node.node_type {
        if tag.eq_ignore_ascii_case(selector) {
            for child in &mut node.children {
                if matches!(child.node_type, NodeType::Text(_)) {
                    child.node_type = NodeType::Text(text.to_string());
                    return true;
                }
            }
            node.children.push(Node::text(text));
            return true;
        }
    }
    for child in &mut node.children {
        if set_text_by_tag(child, selector, text) {
            return true;
        }
    }
    false
}

fn find_all_text_by_tag(node: &Node, selector: &str, out: &mut Vec<String>) {
    if let NodeType::Element(tag) = &node.node_type {
        if tag.eq_ignore_ascii_case(selector) {
            for child in &node.children {
                if let NodeType::Text(t) = &child.node_type {
                    let text = t.trim();
                    if !text.is_empty() {
                        out.push(text.to_string());
                    }
                }
            }
        }
    }
    for child in &node.children {
        find_all_text_by_tag(child, selector, out);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::dom::parser::parse_html;

    #[test]
    fn query_and_set_text() {
        let doc = parse_html("<html><body><h1>Hello</h1></body></html>").unwrap();
        let binding = JsDocumentBinding::new(Rc::new(RefCell::new(doc)));
        assert_eq!(binding.query_selector_text("h1").as_deref(), Some("Hello"));
        assert!(binding.set_first_text_for_tag("h1", "Updated"));
        assert_eq!(
            binding.query_selector_text("h1").as_deref(),
            Some("Updated")
        );
    }

    #[test]
    fn query_selector_all_text() {
        let doc =
            parse_html("<html><body><p>A</p><div><p>B</p></div><p>C</p></body></html>").unwrap();
        let binding = JsDocumentBinding::new(Rc::new(RefCell::new(doc)));
        let all = binding.query_selector_all_text("p");
        assert_eq!(all, vec!["A".to_string(), "B".to_string(), "C".to_string()]);
    }
}
