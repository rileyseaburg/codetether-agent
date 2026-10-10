//! Device-capability checks scoped to a single session.
use crate::{ApiError, runtime::Runtime, state::State};

impl State {
    /// Live runtime after verifying this session's device bearer only.
    pub(crate) fn device(
        &mut self,
        id: &str,
        auth: Option<&str>,
        now: i64,
    ) -> Result<&mut Runtime, ApiError> {
        self.live(id, now)?;
        self.registry.authorize_device(id, auth, now)?;
        self.live(id, now)
    }
    /// Whether the session currently holds a device capability.
    pub(crate) fn paired(&mut self, id: &str, now: i64) -> bool {
        self.registry
            .session(id, now)
            .is_ok_and(|session| session.is_paired())
    }
}
