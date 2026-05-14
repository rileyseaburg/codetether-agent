mod focus;
mod pointer;
mod scroll;

pub(in crate::browser::session::native) use focus::{blur, focus};
pub(in crate::browser::session::native) use pointer::{click, hover};
pub(in crate::browser::session::native) use scroll::scroll;
