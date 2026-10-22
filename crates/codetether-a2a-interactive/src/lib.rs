//! Interactive A2A attachment, admission, dispatch and reply settlement.
//!
//! [command::handle] controls live attachment. [live::admit] queues parked
//! turns, [trigger_next] sends work through the frontend, and [live::settle]
//! resolves its waiting peer. Frontends retain provider/session ownership.
mod binding;
pub mod command;
mod delivered;
mod dispatch;
mod host;
pub mod live;
pub use delivered::already_delivered;
pub use dispatch::trigger_next;
pub use host::{Dispatch, Inbox, Presentation};
