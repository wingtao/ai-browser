use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use super::event_loop::{EventLoop, Task};

#[derive(Default)]
pub struct TimerApi {
    next_id: u64,
    cancelled: Arc<Mutex<HashSet<u64>>>,
}

impl TimerApi {
    pub fn set_timeout(&mut self, event_loop: &mut EventLoop, task: Task) -> u64 {
        self.next_id += 1;
        let id = self.next_id;
        let cancelled = Arc::clone(&self.cancelled);
        let mut task_opt = Some(task);
        event_loop.queue_macrotask(Box::new(move || {
            if cancelled.lock().unwrap().contains(&id) {
                return;
            }
            if let Some(task) = task_opt.take() {
                task();
            }
        }));
        id
    }

    pub fn clear_timeout(&mut self, id: u64) {
        self.cancelled.lock().unwrap().insert(id);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;

    #[test]
    fn cancel_timeout_should_prevent_execution() {
        let mut event_loop = EventLoop::default();
        let mut timer = TimerApi::default();
        let log = Arc::new(Mutex::new(Vec::new()));

        let id = {
            let log = Arc::clone(&log);
            timer.set_timeout(
                &mut event_loop,
                Box::new(move || log.lock().unwrap().push("run")),
            )
        };
        timer.clear_timeout(id);
        event_loop.run_once();
        assert!(log.lock().unwrap().is_empty());
    }
}
