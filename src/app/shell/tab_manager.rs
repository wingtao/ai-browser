use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tab {
    pub id: u64,
    pub title: String,
    pub url: String,
}

#[derive(Debug, Default)]
pub struct TabManager {
    tabs: BTreeMap<u64, Tab>,
    active: Option<u64>,
    next_id: u64,
}

impl TabManager {
    pub fn create_tab(&mut self, url: impl Into<String>) -> u64 {
        self.next_id += 1;
        let id = self.next_id;
        let tab = Tab {
            id,
            title: "New Tab".to_string(),
            url: url.into(),
        };
        self.tabs.insert(id, tab);
        self.active = Some(id);
        id
    }

    pub fn close_tab(&mut self, id: u64) -> bool {
        let removed = self.tabs.remove(&id).is_some();
        if removed && self.active == Some(id) {
            self.active = self.tabs.keys().next_back().copied();
        }
        removed
    }

    pub fn switch_to(&mut self, id: u64) -> bool {
        if self.tabs.contains_key(&id) {
            self.active = Some(id);
            true
        } else {
            false
        }
    }

    pub fn update_title(&mut self, id: u64, title: impl Into<String>) {
        if let Some(tab) = self.tabs.get_mut(&id) {
            tab.title = title.into();
        }
    }

    pub fn update_url(&mut self, id: u64, url: impl Into<String>) {
        if let Some(tab) = self.tabs.get_mut(&id) {
            tab.url = url.into();
        }
    }

    pub fn active_tab(&self) -> Option<&Tab> {
        self.active.and_then(|id| self.tabs.get(&id))
    }

    pub fn all_tabs(&self) -> Vec<&Tab> {
        self.tabs.values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_switch_close_tabs() {
        let mut manager = TabManager::default();
        let t1 = manager.create_tab("https://a.com");
        let t2 = manager.create_tab("https://b.com");
        assert_eq!(manager.active_tab().unwrap().id, t2);
        assert!(manager.switch_to(t1));
        assert_eq!(manager.active_tab().unwrap().id, t1);
        assert!(manager.close_tab(t1));
        assert_eq!(manager.active_tab().unwrap().id, t2);
    }
}
