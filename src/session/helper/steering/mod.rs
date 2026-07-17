//! Session-scoped input steering for active prompt runs.

mod drain;
mod guard;
mod input;
mod queue;

pub(crate) use drain::{drain_into, drain_or_close_into};
pub(crate) use guard::RunGuard;
pub(crate) use input::SteeringInput;
pub(crate) use queue::{clear, open, push};

#[cfg(test)]
mod tests;
