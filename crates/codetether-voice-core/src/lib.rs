//! Voice command interpretation and audio transport policy.
//!
//! Parse speech with [dictation], map commands with [command_router], choose
//! frame routing with [transport], and inspect supported speech models with
//! [stt_catalog]. Native audio capture belongs to the host application.
pub mod command_router;
pub mod dictation;
pub mod ssh_env;
pub mod stt_catalog;
pub mod transport;
