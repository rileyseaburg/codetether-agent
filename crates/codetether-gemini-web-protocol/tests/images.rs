//! Focused image-protocol regression tests using synthetic, credential-free RPC data.
#[path = "images/continuation.rs"]
mod continuation;
#[path = "images/errors.rs"]
mod errors;
#[path = "images/framing.rs"]
mod framing;
#[path = "images/invalid_framing.rs"]
mod invalid_framing;
#[path = "images/payload.rs"]
mod payload;
#[path = "images/snapshots.rs"]
mod snapshots;
mod support;
