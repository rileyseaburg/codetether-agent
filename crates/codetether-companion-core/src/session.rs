use crate::auth::TokenHash;
use codetether_companion_protocol::SessionInput;

/// Private, memory-only session state, with no printable secret representation.
///
/// Obtain a session with [`crate::Registry::session`]; it cannot be constructed
/// or mutated externally. Dropping the registry revokes all capabilities.
///
/// ```
/// # use codetether_companion_core::Registry;
/// # use codetether_companion_protocol::SessionInput;
/// # let mut registry = Registry::default();
/// # let receipt = registry.create(SessionInput { model: "p/m".into(), prompt: "Look".into(), interval_seconds: 30 }, 0)?;
/// let session = registry.session(&receipt.id, 0)?;
/// assert!(!session.is_paired());
/// # Ok::<(), codetether_companion_core::Error>(())
/// ```
pub struct Session {
    pub(crate) input: Option<SessionInput>,
    pub(crate) code: String,
    pub(crate) pair_expires: i64,
    pub(crate) expires: i64,
    pub(crate) device: Option<TokenHash>,
    pub(crate) stopped: bool,
}
impl Session {
    /// Whether this session currently has a device capability.
    pub fn is_paired(&self) -> bool {
        self.device.is_some() && !self.stopped
    }
    /// Validated owner instructions; absent after revocation.
    pub fn input(&self) -> Option<&SessionInput> {
        self.input.as_ref()
    }
    pub(crate) fn stop(&mut self) {
        self.stopped = true;
        self.code.clear();
        self.device = None;
        self.input = None;
    }
}
