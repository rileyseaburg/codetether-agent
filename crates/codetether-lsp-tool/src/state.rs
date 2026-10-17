//! Shared LSP manager cache and monotonic access counter.
use codetether_lsp_core::manager::{LspManager, ManagedClient};
use std::{
    collections::HashMap,
    sync::{Arc, OnceLock, atomic::AtomicU64},
};
use tokio::sync::RwLock;
#[allow(type_alias_bounds)]
pub(crate) type Managers<C: ManagedClient> =
    Arc<RwLock<HashMap<String, (u64, Arc<LspManager<C>>)>>>;
pub(crate) const MAX_LSP_MANAGERS: usize = 8;
/// One host's manager cache and access counter.
///
/// # Examples
/// ```rust
/// # fn example<C: codetether_lsp_core::manager::ManagedClient>() {
/// let _state = codetether_lsp_tool::State::<C>::new();
/// # }
/// ```
pub struct State<C: ManagedClient> {
    pub(crate) managers: OnceLock<Managers<C>>,
    pub(crate) access: AtomicU64,
}
impl<C: ManagedClient> State<C> {
    /// Construct an empty cache with an initial access counter of zero.
    ///
    /// # Examples
    /// ```rust
    /// # fn example<C: codetether_lsp_core::manager::ManagedClient>() {
    /// let _state = codetether_lsp_tool::State::<C>::new();
    /// # }
    /// ```
    pub const fn new() -> Self {
        Self {
            managers: OnceLock::new(),
            access: AtomicU64::new(0),
        }
    }
}
impl<C: ManagedClient> Default for State<C> {
    fn default() -> Self {
        Self::new()
    }
}
