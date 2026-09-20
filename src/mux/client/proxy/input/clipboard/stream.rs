//! Stateful bracketed-paste tracking across arbitrary stdin read boundaries.

#[path = "stream/markers.rs"]
mod markers;

use super::{CTRL_V, local_content, resolve_content};

#[derive(Default)]
pub(in crate::mux::client::proxy) struct Resolver {
    markers: markers::Markers,
}

impl Resolver {
    pub(in crate::mux::client::proxy) fn resolve(&mut self, data: Vec<u8>) -> Vec<u8> {
        if self.markers.observe(&data) {
            return data;
        }
        let content = data.contains(&CTRL_V).then(local_content).flatten();
        resolve_content(data, content)
    }
}

#[cfg(test)]
#[path = "stream_tests.rs"]
mod tests;
