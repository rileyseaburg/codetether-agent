//! WebSocket send/receive, terminal handling, state capture, and recycling.
//!
//! Use [drive] after connection setup; [turn_state::replay] adds sticky routing
//! metadata to subsequent handshakes.
#[path = "websocket/drive.rs"]
mod driver;
#[path = "websocket/interrupted.rs"]
mod interrupted;
#[path = "websocket/next_event.rs"]
mod next_event;
#[path = "websocket/parse.rs"]
mod parse;
#[path = "websocket/receive.rs"]
mod receive;
#[path = "websocket/recycle.rs"]
mod recycle;
#[path = "websocket/send.rs"]
mod send;
#[path = "websocket/turn_state.rs"]
pub mod turn_state;

pub use driver::run as drive;

mod handshake;
pub use handshake::build_responses_ws_request_with_base_url_and_account_id;

mod endpoints;
pub use endpoints::{
    build_chatgpt_responses_ws_request, build_responses_ws_request_with_account_id,
};
