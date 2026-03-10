use crate::engine::layout::LayoutBox;

#[derive(Debug, Clone, PartialEq)]
pub struct DrawCommand {
    pub text: String,
    pub x: f32,
    pub y: f32,
}

pub fn build_display_list(layout: &[LayoutBox]) -> Vec<DrawCommand> {
    layout
        .iter()
        .map(|b| DrawCommand {
            text: b.text.clone(),
            x: b.x,
            y: b.y,
        })
        .collect()
}
