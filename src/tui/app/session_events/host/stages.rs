//! Native event category handlers in shared pipeline order.
use super::{Native, SessionEvent as E};
impl codetether_session_events::pipeline::Stages for Native<'_> {
    type Event = E;
    async fn lifecycle(&mut self, event: E) -> Option<E> {
        super::super::lifecycle::handle_event(self.app, self.slot, self.worker_bridge, event).await
    }
    fn interlude(&mut self, event: E) -> Option<E> {
        super::super::interlude::handle_event(self.app, event)
    }
    fn retry(&mut self, event: E) -> Option<E> {
        super::super::retry::handle_event(self.app, event)
    }
    async fn tools(&mut self, event: E) -> Option<E> {
        super::super::tools::handle_event(self.app, self.worker_bridge, event).await
    }
    fn text(&mut self, event: E) -> Option<E> {
        super::super::text_dispatch::handle_event(self.app, event)
    }
    fn usage(&mut self, event: E) -> Option<E> {
        super::super::usage::handle_event(self.app, event)
    }
    async fn errors(&mut self, event: E) -> Option<E> {
        super::super::errors::handle_event(self.app, self.slot, self.worker_bridge, event).await
    }
}
