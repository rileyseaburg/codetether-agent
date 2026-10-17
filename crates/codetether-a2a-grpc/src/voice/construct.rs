//! Voice transport construct.
use super::*;

impl VoiceServiceImpl {
    pub fn new(bus: Arc<AgentBus>) -> Self {
        Self { bus }
    }

    pub fn into_service(self) -> VoiceServiceServer<Self> {
        VoiceServiceServer::new(self)
    }
}
