//! Viewport, hit-test, and media-query browser globals.
//!
//! The standalone JS runtime currently keeps a fixed layout viewport width and
//! no page-level viewport height. Height therefore follows the deterministic
//! browser-agent default until runtime viewport plumbing is shared.

use super::*;

#[path = "constants.rs"]
mod constants;
#[path = "document.rs"]
mod document;
#[path = "hit.rs"]
mod hit;
#[path = "hit_collect.rs"]
mod hit_collect;
#[path = "hit_style.rs"]
mod hit_style;
#[path = "media_dispatch.rs"]
mod media_dispatch;
#[path = "media_event.rs"]
mod media_event;
#[path = "media_legacy.rs"]
mod media_legacy;
#[path = "media_object.rs"]
mod media_object;
#[path = "media_query.rs"]
mod media_query;
#[path = "media_window.rs"]
mod media_window;
#[path = "metrics.rs"]
mod metrics;
#[path = "point.rs"]
mod point;
#[path = "screen.rs"]
mod screen;
#[path = "visual_viewport.rs"]
mod visual_viewport;

pub(super) fn install_document(document: &JsValue, root: Rc<RefCell<Node>>) {
    document::install(document, root);
}

pub(super) fn install_window(window: &mut HashMap<String, JsValue>) {
    screen::install(window);
    metrics::install(window);
    visual_viewport::install(window);
    media_window::install(window);
}

#[cfg(test)]
#[path = "tests_hit.rs"]
mod tests_hit;

#[cfg(test)]
#[path = "tests_media.rs"]
mod tests_media;

#[cfg(test)]
#[path = "tests_metrics.rs"]
mod tests_metrics;
