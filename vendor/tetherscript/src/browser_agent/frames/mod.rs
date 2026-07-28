//! Frame identity and tree metadata for agent-controlled pages.
//!
//! The module models browsing-context relationships without owning JS runtime,
//! DOM, or network state. Higher-level page code can attach those pieces later.

mod attrs;
mod build;
mod context_opener;
mod frame;
mod id;
mod insert;
mod message;
mod message_queue;
mod nav;
mod origin;
mod page;
mod page_messages;
mod page_window;
mod path_id;
mod traversal;
mod tree;
mod walk;
mod window;
mod window_state;

pub use frame::BrowserFrame;
pub use id::FrameId;
pub use message::FrameMessage;
pub use tree::FrameTree;
pub use window::{FrameWindowRelation, WindowOpener};
pub(crate) use window_state::FrameWindowState;

#[cfg(test)]
mod message_policy_tests;
#[cfg(test)]
mod message_tests;
#[cfg(test)]
mod opener_tests;
#[cfg(test)]
mod page_tests;
#[cfg(test)]
mod tests;
