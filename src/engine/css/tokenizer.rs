#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleChunk {
    pub selector: String,
    pub body: String,
}

pub fn tokenize_stylesheet(input: &str) -> Vec<RuleChunk> {
    let mut chunks = Vec::new();
    for chunk in input.split('}') {
        if let Some((selector, body)) = chunk.split_once('{') {
            let selector = selector.trim();
            if selector.is_empty() {
                continue;
            }
            chunks.push(RuleChunk {
                selector: selector.to_string(),
                body: body.trim().to_string(),
            });
        }
    }
    chunks
}

#[cfg(test)]
mod tests {
    use super::tokenize_stylesheet;

    #[test]
    fn tokenize_css_rule_chunks() {
        let chunks = tokenize_stylesheet("h1 { color: red; } p { margin: 0; }");
        assert_eq!(chunks.len(), 2);
        assert_eq!(chunks[0].selector, "h1");
        assert_eq!(chunks[1].selector, "p");
    }
}
