//! Native session view for runtime-input enrichment.
use crate::{
    provider::Message,
    session::{Session, SessionMetadata},
};
impl codetether_runtime_input::Source for Session {
    fn id(&self) -> &str {
        &self.id
    }
    fn agent(&self) -> &str {
        &self.agent
    }
    fn metadata(&self) -> &SessionMetadata {
        &self.metadata
    }
    fn messages(&self) -> &[Message] {
        &self.messages
    }
}
