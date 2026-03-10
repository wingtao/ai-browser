pub fn redact_sensitive(input: &str) -> String {
    let patterns = ["password=", "token=", "apikey=", "authorization:"];
    input
        .split_whitespace()
        .map(|word| {
            let lower = word.to_lowercase();
            if patterns.iter().any(|p| lower.contains(p)) {
                "[REDACTED]".to_string()
            } else {
                word.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redact_tokens() {
        let raw = "url=example token=abc123 data";
        assert_eq!(redact_sensitive(raw), "url=example [REDACTED] data");
    }
}
