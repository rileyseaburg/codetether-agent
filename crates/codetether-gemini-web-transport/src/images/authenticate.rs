use super::{ImageClient, client::State, tokens::Tokens};
use anyhow::{Error, Result};

pub(super) fn retry_cached_anonymous(cached: bool, error: &Error) -> bool {
    cached && error.to_string() == "image_session_not_authenticated"
}

impl ImageClient {
    pub(super) async fn authenticate(&self, state: &mut State) -> Result<Tokens> {
        let cached = match self.refresh(state, false).await {
            Ok(cached) => cached,
            Err(error)
                if super::renewal_throttle::cached_session_allowed(
                    &error,
                    state.rotated_at,
                    &state.updates,
                    super::cookies::now(),
                ) =>
            {
                // Valid cached cookies may outlive the renewal cooldown. The
                // page must still authenticate; never retry renewal or bypass it.
                tracing::debug!(
                    provider = "gemini-web",
                    status = 429,
                    "Cookie renewal throttled; checking the existing session"
                );
                return self.page(state).await;
            }
            Err(error) => return Err(error),
        };
        match self.page(state).await {
            Ok(tokens) => Ok(tokens),
            Err(error) if retry_cached_anonymous(cached, &error) => {
                // A fresh cache can still contain a server-invalidated rotation.
                // Retry only bootstrap: never replay a generation request.
                self.refresh(state, true).await?;
                self.page(state).await
            }
            Err(error) => Err(error),
        }
    }
}
