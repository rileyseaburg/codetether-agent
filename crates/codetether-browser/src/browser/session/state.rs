//! Shared browser session state.
//!
//! The public session handle is cloneable while the backend-specific runtime
//! state remains behind internal synchronization.

use std::sync::Arc;
use tokio::sync::Mutex;

/// Cloneable handle for executing browser commands.
///
/// # Examples
///
/// ```rust
/// use codetether_browser::BrowserSession;
///
/// let session = BrowserSession::new();
/// let clone = session.clone();
/// drop(clone);
/// ```
#[derive(Clone, Default)]
pub struct BrowserSession {
    pub(super) inner: Arc<SessionInner>,
}

/// Internal backend state stored by a browser session.
#[derive(Default)]
pub(super) struct SessionInner {
    #[cfg(feature = "tetherscript")]
    pub native: Mutex<Option<super::native::NativeRuntime>>,
}

impl BrowserSession {
    /// Create an empty browser session handle.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_browser::BrowserSession;
    ///
    /// let session = BrowserSession::new();
    /// let _ = session.clone();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Execute a browser command against this session.
    ///
    /// # Errors
    ///
    /// Returns [`crate::browser::BrowserError`] when the backend is not started
    /// or the requested command cannot be completed.
    pub async fn execute(
        &self,
        command: crate::browser::BrowserCommand,
    ) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
        super::runtime::execute(self, command).await
    }
}
