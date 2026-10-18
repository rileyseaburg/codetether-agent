//! Attach a new specialist after successful session creation.
use crate::{Host, Session, state::Runtime};
use codetether_relay_plan::RelayProfile;
impl<H: Host> Runtime<H> {
    pub(super) async fn accept_spawn(
        &mut self,
        profile: RelayProfile,
        reason: String,
        model_ref: &str,
        idx: usize,
    ) {
        match H::Session::new().await {
            Ok(spawned_session) => {
                self.attach_spawn(spawned_session, profile, reason, model_ref, idx);
            }
            Err(err) => {
                tracing::warn!(
                    agent = %profile.name,
                    error = %err,
                    "Dynamic relay spawn requested but failed"
                );
            }
        }
    }
}
