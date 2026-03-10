use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct TabInfo {
    pub id: u64,
    pub title: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabGroupSuggestion {
    pub group_name: String,
    pub tab_ids: Vec<u64>,
}

pub fn suggest_groups(tabs: &[TabInfo]) -> Vec<TabGroupSuggestion> {
    let mut map: BTreeMap<String, Vec<u64>> = BTreeMap::new();
    for tab in tabs {
        let domain = tab
            .url
            .split("://")
            .nth(1)
            .unwrap_or(&tab.url)
            .split('/')
            .next()
            .unwrap_or("other")
            .to_string();
        map.entry(domain).or_default().push(tab.id);
    }
    map.into_iter()
        .filter(|(_, ids)| ids.len() >= 2)
        .map(|(name, ids)| TabGroupSuggestion {
            group_name: name,
            tab_ids: ids,
        })
        .collect()
}
