use crate::app::ai::omnibox_intent::{detect_intent, OmniboxIntent};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressAction {
    Navigate(String),
    Search(String),
    Command(String),
}

pub fn resolve_action(input: &str) -> AddressAction {
    match detect_intent(input) {
        OmniboxIntent::Navigate { target } => {
            let normalized = normalize_url(&target);
            AddressAction::Navigate(normalized)
        }
        OmniboxIntent::Search { query } => AddressAction::Search(query),
        OmniboxIntent::Command { command } => AddressAction::Command(command),
    }
}

fn normalize_url(raw: &str) -> String {
    if raw.starts_with("http://") || raw.starts_with("https://") {
        raw.to_string()
    } else {
        format!("https://{raw}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_domain_input_to_https() {
        assert_eq!(
            resolve_action("example.com"),
            AddressAction::Navigate("https://example.com".to_string())
        );
    }
}
