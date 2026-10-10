use super::{device::Device, error::Error, http};
use codetether_companion_protocol::Paused;
use tokio_util::sync::CancellationToken;

impl Device {
    /// Best-effort relay notification after local capture has already stopped.
    pub(crate) async fn pause_remote(&self, cancel: &CancellationToken) -> Result<(), Error> {
        self.live(cancel)?;
        let request = self
            .authorize(self.agent.post(self.path("pause")))?
            .header("Content-Type", "application/json")
            .body("{}");
        let paused: Paused = http::receive(request, 200, cancel).await?;
        if !paused.paused {
            return Err(Error::InvalidResponse);
        }
        Ok(())
    }
}
