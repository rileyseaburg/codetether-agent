use super::{device::Device, error::Error, http, validation};
use codetether_companion_protocol::DeviceCommand;
use tokio_util::sync::CancellationToken;

impl Device {
    /// Polls capture IDs and owner-approved plain text, never the owner's question.
    pub(crate) async fn poll(&self, cancel: &CancellationToken) -> Result<super::Commands, Error> {
        self.live(cancel)?;
        let request = self.authorize(self.agent.get(self.path("commands")))?;
        let command: DeviceCommand = http::receive(request, 200, cancel).await?;
        let command = super::Commands::from(command);
        self.live(cancel)?;
        if let Some(id) = &command.request_id {
            validation::request_id(id)?;
        }
        if let Some(reply) = &command.reply {
            validation::request_id(&reply.id)?;
        }
        Ok(command)
    }
}
