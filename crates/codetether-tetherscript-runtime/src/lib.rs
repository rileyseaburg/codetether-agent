//! TetherScript execution with explicit capability grants and bounded output.
//!
//! [run] chooses the capability host when browser or computer access is granted,
//! otherwise a raw interpreter. [BrowserGrant] and [ComputerGrant] describe those
//! grants; [Host] supplies computer dispatch, and [TetherScriptOutcome] carries
//! text, JSON return values and success state.
mod browser;
mod dispatch;
mod finish;
mod host;
mod host_grants;
mod interp;
mod outcome;
mod parse;
mod unwind;
pub use browser::BrowserGrant;
pub use codetether_tetherscript_computer::{ComputerGrant, Host};
pub use dispatch::run;
pub use outcome::TetherScriptOutcome;
