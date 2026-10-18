//! Argument schema and execution for persistent mux CLI commands.
//!
//! [MuxArgs] parses lifecycle commands. Implement [Host] for application-owned
//! session lookup and clipboard access, then call [execute] with [MuxCommand].
mod args;
mod attach;
mod attachment;
mod command;
mod dispatch;
mod host;
mod list;
mod new_session;
mod resume;
pub mod resume_name;
mod serve;
mod start_options;
pub use args::MuxArgs;
pub use command::MuxCommand;
pub use dispatch::execute;
pub use host::Host;
pub use start_options::MuxStartOptions;

mod list_entry;
mod list_render;
