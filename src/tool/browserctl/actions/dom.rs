mod motion;
mod read;
mod write;

pub(in crate::tool::browserctl) use motion::{blur, focus, hover, scroll};
pub(in crate::tool::browserctl) use read::{html, text};
pub(in crate::tool::browserctl) use write::{click, fill, press, type_text};
