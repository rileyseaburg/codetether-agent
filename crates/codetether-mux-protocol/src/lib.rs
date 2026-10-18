//! Versioned mux control messages and bounded JSON-lines framing.
//!
//! [ClientRequest] and [ServerResponse] compose shared model and lease records.
//! [read_frame] and [write_frame] transport those messages under a fixed byte cap.
mod agent_request;
mod agent_response;
mod program_request;
mod read;
mod request;
mod response;
mod write;
pub use agent_request::AgentRequest;
pub use agent_response::AgentResponse;
pub use program_request::ProgramRequest;
pub use read::read_frame;
pub use request::ClientRequest;
pub use response::ServerResponse;
pub use write::write_frame;
/// Current mux client/server protocol version.
pub const VERSION: u16 = 8;
const MAX_FRAME_BYTES: usize = 1024 * 1024;
#[cfg(test)]
mod agent_output_carries_replay_and_completion_state_test;
#[cfg(test)]
mod agent_start_round_trips_over_authenticated_wire_shape_test;
#[cfg(test)]
mod coordination_request_round_trips_over_json_frame_test;
#[cfg(test)]
mod request_round_trips_over_json_frame_test;
#[cfg(test)]
mod runtime_status_round_trips_over_json_frame_test;
#[cfg(test)]
mod semantic_steer_round_trips_over_json_frame_test;
