use super::*;
impl SwarmCache {
    /// Generate a task key using the standard library hasher.
    ///
    /// # Arguments
    /// * `task` - Task identity, instruction, and execution context.
    ///
    /// # Returns
    /// A hexadecimal key for the task and its dependency results.
    ///
    /// # Examples
    /// ```
    /// let task = codetether_swarm_core::SubTask::new("example", "inspect source");
    /// assert_eq!(codetether_swarm_core::cache::SwarmCache::generate_key(&task).len(), 16);
    /// ```
    pub fn generate_key(task: &SubTask) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        // Create a deterministic hash from task content
        let mut hasher = DefaultHasher::new();
        task.name.hash(&mut hasher);
        task.instruction.hash(&mut hasher);
        task.specialty.hash(&mut hasher);
        task.max_steps.hash(&mut hasher);

        // Include context that affects execution
        if let Some(parent) = &task.context.parent_task {
            parent.hash(&mut hasher);
        }

        // Hash dependency results that are part of the context
        let mut dep_keys: Vec<_> = task.context.dependency_results.keys().collect();
        dep_keys.sort(); // Ensure deterministic ordering
        for key in dep_keys {
            key.hash(&mut hasher);
            task.context.dependency_results[key].hash(&mut hasher);
        }

        format!("{:016x}", hasher.finish())
    }

    /// Generate a content hash for verification
    pub(super) fn generate_content_hash(task: &SubTask) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        task.instruction.hash(&mut hasher);
        format!("{:016x}", hasher.finish())
    }
}
