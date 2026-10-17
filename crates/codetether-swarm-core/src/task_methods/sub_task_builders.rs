//! SubTask builders operations.
use super::*;
impl SubTask {
    /// Add a specialty
    ///
    /// # Arguments
    /// * `specialty` - Value used for this operation.
    /// # Returns
    /// The task with its specialty replaced.
    pub fn with_specialty(mut self, specialty: impl Into<String>) -> Self {
        self.specialty = Some(specialty.into());
        self
    }

    /// Add dependencies
    ///
    /// # Arguments
    /// * `deps` - Value used for this operation.
    /// # Returns
    /// The task with its dependencies replaced.
    pub fn with_dependencies(mut self, deps: Vec<String>) -> Self {
        self.dependencies = deps;
        self
    }

    /// Set priority
    ///
    /// # Arguments
    /// * `priority` - Value used for this operation.
    /// # Returns
    /// The task with its priority replaced.
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    /// Set max steps
    ///
    /// # Arguments
    /// * `max_steps` - Value used for this operation.
    /// # Returns
    /// The task with its step limit replaced.
    pub fn with_max_steps(mut self, max_steps: usize) -> Self {
        self.max_steps = max_steps;
        self
    }

    /// Add context
    ///
    /// # Arguments
    /// * `context` - Value used for this operation.
    /// # Returns
    /// The task with its context replaced.
    pub fn with_context(mut self, context: SubTaskContext) -> Self {
        self.context = context;
        self
    }

    /// Record the model-suggested worktree preference. `false` is only
    /// advisory; mutating instructions still require isolation.
    ///
    /// # Arguments
    /// * `needs` - Value used for this operation.
    /// # Returns
    /// The task with its advisory worktree preference recorded.
    pub fn with_needs_worktree(mut self, needs: bool) -> Self {
        self.needs_worktree = Some(needs);
        self
    }
}
