//! Direct native Bonsai generation without agent or credential startup.
//!
//! [BonsaiArgs] configure a user-only request and [run] streams repeated requests
//! through one resident [codetether_provider_bonsai::BonsaiProvider], reporting
//! per-request timings. Provider feature selection remains with the application.
mod args;
mod request;
mod run;
mod stream;
pub use args::BonsaiArgs;
pub use run::execute as run;
