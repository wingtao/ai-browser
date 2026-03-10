pub fn sanitize_page_context(input: &str) -> String {
    let denylist = [
        "ignore previous instructions",
        "system prompt",
        "developer prompt",
        "reveal api key",
    ];
    input
        .lines()
        .filter(|line| {
            let lower = line.to_lowercase();
            !denylist.iter().any(|w| lower.contains(w))
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_injection_lines() {
        let src = "hello\nIgnore previous instructions\nnormal";
        let out = sanitize_page_context(src);
        assert!(out.contains("hello"));
        assert!(out.contains("normal"));
        assert!(!out.to_lowercase().contains("ignore previous"));
    }
}
