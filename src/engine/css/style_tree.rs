use std::collections::HashMap;

use crate::engine::dom::node::{Node, NodeType};

use super::parser::StyleRule;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StyledNode {
    pub tag_name: Option<String>,
    pub styles: HashMap<String, String>,
    pub children: Vec<StyledNode>,
}

pub fn build_style_tree(node: &Node, rules: &[StyleRule]) -> StyledNode {
    let mut styles = HashMap::new();
    if let NodeType::Element(tag) = &node.node_type {
        for rule in rules {
            if selector_matches(&rule.selector, tag) {
                for (k, v) in &rule.declarations {
                    styles.insert(k.clone(), v.clone());
                }
            }
        }
    }
    StyledNode {
        tag_name: match &node.node_type {
            NodeType::Element(tag) => Some(tag.clone()),
            _ => None,
        },
        styles,
        children: node
            .children
            .iter()
            .map(|child| build_style_tree(child, rules))
            .collect(),
    }
}

fn selector_matches(selector: &str, tag: &str) -> bool {
    selector
        .split(',')
        .map(str::trim)
        .any(|s| s.eq_ignore_ascii_case(tag))
}

#[cfg(test)]
mod tests {
    use crate::engine::{css::parser::parse_stylesheet, dom::parser::parse_html};

    use super::build_style_tree;

    #[test]
    fn apply_tag_selector_style() {
        let doc = parse_html("<html><body><h1>Hello</h1></body></html>").unwrap();
        let rules = parse_stylesheet("h1 { color: red; }");
        let root = build_style_tree(&doc.children[0], &rules);
        let body = &root.children[0];
        let h1 = &body.children[0];
        assert_eq!(h1.styles.get("color").map(String::as_str), Some("red"));
    }
}
