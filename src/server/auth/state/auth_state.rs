use super::{default_public_paths, load_token};
use std::{collections::HashSet, sync::Arc};

/// Shared authentication state for the HTTP server.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::server::auth::AuthState;
///
/// let state = AuthState::with_token("example-token");
///
/// assert_eq!(state.token(), "example-token");
/// assert!(state.is_public_path("/health"));
/// ```
#[derive(Debug, Clone)]
pub struct AuthState {
    token: Arc<String>,
    public_paths: Arc<HashSet<String>>,
}

impl AuthState {
    /// Build auth state from environment variables.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::server::auth::AuthState;
    ///
    /// # unsafe {
    /// std::env::set_var("CODETETHER_AUTH_TOKEN", "example-token");
    /// # }
    /// let state = AuthState::from_env();
    ///
    /// assert_eq!(state.token(), "example-token");
    /// ```
    pub fn from_env() -> Self {
        let token = load_token();
        Self {
            token: Arc::new(token),
            public_paths: Arc::new(default_public_paths()),
        }
    }

    /// Build auth state with an explicit token.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::server::auth::AuthState;
    ///
    /// let state = AuthState::with_token("example-token");
    /// assert_eq!(state.token(), "example-token");
    /// ```
    pub fn with_token(token: impl Into<String>) -> Self {
        Self {
            token: Arc::new(token.into()),
            public_paths: Arc::new(default_public_paths()),
        }
    }

    /// Return the active bearer token.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::server::auth::AuthState;
    ///
    /// # unsafe {
    /// std::env::set_var("CODETETHER_AUTH_TOKEN", "example-token");
    /// # }
    /// let state = AuthState::from_env();
    ///
    /// assert_eq!(state.token(), "example-token");
    /// ```
    pub fn token(&self) -> &str {
        &self.token
    }

    /// Return whether a request path is exempt from authentication.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::server::auth::AuthState;
    ///
    /// let state = AuthState::with_token("example-token");
    ///
    /// assert!(state.is_public_path("/health"));
    /// assert!(!state.is_public_path("/secure"));
    /// ```
    pub fn is_public_path(&self, path: &str) -> bool {
        self.public_paths.contains(path)
    }
}
