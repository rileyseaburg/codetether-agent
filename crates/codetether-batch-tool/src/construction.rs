//! Batch construction and weak registry binding.
use super::{BatchTool, Registry};
use std::sync::{Arc, RwLock, Weak};

impl<R: Registry> BatchTool<R> {
    /// Create an unbound batch tool.
    ///
    /// # Arguments
    /// None.
    /// # Returns
    /// A tool that reports an uninitialized registry until it is bound.
    /// # Examples
    /// ```rust
    /// # fn example<R: codetether_batch_tool::Registry>() {
    /// let _tool = codetether_batch_tool::BatchTool::<R>::new();
    /// # }
    /// ```
    pub fn new() -> Self {
        Self {
            registry: Arc::new(RwLock::new(None)),
        }
    }

    /// Set the registry after construction to break circular dependency.
    ///
    /// # Arguments
    /// `registry` is a weak reference to the owning tool registry.
    /// # Returns
    /// Nothing; subsequent calls use this registry.
    /// # Examples
    /// ```rust
    /// # fn bind<R: codetether_batch_tool::Registry>(registry: &std::sync::Arc<R>) {
    /// let tool = codetether_batch_tool::BatchTool::<R>::new();
    /// tool.set_registry(std::sync::Arc::downgrade(registry));
    /// # }
    /// ```
    pub fn set_registry(&self, registry: Weak<R>) {
        let mut guard = self.registry.write().unwrap_or_else(|e| e.into_inner());
        *guard = Some(registry);
    }
}

impl<R: Registry> Default for BatchTool<R> {
    fn default() -> Self {
        Self::new()
    }
}
