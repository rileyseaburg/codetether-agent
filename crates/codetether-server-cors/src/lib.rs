//! Browser CORS policy for the HTTP and gRPC-Web server listeners.
//!
//! [http()] returns the credentialed main API allowlist. [grpc()] returns the
//! separate gRPC-Web origin policy with exposed status headers.
mod grpc;
mod http;
pub use grpc::grpc;
pub use http::http;
