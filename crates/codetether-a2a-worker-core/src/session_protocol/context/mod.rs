//! Borrowed provider and response settings retained during a worker session.
//! [Context] groups the provider request settings with [ResponseContext], which
//! supplies the registry and execution policy used to handle a response.
mod request;
mod response;
pub use request::Context;
pub use response::ResponseContext;
