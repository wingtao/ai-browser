#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OmniboxIntent {
    Navigate { target: String },
    Search { query: String },
    Command { command: String },
}

pub fn detect_intent(input: &str) -> OmniboxIntent {
    let raw = input.trim();
    let lower = raw.to_lowercase();
    let command_prefixes = ["关闭", "close ", "新建标签", "刷新", "后退", "前进"];
    if command_prefixes.iter().any(|p| lower.starts_with(p)) {
        return OmniboxIntent::Command {
            command: raw.to_string(),
        };
    }

    if lower.starts_with("搜索 ") || lower.starts_with("search ") {
        let q = raw
            .split_once(' ')
            .map(|(_, rhs)| rhs.trim().to_string())
            .unwrap_or_default();
        return OmniboxIntent::Search { query: q };
    }

    if looks_like_url(raw) || lower.starts_with("打开 ") || lower.starts_with("open ") {
        let target = raw
            .trim_start_matches("打开 ")
            .trim_start_matches("open ")
            .trim()
            .to_string();
        return OmniboxIntent::Navigate { target };
    }

    OmniboxIntent::Search {
        query: raw.to_string(),
    }
}

fn looks_like_url(input: &str) -> bool {
    input.starts_with("http://")
        || input.starts_with("https://")
        || (input.contains('.') && !input.contains(' '))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_command() {
        let intent = detect_intent("关闭其他标签页");
        assert_eq!(
            intent,
            OmniboxIntent::Command {
                command: "关闭其他标签页".to_string()
            }
        );
    }

    #[test]
    fn classify_search() {
        let intent = detect_intent("搜索 rust parser");
        assert_eq!(
            intent,
            OmniboxIntent::Search {
                query: "rust parser".to_string()
            }
        );
    }

    #[test]
    fn classify_url() {
        let intent = detect_intent("https://example.com");
        assert_eq!(
            intent,
            OmniboxIntent::Navigate {
                target: "https://example.com".to_string()
            }
        );
    }
}
