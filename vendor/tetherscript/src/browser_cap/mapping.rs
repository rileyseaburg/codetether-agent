//! Dispatch from tetherscript browser methods to browserctl actions.

use super::{actions, args, authority, call, value};

#[path = "map_diag.rs"]
mod diag;
#[path = "map_dispatch.rs"]
mod dispatch;
#[path = "map_dom.rs"]
mod dom;
#[path = "map_extra.rs"]
mod extra;
#[path = "map_nav.rs"]
mod nav;
#[path = "map_net.rs"]
mod net;
#[path = "map_raw.rs"]
mod raw;
#[path = "map_storage.rs"]
mod storage;
#[path = "map_visual.rs"]
mod visual;

pub(crate) use dispatch::prepare;
