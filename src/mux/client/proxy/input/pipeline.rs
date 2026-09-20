//! Stateful local input transformations for one mux attachment.

use super::Resolver;

pub(in crate::mux::client::proxy) struct Pipeline {
    detach: super::super::detach::Detector,
    paste: Resolver,
}

impl Pipeline {
    pub(in crate::mux::client::proxy) fn new() -> Self {
        Self {
            detach: super::super::detach::Detector::new(),
            paste: Resolver::default(),
        }
    }

    pub(in crate::mux::client::proxy) fn filter(
        &mut self,
        raw: &[u8],
    ) -> super::super::detach::Filtered {
        let mut filtered = self.detach.filter(raw);
        filtered.data = self.paste.resolve(filtered.data);
        filtered
    }
}
