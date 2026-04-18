mod lifecycle;
mod page;

pub(in crate::tool::browserctl) use lifecycle::{console, health, snapshot, start, stop};
pub(in crate::tool::browserctl) use page::{back, goto, reload};
