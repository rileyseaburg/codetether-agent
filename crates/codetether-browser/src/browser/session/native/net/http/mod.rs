mod send;
mod verbs;

pub(super) use send::send;
pub(super) use verbs::{axios, fetch, xhr};
