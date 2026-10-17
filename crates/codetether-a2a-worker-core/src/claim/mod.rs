//! Shared task-claim response and provenance conversion.
//!
//! [TaskClaimResponse] preserves the control-plane wire fields. Its conversion
//! uses an explicit agent identity, falling back to the returned user ID.
mod provenance;
mod record;
pub use record::TaskClaimResponse;
#[cfg(test)]
mod tests;
