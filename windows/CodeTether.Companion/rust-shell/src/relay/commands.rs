//! Runtime command ownership erases received text without cloning it.
use codetether_companion_protocol::DeviceCommand;
use zeroize::Zeroizing;

pub(crate) struct Reply {
    pub(crate) id: String,
    pub(crate) text: Zeroizing<String>,
}
pub(crate) struct Commands {
    pub(crate) request_id: Option<String>,
    pub(crate) reply: Option<Reply>,
}
impl From<DeviceCommand> for Commands {
    fn from(command: DeviceCommand) -> Self {
        Self {
            request_id: command.request_id,
            reply: command.reply.map(|reply| Reply {
                id: reply.id,
                text: Zeroizing::new(reply.text),
            }),
        }
    }
}
