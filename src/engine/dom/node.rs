#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Document,
    Element(String),
    Text(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub node_type: NodeType,
    pub children: Vec<Node>,
}

impl Node {
    pub fn document(children: Vec<Node>) -> Self {
        Self {
            node_type: NodeType::Document,
            children,
        }
    }

    pub fn element(tag: impl Into<String>, children: Vec<Node>) -> Self {
        Self {
            node_type: NodeType::Element(tag.into()),
            children,
        }
    }

    pub fn text(content: impl Into<String>) -> Self {
        Self {
            node_type: NodeType::Text(content.into()),
            children: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    pub children: Vec<Node>,
}

impl Document {
    pub fn new(children: Vec<Node>) -> Self {
        Self { children }
    }

    pub fn find_title(&self) -> Option<String> {
        fn walk(node: &Node) -> Option<String> {
            match &node.node_type {
                NodeType::Element(tag) if tag.eq_ignore_ascii_case("title") => {
                    for child in &node.children {
                        if let NodeType::Text(text) = &child.node_type {
                            let t = text.trim();
                            if !t.is_empty() {
                                return Some(t.to_string());
                            }
                        }
                    }
                    None
                }
                _ => {
                    for child in &node.children {
                        if let Some(v) = walk(child) {
                            if !v.is_empty() {
                                return Some(v);
                            }
                        }
                    }
                    None
                }
            }
        }

        for node in &self.children {
            if let Some(t) = walk(node) {
                if !t.is_empty() {
                    return Some(t);
                }
            }
        }
        None
    }

    pub fn visible_text(&self, max_chars: usize) -> String {
        fn walk(node: &Node, out: &mut String, hidden: bool) {
            match &node.node_type {
                NodeType::Text(text) => {
                    if hidden {
                        return;
                    }
                    let t = text.trim();
                    if !t.is_empty() {
                        if !out.is_empty() {
                            out.push('\n');
                        }
                        out.push_str(t);
                    }
                }
                NodeType::Element(tag) => {
                    let hidden = hidden
                        || tag.eq_ignore_ascii_case("script")
                        || tag.eq_ignore_ascii_case("style");
                    for child in &node.children {
                        walk(child, out, hidden);
                    }
                }
                NodeType::Document => {
                    for child in &node.children {
                        walk(child, out, hidden);
                    }
                }
            }
        }

        let mut out = String::new();
        for node in &self.children {
            walk(node, &mut out, false);
            if out.len() >= max_chars {
                break;
            }
        }
        out.chars().take(max_chars).collect()
    }

    pub fn collect_text_by_tag(&self, tag: &str) -> Vec<String> {
        fn walk(node: &Node, tag: &str, out: &mut Vec<String>) {
            if let NodeType::Element(name) = &node.node_type {
                if name.eq_ignore_ascii_case(tag) {
                    for child in &node.children {
                        if let NodeType::Text(text) = &child.node_type {
                            let text = text.trim();
                            if !text.is_empty() {
                                out.push(text.to_string());
                            }
                        }
                    }
                }
            }
            for child in &node.children {
                walk(child, tag, out);
            }
        }

        let mut out = Vec::new();
        for node in &self.children {
            walk(node, tag, &mut out);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::{Document, Node};

    #[test]
    fn visible_text_should_skip_script_style() {
        let doc = Document::new(vec![Node::element(
            "html",
            vec![Node::element(
                "body",
                vec![
                    Node::element("h1", vec![Node::text("Hello")]),
                    Node::element("script", vec![Node::text("dom_set_text(\"h1\",\"x\")")]),
                    Node::element("style", vec![Node::text("h1 { color: red; }")]),
                ],
            )],
        )]);
        let text = doc.visible_text(200);
        assert!(text.contains("Hello"));
        assert!(!text.contains("dom_set_text"));
        assert!(!text.contains("color: red"));
    }
}
