//! Loopback HTTP/SSE relay for the screen companion.
//!
//! Wire-compatible Rust port of `scripts/public-server/companion`. It wraps
//! [`codetether_companion_core::Registry`] (owner/device authentication and
//! session lifecycle) with per-session runtime state, bounded capture
//! validation, owner fresh-frame requests, SSE fan-out, and direct streamed
//! vision inference (`tools: []`, no agent loop). Everything is memory-only;
//! dropping the process revokes every session.
//!
//! ```no_run
//! # async fn demo() -> anyhow::Result<()> {
//! use codetether_companion_relay::{Relay, router, vision_analyzer};
//! let token = std::env::var("CODETETHER_AUTH_TOKEN")?;
//! let analyze = vision_analyzer(&token, "https://server.codetether.run")?;
//! let relay = Relay::new(&token, analyze, "https://server.codetether.run", None)?;
//! let app = router(relay);
//! # let _ = app; Ok(()) }
//! ```
mod admission;
mod analysis;
mod analysis_finish;
mod analyze;
mod assets;
mod assets_headers;
mod body;
mod capture;
mod commands;
mod delta;
mod device;
mod error;
mod events;
mod fixed;
mod frame;
mod frame_fields;
mod jpeg;
mod model_typing;
mod owner;
mod paths;
mod poll;
mod relay;
mod replies;
mod reply;
mod routes;
mod runtime;
mod signal;
mod sse;
mod state;
mod state_device;
mod state_ops;
mod stop;
mod typing_proposal;
mod upstream;
mod upstream_body;
mod upstream_sse;

pub use analyze::{Analysis, Analyze, Delta};
pub use error::ApiError;
pub use relay::{Relay, Shared, now, shutdown, spawn_sweeper};
pub use routes::router;
pub use signal::terminate;
pub use upstream::vision_analyzer;
pub use upstream_sse::SseParser;
