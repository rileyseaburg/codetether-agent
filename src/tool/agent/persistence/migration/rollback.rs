//! Best-effort rollback for an interrupted multi-manifest ownership transfer.

use super::{Manifest, atomic, paths};

pub(super) async fn run(manifests: &[Manifest], from: &str, to: &str) {
    for original in manifests.iter().rev() {
        let Ok(path) = paths::manifest(&original.child_session_id) else {
            tracing::error!(child = %original.child_session_id, "Invalid child id during rollback");
            continue;
        };
        if let Err(error) = atomic::write(&path, original).await {
            tracing::error!(child = %original.child_session_id, %error, "Agent owner rollback failed");
        }
        if let Err(error) =
            super::super::super::collaboration_runtime::message_queue::reparent_owner(
                &original.child_session_id,
                to,
                from,
            )
            .await
        {
            tracing::error!(child = %original.child_session_id, %error, "Mailbox owner rollback failed");
        }
    }
}
