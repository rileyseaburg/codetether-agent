//! Agent-side selector extensions layered over the deterministic CSS matcher.

mod args;
pub mod heal;
mod matches;
mod order;
mod parse;
mod pseudo;
mod simple;
mod state;
mod text;
mod types;
mod visible;

pub(crate) use matches::matches;
pub(crate) use order::apply as apply_order;
