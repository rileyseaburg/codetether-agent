//! Latest-value coalescing state independent of provider/runtime details.

pub(super) struct Slot<T> {
    pending: Option<T>,
    generation: u64,
    running: bool,
}

impl<T> Default for Slot<T> {
    fn default() -> Self {
        Self {
            pending: None,
            generation: 0,
            running: false,
        }
    }
}

impl<T> Slot<T> {
    pub fn enqueue(&mut self, build: impl FnOnce(u64) -> T) -> bool {
        self.generation = self.generation.wrapping_add(1);
        self.pending = Some(build(self.generation));
        let spawn = !self.running;
        self.running = true;
        spawn
    }

    pub fn take(&mut self) -> Option<T> {
        self.pending.take()
    }

    pub fn is_current(&self, generation: u64) -> bool {
        self.generation == generation
    }

    pub fn settle(&mut self) -> bool {
        if self.pending.is_some() {
            return false;
        }
        self.running = false;
        true
    }
}
