//! Shared mux protocol version, messages and JSON-lines framing.
pub(super) use codetether_mux_protocol::{
    AgentRequest, AgentResponse, ClientRequest, ProgramRequest, ServerResponse, VERSION,
    read_frame, write_frame,
};
