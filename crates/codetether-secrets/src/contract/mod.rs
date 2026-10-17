//! Provider storage and manager contracts using localhost-only Vault responses.
use crate::*;
mod failures;
mod fixture;
mod global;
mod kubernetes;
mod reads;
mod response;
mod retry;
mod serialization;
mod writes;
