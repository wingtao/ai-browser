use super::display_list::DrawCommand;

#[derive(Debug, Clone, PartialEq)]
pub struct RasterizedFrame {
    pub width: u32,
    pub height: u32,
    pub text_runs: Vec<String>,
}

pub fn rasterize(commands: &[DrawCommand], width: u32, height: u32) -> RasterizedFrame {
    RasterizedFrame {
        width,
        height,
        text_runs: commands.iter().map(|cmd| cmd.text.clone()).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::rasterize;
    use crate::engine::paint::display_list::DrawCommand;

    #[test]
    fn rasterize_collects_text_runs() {
        let frame = rasterize(
            &[
                DrawCommand {
                    text: "A".to_string(),
                    x: 0.0,
                    y: 0.0,
                },
                DrawCommand {
                    text: "B".to_string(),
                    x: 1.0,
                    y: 2.0,
                },
            ],
            800,
            600,
        );
        assert_eq!(frame.text_runs, vec!["A".to_string(), "B".to_string()]);
    }
}
