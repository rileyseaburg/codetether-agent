/// Read a session identifier without requiring its full debug representation.
/// # Implementors
/// Native sessions expose the identifier already used for runtime routing.
pub trait SessionIdentity {
    /// Stable identifier included in prompt-request diagnostics.
    fn session_id(&self) -> &str;
}
