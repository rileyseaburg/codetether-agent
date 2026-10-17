//! Construct the reusable avatar-service HTTP client.
use super::{AvatarTool, config::REQUEST_TIMEOUT};

impl AvatarTool {
    /// Create a reusable HTTP client with the avatar-generation timeout.
    ///
    /// # Arguments
    /// None.
    /// # Returns
    /// A tool ready for registration.
    /// # Panics
    /// Panics if the HTTP client cannot be constructed.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_avatar_tool::AvatarTool::new();
    /// ```

    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .user_agent("CodeTether-Agent/1.0")
            .build()
            .expect("Failed to build HTTP client");
        Self { client }
    }
}

impl Default for AvatarTool {
    fn default() -> Self {
        Self::new()
    }
}
