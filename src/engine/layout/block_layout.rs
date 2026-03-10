use crate::engine::dom::node::{Document, NodeType};

use super::box_tree::LayoutBox;

pub fn block_layout(doc: &Document, viewport_width: f32) -> Vec<LayoutBox> {
    let mut result = Vec::new();
    let mut y = 8.0f32;
    for node in &doc.children {
        walk_node(node, 8.0, &mut y, viewport_width - 16.0, &mut result);
    }
    result
}

fn walk_node(
    node: &crate::engine::dom::node::Node,
    x: f32,
    y: &mut f32,
    width: f32,
    boxes: &mut Vec<LayoutBox>,
) {
    match &node.node_type {
        NodeType::Text(text) => {
            let content = text.trim();
            if !content.is_empty() {
                boxes.push(LayoutBox {
                    text: content.to_string(),
                    x,
                    y: *y,
                    width,
                    height: 20.0,
                });
                *y += 22.0;
            }
        }
        NodeType::Element(_) | NodeType::Document => {
            for child in &node.children {
                walk_node(child, x, y, width, boxes);
            }
            if matches!(&node.node_type, NodeType::Element(tag) if tag == "p" || tag == "div") {
                *y += 8.0;
            }
        }
    }
}
