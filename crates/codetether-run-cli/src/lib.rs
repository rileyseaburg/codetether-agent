//! Non-interactive command preparation, routing and failure reporting.
//!
//! [execute] selects the Sol workflow before ordinary preparation, then routes
//! validated relay requests or normal prompts through the bound [Host] types.
mod entry;
mod host;
mod normal;
mod prepare;
mod prepared;
mod relay;
pub use entry::execute;
pub use host::Host;
