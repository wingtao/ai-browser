#[derive(Debug, Default, Clone)]
pub struct NavigationState {
    back_stack: Vec<String>,
    current: Option<String>,
    forward_stack: Vec<String>,
}

impl NavigationState {
    pub fn navigate(&mut self, url: impl Into<String>) {
        if let Some(current) = self.current.take() {
            self.back_stack.push(current);
        }
        self.current = Some(url.into());
        self.forward_stack.clear();
    }

    pub fn back(&mut self) -> Option<String> {
        let prev = self.back_stack.pop()?;
        if let Some(current) = self.current.take() {
            self.forward_stack.push(current);
        }
        self.current = Some(prev.clone());
        Some(prev)
    }

    pub fn forward(&mut self) -> Option<String> {
        let next = self.forward_stack.pop()?;
        if let Some(current) = self.current.take() {
            self.back_stack.push(current);
        }
        self.current = Some(next.clone());
        Some(next)
    }

    pub fn current(&self) -> Option<&str> {
        self.current.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn navigate_back_forward() {
        let mut nav = NavigationState::default();
        nav.navigate("a");
        nav.navigate("b");
        nav.navigate("c");
        assert_eq!(nav.back(), Some("b".to_string()));
        assert_eq!(nav.back(), Some("a".to_string()));
        assert_eq!(nav.forward(), Some("b".to_string()));
    }
}
