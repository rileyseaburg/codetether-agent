mod send;
mod verbs;

pub(in crate::browser::session::native) use send::send;
pub(in crate::browser::session::native) use verbs::{axios, fetch, xhr};
