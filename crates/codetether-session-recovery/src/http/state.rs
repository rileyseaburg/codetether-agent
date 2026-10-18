//! Control-plane configuration used to authorize session recovery requests.
/// Request state exposing the configured workspace authority.
/// # Implementors
/// Native applications provide their existing A2A server URL.
pub trait HttpState: Clone + Send + Sync + 'static {
    /// Return the configured control-plane URL, before blank-value filtering.
    fn server_url(&self) -> Option<&str>;
}
