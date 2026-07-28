#![allow(dead_code)]
//! Resource loading pipeline for the tetherscript browser.

pub mod cors;
pub mod dependency;
pub mod discover;
pub mod priority;
pub mod request;
pub mod response;
pub mod scheduler;
pub mod state;

#[cfg(test)]
mod tests;

pub use cors::*;
pub use dependency::*;
pub use discover::*;
pub use priority::*;
pub use request::*;
pub use response::*;
pub use scheduler::*;
pub use state::*;

/// Coordinates discovery, scheduling and load-state bookkeeping.
pub struct ResourceLoader {
    pub scheduler: LoadScheduler,
    pub states: LoadStateTracker,
    pub graph: DependencyGraph,
}

impl ResourceLoader {
    /// Create a loader with the browser default HTTP/1.1 per-origin limit.
    pub fn new() -> Self {
        Self {
            scheduler: LoadScheduler::new(6),
            states: LoadStateTracker::new(),
            graph: DependencyGraph::new(),
        }
    }

    /// Scan a document and enqueue discovered resources.
    pub fn scan_document<D: core::fmt::Debug>(
        &mut self,
        document: &D,
        base_url: &str,
    ) -> Vec<ResourceRequest> {
        let requests = discover_resources(document, base_url);
        for r in &requests {
            self.states.insert(r.url.clone(), LoadStatus::Pending);
            match r.resource_type {
                ResourceType::DnsPrefetch => self.scheduler.add_dns_prefetch(&r.url),
                ResourceType::Preconnect => self.scheduler.add_preconnect(&r.url),
                _ => self.scheduler.enqueue(r.clone()),
            }
        }
        requests
    }

    /// Mark a request as loading if the scheduler can start it now.
    pub fn start_next(&mut self) -> Option<ResourceRequest> {
        let r = self.scheduler.next_startable()?;
        self.states.insert(r.url.clone(), LoadStatus::Loading);
        Some(r)
    }

    /// Complete a load and update progress/state.
    pub fn complete(&mut self, response: ResourceResponse) {
        let len = response
            .content_length
            .unwrap_or(response.body.len() as u64);
        self.scheduler.finish(&response.url);
        self.states.set_progress(&response.url, len, Some(len));
        self.states.insert(
            response.url,
            if response.status < 400 {
                LoadStatus::Loaded
            } else {
                LoadStatus::Error
            },
        );
    }
}

impl Default for ResourceLoader {
    fn default() -> Self {
        Self::new()
    }
}
