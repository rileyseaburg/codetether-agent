//! Focused legacy action handler exports.

#[path = "handlers/host.rs"]
mod host;
#[path = "handlers/interrupt.rs"]
mod interrupt;
#[path = "handlers/kill.rs"]
mod kill;
#[path = "handlers/list.rs"]
mod list;

pub(super) use interrupt::handle as handle_interrupt;
pub(super) use kill::handle as handle_kill;
pub(super) use list::handle as handle_list;
