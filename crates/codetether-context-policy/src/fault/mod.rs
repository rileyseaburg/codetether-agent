//! Observable context and recall faults with stable wire codes.
//!
//! [Fault] distinguishes missing results, denials, backend failures and context
//! residency violations; its serialized representation and display text are stable.
mod methods;
mod types;
pub use types::Fault;
#[cfg(test)]
mod tests;
