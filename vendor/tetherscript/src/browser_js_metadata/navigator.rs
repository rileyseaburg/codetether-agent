//! Navigator identity metadata and beacon registration.

use std::collections::HashMap;

use crate::js::JsValue;

use super::super::{native, SharedBrowserJsRouteHandler};
use super::set_str;

#[path = "navigator/battery.rs"]
mod battery;
#[path = "navigator/capabilities.rs"]
mod capabilities;
#[path = "navigator/connection.rs"]
mod connection;
#[path = "navigator/extras.rs"]
mod extras;
#[path = "navigator/identity.rs"]
mod identity;
#[path = "navigator/install.rs"]
mod installer;
#[path = "navigator/locks.rs"]
mod locks;
#[path = "navigator/media_capabilities.rs"]
mod media_capabilities;
#[path = "navigator/media_session.rs"]
mod media_session;
#[path = "navigator/permissions.rs"]
mod permissions;
#[path = "navigator/rejection.rs"]
mod rejection;
#[path = "navigator/scheduling.rs"]
mod scheduling;
#[path = "navigator/share.rs"]
mod share;
#[path = "navigator/storage.rs"]
mod storage;
#[path = "navigator/thenable.rs"]
mod thenable;
#[path = "navigator/user_agent_data.rs"]
mod user_agent_data;
#[path = "navigator/vibration.rs"]
mod vibration;
#[path = "navigator/wake_lock.rs"]
mod wake_lock;

pub(super) fn install(navigator: &JsValue, route_handler: SharedBrowserJsRouteHandler) {
    installer::install(navigator, route_handler);
}
