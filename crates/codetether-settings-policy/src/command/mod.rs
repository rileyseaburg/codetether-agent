//! Interactive boolean setting command policy.
//!
//! [`execute`] parses a [`Setting`] command and uses a [`Host`] to read the
//! current value, apply changes or display the existing status text.
mod execute;
mod host;
mod parse;
mod setting;
pub use execute::execute;
pub use host::Host;
pub use setting::Setting;
