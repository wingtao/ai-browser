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
        fn walk(node: &Node, in_title: bool) -> Option<String> {
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
                NodeType::Text(text) if in_title => Some(text.trim().to_string()),
                _ => {
                    for child in &node.children {
                        if let Some(v) = walk(child, false) {
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
            if let Some(t) = walk(node, false) {
                if !t.is_empty() {
                    return Some(t);
                }
            }
        }
        None
    }

    pub fn visible_text(&self, max_chars: usize) -> String {
        fn walk(node: &Node, out: &mut String) {
            match &node.node_type {
                NodeType::Text(text) => {
                    let t = text.trim();
                    if !t.is_empty() {
                        if !out.is_empty() {
                            out.push('\n');
                        }
                        out.push_str(t);
                    }
                }
                _ => {
                    for child in &node.children {
                        walk(child, out);
                    }
                }
            }
        }

        let mut out = String::new();
        for node in &self.children {
            walk(node, &mut out);
            if out.len() >= max_chars {
                break;
            }
        }
        out.chars().take(max_chars).collect()
    }
}
