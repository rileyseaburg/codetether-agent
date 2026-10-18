//! Construct a stateless token display.

impl crate::TokenDisplay {
    /// Construct a stateless display.
    /// # Arguments
    /// None.
    /// # Returns
    /// A presenter reading shared telemetry on demand.
    /// # Examples
    /// ```
    /// let _display = codetether_token_display::TokenDisplay::new();
    /// ```
    pub fn new() -> Self {
        Self
    }

    /// Get context limit for a model by delegating to the canonical
    /// [`codetether_provider_core::limits::context_window_for_model`].
    ///
    /// Returns `None` only for the zero-length model string edge case;
    /// callers that relied on the old `HashMap::get` returning `None`
    /// for unknown models will now get the default 128 000 instead,
    /// which is strictly more correct (all models have *some* context
    /// window).
    /// Resolve a model context limit.
    /// # Arguments
    /// * `model` - Canonical or aliased model name.
    /// # Returns
    /// The shared model limit, or None for an empty model name.
    /// # Examples
    /// ```
    /// assert_eq!(codetether_token_display::TokenDisplay::new().get_context_limit(""), None);
    /// ```
    pub fn get_context_limit(&self, model: &str) -> Option<u64> {
        if model.is_empty() {
            return None;
        }
        Some(codetether_provider_core::limits::context_window_for_model(model) as u64)
    }
}
