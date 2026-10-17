//! Update memory metadata and track accesses.
use super::MemoryEntry;
use chrono::Utc;
impl MemoryEntry {
    /// Assign the record's scope.
    ///
    /// # Examples
    /// ```rust
    /// let entry = codetether_memory_core::MemoryEntry::new("text", vec![]).with_scope("project");
    /// assert_eq!(entry.scope.as_deref(), Some("project"));
    /// ```
    pub fn with_scope(mut self, scope: impl Into<String>) -> Self {
        self.scope = Some(scope.into());
        self
    }

    /// Set the source of this memory entry
    #[allow(dead_code)]
    ///
    /// # Examples
    /// ```rust
    /// let entry = codetether_memory_core::MemoryEntry::new("text", vec![]).with_source("session");
    /// assert_eq!(entry.source.as_deref(), Some("session"));
    /// ```
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Assign importance, capped at five.
    ///
    /// # Examples
    /// ```rust
    /// let entry = codetether_memory_core::MemoryEntry::new("text", vec![]).with_importance(9);
    /// assert_eq!(entry.importance, 5);
    /// ```
    pub fn with_importance(mut self, importance: u8) -> Self {
        self.importance = importance.min(5);
        self
    }

    /// Update the access timestamp and increment the access count.
    ///
    /// # Examples
    /// ```rust
    /// let mut entry = codetether_memory_core::MemoryEntry::new("text", vec![]);
    /// entry.touch();
    /// assert_eq!(entry.access_count, 1);
    /// ```
    pub fn touch(&mut self) {
        self.accessed_at = Utc::now();
        self.access_count += 1;
    }
}
