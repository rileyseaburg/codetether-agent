//! Canonical JSON-body binding and possession proofs for worker requests.
//!
//! [for_json] binds a task ID to serialized JSON. [sign] signs the protocol
//! payload. Callers retain credential lookup, timestamps, and HTTP headers.
mod resource;
mod sign;
pub use resource::for_json;
pub use sign::sign;
