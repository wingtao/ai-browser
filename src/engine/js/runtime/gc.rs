#[derive(Debug, Default)]
pub struct GcRuntime {
    allocated_objects: usize,
}

impl GcRuntime {
    pub fn track_alloc(&mut self) {
        self.allocated_objects += 1;
    }

    pub fn allocated_objects(&self) -> usize {
        self.allocated_objects
    }

    pub fn collect(&mut self) {
        // v1: 占位实现。后续替换为 mark-sweep。
    }
}
