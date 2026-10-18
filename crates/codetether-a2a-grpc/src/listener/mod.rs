//! Background gRPC-Web startup for the A2A and voice services.
//!
//! [spawn] resolves the listener address and starts both services on the supplied
//! bus with the shared gRPC-Web CORS policy.
mod address;
mod spawn;
pub use spawn::spawn;
