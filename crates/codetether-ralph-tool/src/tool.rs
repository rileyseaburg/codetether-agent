//! Tool construction and provider configuration.

use crate::Host;
use codetether_provider_core::Provider;
use std::{marker::PhantomData, sync::Arc};

/// Tool for running the Ralph autonomous agent loop.
///
/// # Examples
/// ```
/// use codetether_ralph_tool::{RalphTool,Host};
/// fn construct<H:Host>()->RalphTool<H> {RalphTool::new()}
/// ```
pub struct RalphTool<H: Host> {
    pub(crate) provider: Option<Arc<dyn Provider>>,
    pub(crate) model: String,
    host: PhantomData<fn() -> H>,
}

impl<H: Host> Default for RalphTool<H> {
    fn default() -> Self {
        Self::new()
    }
}

impl<H: Host> RalphTool<H> {
    /// Construct a tool without a provider.
    pub fn new() -> Self {
        Self {
            provider: None,
            model: String::new(),
            host: PhantomData,
        }
    }

    /// Create with a specific provider and model
    pub fn with_provider(provider: Arc<dyn Provider>, model: String) -> Self {
        Self {
            provider: Some(provider),
            model,
            host: PhantomData,
        }
    }

    /// Set the provider after construction
    #[allow(dead_code)]
    pub fn set_provider(&mut self, provider: Arc<dyn Provider>, model: String) {
        self.provider = Some(provider);
        self.model = model;
    }
}
