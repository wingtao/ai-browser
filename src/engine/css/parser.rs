use super::tokenizer::tokenize_stylesheet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StyleRule {
    pub selector: String,
    pub declarations: Vec<(String, String)>,
}

pub fn parse_stylesheet(input: &str) -> Vec<StyleRule> {
    let chunks = tokenize_stylesheet(input);
    parse_stylesheet_chunks(&chunks)
}

pub fn parse_stylesheet_chunks(chunks: &[super::tokenizer::RuleChunk]) -> Vec<StyleRule> {
    chunks
        .iter()
        .map(|chunk| {
            let declarations = chunk
                .body
                .split(';')
                .filter_map(|line| line.split_once(':'))
                .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
                .collect::<Vec<_>>();
            StyleRule {
                selector: chunk.selector.clone(),
                declarations,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::parse_stylesheet;

    #[test]
    fn parse_simple_css() {
        let rules = parse_stylesheet("h1 { color: red; font-size: 20px; }");
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].selector, "h1");
        assert_eq!(rules[0].declarations.len(), 2);
    }
}
