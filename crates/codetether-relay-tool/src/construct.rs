//! construct for relay coordination.
use super::*;
impl RelayAutoChatTool {
    /// Construct a relay tool sharing the private relay registry.
    ///
    /// # Examples
    /// ```rust
    /// let _tool = codetether_relay_tool::RelayAutoChatTool::new();
    /// ```
    pub fn new() -> Self {
        Self
    }
}

impl Default for RelayAutoChatTool {
    fn default() -> Self {
        Self::new()
    }
}
