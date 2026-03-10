use std::collections::VecDeque;

pub type Task = Box<dyn FnOnce() + Send>;

#[derive(Default)]
pub struct EventLoop {
    microtasks: VecDeque<Task>,
    macrotasks: VecDeque<Task>,
}

impl EventLoop {
    pub fn queue_microtask(&mut self, task: Task) {
        self.microtasks.push_back(task);
    }

    pub fn queue_macrotask(&mut self, task: Task) {
        self.macrotasks.push_back(task);
    }

    pub fn run_once(&mut self) {
        while let Some(task) = self.microtasks.pop_front() {
            task();
        }
        if let Some(task) = self.macrotasks.pop_front() {
            task();
        }
    }

    pub fn has_pending_tasks(&self) -> bool {
        !self.microtasks.is_empty() || !self.macrotasks.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::EventLoop;

    #[test]
    fn microtask_runs_before_macrotask() {
        let log = Arc::new(Mutex::new(vec![]));
        let mut loop_ = EventLoop::default();

        {
            let log = Arc::clone(&log);
            loop_.queue_macrotask(Box::new(move || log.lock().unwrap().push("macro")));
        }
        {
            let log = Arc::clone(&log);
            loop_.queue_microtask(Box::new(move || log.lock().unwrap().push("micro")));
        }

        loop_.run_once();
        assert_eq!(log.lock().unwrap().as_slice(), ["micro", "macro"]);
    }
}
