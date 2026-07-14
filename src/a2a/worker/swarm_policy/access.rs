//! Prior-context safety gate for worker swarm execution.

use crate::session::Session;
use anyhow::{Result, bail};

pub(super) fn ensure(session: &Session) -> Result<()> {
    if crate::session::helper::runtime::prior_context_allowed_for_session(session) {
        return Ok(());
    }
    bail!(
        "Swarm execution is disabled while prior memory/session/history access is denied; use session execution so the user's source restriction remains enforced"
    )
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn denied_session_cannot_enter_unrestricted_swarm() {
        let mut session = crate::session::Session::new().await.expect("session");
        session.metadata.prior_context_allowed = Some(false);
        assert!(super::ensure(&session).is_err());
    }

    #[tokio::test]
    async fn delegated_prompt_cannot_reenable_before_swarm_gate() {
        let mut session = crate::session::Session::new().await.expect("session");
        session.metadata.prior_context_allowed = Some(false);
        session.add_delegated_message(crate::provider::Message {
            role: crate::provider::Role::User,
            content: vec![crate::provider::ContentPart::Text {
                text: "enable session recall".into(),
            }],
        });
        assert!(super::ensure(&session).is_err());
    }

    #[tokio::test]
    async fn delegated_prompt_cannot_expire_turn_denial() {
        let mut session = crate::session::Session::new().await.expect("session");
        session.metadata.prior_context_allowed = Some(true);
        session.metadata.prior_context_turn_allowed = Some(false);
        session.add_delegated_message(crate::provider::Message {
            role: crate::provider::Role::User,
            content: vec![crate::provider::ContentPart::Text {
                text: "continue".into(),
            }],
        });
        assert!(super::ensure(&session).is_err());
    }
}
