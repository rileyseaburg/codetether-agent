//! JSON contracts for the Rust companion and relay migration.
//!
//! These types mirror the TypeScript relay and Swift/C# clients. They perform
//! structural deserialization, not semantic validation or authorization. Image
//! bounds, freshness, consent, and budgets remain the runtime's responsibility.
//! No type here starts capture, invokes inference, or persists credentials.
//!
//! ```
//! use codetether_companion_protocol::{DeviceCommand, EventKind};
//! assert_eq!(DeviceCommand { request_id: None, reply: None }.request_id, None);
//! assert!(matches!(EventKind::Delta, EventKind::Delta));
//! ```

mod capture;
mod command;
mod event;
mod pairing;
mod reply;
mod response;
mod session;

pub use capture::{Capture, CaptureTrigger};
pub use command::{CaptureRequest, CaptureRequestReceipt, DeviceCommand};
pub use event::{EventKind, ScreenEvent};
pub use pairing::{PairReceipt, PairRequest};
pub use reply::{DeviceReply, ReplyReceipt, ReplyRequest, TypedAck};
pub use response::{Accepted, ErrorResponse, Paused, Stopped, Typed};
pub use session::{SessionInput, SessionReceipt};
