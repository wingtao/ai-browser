use std::{cell::RefCell, rc::Rc};

use crate::engine::dom::node::{Document, Node, NodeType};

#[derive(Clone, Debug)]
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

    pub fn ancestor_chain_for_tag(&self, selector: &str) -> Vec<String> {
        let doc = self.doc.borrow();
        let mut path = Vec::new();
        for node in &doc.children {
            let mut current = Vec::new();
            if find_path_to_tag(node, selector, &mut current) {
                path = current;
                break;
            }
        }
        if path.is_empty() {
            return vec![selector.to_string(), "document".to_string()];
        }
        path.reverse();
        path.push("document".to_string());
        path
    }

    pub fn append_child_text_to_first_tag(
        &self,
        parent_selector: &str,
        child_tag: &str,
        text: &str,
    ) -> bool {
        let mut doc = self.doc.borrow_mut();
        for node in &mut doc.children {
            if append_child_to_first_tag(node, parent_selector, child_tag, text) {
                return true;
            }
        }
        false
    }

    pub fn remove_first_child_tag_from_first_tag(
        &self,
        parent_selector: &str,
        child_selector: &str,
    ) -> bool {
        let mut doc = self.doc.borrow_mut();
        for node in &mut doc.children {
            if remove_first_child_from_first_tag(node, parent_selector, child_selector) {
                return true;
            }
        }
        false
    }

    pub fn inner_html_for_first_tag(&self, selector: &str) -> Option<String> {
        let doc = self.doc.borrow();
        for node in &doc.children {
            if let Some(html) = inner_html_for_first_tag(node, selector) {
                return Some(html);
            }
        }
        None
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

fn find_path_to_tag(node: &Node, selector: &str, out: &mut Vec<String>) -> bool {
    if let NodeType::Element(tag) = &node.node_type {
        out.push(tag.to_lowercase());
        if tag.eq_ignore_ascii_case(selector) {
            return true;
        }
        for child in &node.children {
            if find_path_to_tag(child, selector, out) {
                return true;
            }
        }
        out.pop();
        false
    } else {
        for child in &node.children {
            if find_path_to_tag(child, selector, out) {
                return true;
            }
        }
        false
    }
}

fn append_child_to_first_tag(
    node: &mut Node,
    parent_selector: &str,
    child_tag: &str,
    text: &str,
) -> bool {
    if let NodeType::Element(tag) = &node.node_type {
        if tag.eq_ignore_ascii_case(parent_selector) {
            node.children
                .push(Node::element(child_tag, vec![Node::text(text.to_string())]));
            return true;
        }
    }
    for child in &mut node.children {
        if append_child_to_first_tag(child, parent_selector, child_tag, text) {
            return true;
        }
    }
    false
}

fn remove_first_child_from_first_tag(
    node: &mut Node,
    parent_selector: &str,
    child_selector: &str,
) -> bool {
    if let NodeType::Element(tag) = &node.node_type {
        if tag.eq_ignore_ascii_case(parent_selector) {
            if let Some(idx) = node.children.iter().position(|child| {
                matches!(&child.node_type, NodeType::Element(t) if t.eq_ignore_ascii_case(child_selector))
            }) {
                node.children.remove(idx);
                return true;
            }
            return false;
        }
    }
    for child in &mut node.children {
        if remove_first_child_from_first_tag(child, parent_selector, child_selector) {
            return true;
        }
    }
    false
}

fn inner_html_for_first_tag(node: &Node, selector: &str) -> Option<String> {
    if let NodeType::Element(tag) = &node.node_type {
        if tag.eq_ignore_ascii_case(selector) {
            let html = node
                .children
                .iter()
                .map(node_to_html)
                .collect::<Vec<_>>()
                .join("");
            return Some(html);
        }
    }
    for child in &node.children {
        if let Some(v) = inner_html_for_first_tag(child, selector) {
            return Some(v);
        }
    }
    None
}

fn node_to_html(node: &Node) -> String {
    match &node.node_type {
        NodeType::Text(text) => text.clone(),
        NodeType::Element(tag) => {
            let children = node
                .children
                .iter()
                .map(node_to_html)
                .collect::<Vec<_>>()
                .join("");
            format!("<{tag}>{children}</{tag}>")
        }
        NodeType::Document => node
            .children
            .iter()
            .map(node_to_html)
            .collect::<Vec<_>>()
            .join(""),
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

    #[test]
    fn build_ancestor_chain() {
        let doc = parse_html("<html><body><div><button>Go</button></div></body></html>").unwrap();
        let binding = JsDocumentBinding::new(Rc::new(RefCell::new(doc)));
        let chain = binding.ancestor_chain_for_tag("button");
        assert_eq!(
            chain,
            vec![
                "button".to_string(),
                "div".to_string(),
                "body".to_string(),
                "html".to_string(),
                "document".to_string()
            ]
        );
    }

    #[test]
    fn append_remove_and_inner_html() {
        let doc = parse_html("<html><body><div><p>A</p></div></body></html>").unwrap();
        let binding = JsDocumentBinding::new(Rc::new(RefCell::new(doc)));
        assert!(binding.append_child_text_to_first_tag("div", "span", "B"));
        let html = binding.inner_html_for_first_tag("div").unwrap();
        assert!(html.contains("<p>A</p>"));
        assert!(html.contains("<span>B</span>"));
        assert!(binding.remove_first_child_tag_from_first_tag("div", "p"));
        let html = binding.inner_html_for_first_tag("div").unwrap();
        assert!(!html.contains("<p>A</p>"));
    }
}
