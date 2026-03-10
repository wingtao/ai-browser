use std::collections::HashMap;

pub type EventCallback = Box<dyn FnMut() + Send>;

#[derive(Default)]
pub struct EventTarget {
    listeners: HashMap<String, Vec<EventCallback>>,
}

impl EventTarget {
    pub fn add_event_listener(&mut self, event: impl Into<String>, callback: EventCallback) {
        self.listeners
            .entry(event.into())
            .or_default()
            .push(callback);
    }

    pub fn dispatch(&mut self, event: &str) {
        if let Some(listeners) = self.listeners.get_mut(event) {
            for listener in listeners {
                listener();
            }
        }
    }
}
