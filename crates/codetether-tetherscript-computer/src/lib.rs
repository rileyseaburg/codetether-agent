//! TetherScript computer capability with scope, origin and host policy checks.
//!
//! [ComputerGrant] carries configuration; [ComputerAuthority] validates and
//! narrows grants before dispatching to [Host]. Payload conversion, authorization,
//! runtime selection and backend dispatch are separated into focused modules.
mod computer;
mod computer_debug;
mod computer_dispatch;
mod computer_invoke;
mod computer_lists;
mod computer_narrow;
mod computer_origin;
mod computer_payload;
mod computer_policy;
mod computer_scopes;
mod computer_value;
mod host;
pub use computer::{ComputerAuthority, ComputerGrant};
pub use host::Host;
