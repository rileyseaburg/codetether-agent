//! Ordered session utility commands and alias rerouting.
//!
//! [run] gives access-mode commands precedence, then routes status, diff, copy,
//! review and session aliases through [Host]. Review composition lives in [review].
mod host;
pub mod review;
mod run;
pub use host::Host;
pub use run::run;
