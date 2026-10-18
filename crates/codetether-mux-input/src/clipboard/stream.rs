//! Stateful bracketed-paste tracking across arbitrary stdin read boundaries.

#[path = "stream/markers.rs"]
mod markers;

use super::{CTRL_V, resolve_content};

#[derive(Default)]
pub(crate) struct Resolver {
    markers: markers::Markers,
}

impl Resolver {
    pub(crate) fn resolve(
        &mut self,
        data: Vec<u8>,
        clipboard: impl FnOnce() -> Option<String>,
    ) -> Vec<u8> {
        if self.markers.observe(&data) {
            return data;
        }
        let content = data.contains(&CTRL_V).then(clipboard).flatten();
        resolve_content(data, content)
    }
}

#[cfg(test)]
#[path = "stream_tests.rs"]
mod tests;
