#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HtmlToken {
    StartTag(String),
    EndTag(String),
    Text(String),
}

pub fn tokenize_html(input: &str) -> Vec<HtmlToken> {
    let mut tokens = Vec::new();
    let chars = input.chars().collect::<Vec<_>>();
    let mut i = 0usize;

    while i < chars.len() {
        if chars[i] == '<' {
            if let Some(end) = chars[i..].iter().position(|c| *c == '>') {
                let end_idx = i + end;
                let raw = chars[i + 1..end_idx].iter().collect::<String>();
                let tag = raw.trim();
                if let Some(name) = tag.strip_prefix('/') {
                    let name = name
                        .split_whitespace()
                        .next()
                        .unwrap_or_default()
                        .to_lowercase();
                    if !name.is_empty() {
                        tokens.push(HtmlToken::EndTag(name));
                    }
                } else {
                    let name = tag
                        .split_whitespace()
                        .next()
                        .unwrap_or_default()
                        .to_lowercase();
                    if !name.is_empty() {
                        tokens.push(HtmlToken::StartTag(name));
                    }
                }
                i = end_idx + 1;
            } else {
                break;
            }
        } else {
            let next = chars[i..]
                .iter()
                .position(|c| *c == '<')
                .map(|v| i + v)
                .unwrap_or(chars.len());
            let text = chars[i..next].iter().collect::<String>();
            if !text.trim().is_empty() {
                tokens.push(HtmlToken::Text(text));
            }
            i = next;
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_basic_html() {
        let tokens = tokenize_html("<h1>Hello</h1>");
        assert_eq!(
            tokens,
            vec![
                HtmlToken::StartTag("h1".to_string()),
                HtmlToken::Text("Hello".to_string()),
                HtmlToken::EndTag("h1".to_string())
            ]
        );
    }
}
