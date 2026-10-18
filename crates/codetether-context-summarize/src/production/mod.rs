//! Phase-summary production from an application-captured session snapshot.
//!
//! [`Prepared`] owns the existing transcript/configuration snapshot.
//! [`produce`] forwards it to the summary producer with the caller's resolver.
mod prepared;
mod run;
pub use prepared::Prepared;
pub use run::produce;
