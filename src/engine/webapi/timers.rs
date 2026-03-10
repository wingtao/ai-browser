use super::event_loop::{EventLoop, Task};

#[derive(Default)]
pub struct TimerApi;

impl TimerApi {
    pub fn set_timeout(event_loop: &mut EventLoop, task: Task) {
        event_loop.queue_macrotask(task);
    }
}
