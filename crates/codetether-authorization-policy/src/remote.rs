//! Remote OPA decisions and configured failure handling.
use crate::{
    PolicyResource, PolicyUser,
    client::HTTP_CLIENT,
    config::{fail_open, opa_path, opa_url},
    wire::{self, OpaResponse},
};
pub(crate) async fn check(
    user: &PolicyUser,
    action: &str,
    resource: Option<&PolicyResource>,
) -> bool {
    let url = format!("{}/{}", opa_url(), opa_path());
    let body = wire::input(user, action, resource);
    match HTTP_CLIENT.post(&url).json(&body).send().await {
        Ok(resp) => match resp.json::<OpaResponse>().await {
            Ok(opa) => {
                let allowed = opa.result.unwrap_or(false);
                if !allowed {
                    tracing::info!(
                        user_id = %user.user_id,
                        action = %action,
                        "OPA denied"
                    );
                }
                allowed
            }
            Err(e) => {
                tracing::error!("Failed to parse OPA response: {}", e);
                fail_open()
            }
        },
        Err(e) => {
            tracing::error!("OPA request failed: {}", e);
            if fail_open() {
                tracing::warn!("OPA unreachable — failing open (ALLOW)");
                true
            } else {
                tracing::warn!("OPA unreachable — failing closed (DENY)");
                false
            }
        }
    }
}
