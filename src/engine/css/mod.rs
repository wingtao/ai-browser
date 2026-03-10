#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StyleRule {
    pub selector: String,
    pub declarations: Vec<(String, String)>,
}

pub fn parse_stylesheet(input: &str) -> Vec<StyleRule> {
    let mut rules = Vec::new();
    for chunk in input.split('}') {
        if let Some((selector, body)) = chunk.split_once('{') {
            let selector = selector.trim();
            if selector.is_empty() {
                continue;
            }
            let declarations = body
                .split(';')
                .filter_map(|line| line.split_once(':'))
                .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
                .collect::<Vec<_>>();
            rules.push(StyleRule {
                selector: selector.to_string(),
                declarations,
            });
        }
    }
    rules
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_css() {
        let rules = parse_stylesheet("h1 { color: red; font-size: 20px; }");
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].selector, "h1");
        assert_eq!(rules[0].declarations.len(), 2);
    }
}
