//! Enqueue, consume and settle the latest generation.
use super::Slot;

impl<T> Slot<T> {
    /// Replace pending work and return whether a new worker should start.
    pub fn enqueue(&mut self, build: impl FnOnce(u64) -> T) -> bool {
        self.generation = self.generation.wrapping_add(1);
        self.pending = Some(build(self.generation));
        let spawn = !self.running;
        self.running = true;
        spawn
    }

    /// Take the newest pending snapshot, if any.
    pub fn take(&mut self) -> Option<T> {
        self.pending.take()
    }

    /// Test whether work still belongs to the newest generation.
    pub fn is_current(&self, generation: u64) -> bool {
        self.generation == generation
    }

    /// Mark the slot idle only when no pending snapshot remains.
    pub fn settle(&mut self) -> bool {
        if self.pending.is_some() {
            return false;
        }
        self.running = false;
        true
    }
}
