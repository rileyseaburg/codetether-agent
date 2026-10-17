//! Local peer advertisement and discovery startup.
//!
//! UDP discovery starts first; successful mDNS startup owns the shared intake.
use super::*;
mod advertise;
mod run;
mod udp;
use advertise::start as start_mdns;
pub(super) use run::start;
use udp::start as start_lan;
