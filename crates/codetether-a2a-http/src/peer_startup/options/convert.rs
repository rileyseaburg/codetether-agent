//! Conversion from application-owned command arguments.
use super::SpawnOptions;
impl SpawnOptions {
    /// Convert application arguments through their existing options mapping.
    ///
    /// # Arguments
    ///
    /// * `args` - Arguments implementing conversion into startup options.
    ///
    /// # Returns
    ///
    /// The converted peer options.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_a2a_http::peer_startup::SpawnOptions;
    /// let options = SpawnOptions::from_spawn_args(SpawnOptions::auto());
    /// assert!(options.mdns);
    /// ```
    pub fn from_spawn_args(args: impl Into<Self>) -> Self {
        args.into()
    }
}
