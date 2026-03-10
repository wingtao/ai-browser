use crate::engine::dom::node::Document;

#[derive(Debug, Clone)]
pub struct PageContext {
    pub url: String,
    pub title: String,
    pub visible_text: String,
}

impl PageContext {
    pub fn from_document(url: impl Into<String>, doc: &Document) -> Self {
        Self {
            url: url.into(),
            title: doc.find_title().unwrap_or_else(|| "Untitled".to_string()),
            visible_text: doc.visible_text(8_000),
        }
    }
}
