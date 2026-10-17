//! Authenticated device HTTP operations scoped to one paired session.
//!
//! Pairing exchanges a code for a token; session dispatch delegates reply
//! acknowledgements and frame uploads while retaining command polling.
mod pair;
mod session;
mod typed;
mod upload;
pub(crate) use pair::pair;
pub(crate) use session::session;
