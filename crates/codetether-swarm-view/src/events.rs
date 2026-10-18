//! Route swarm events to their state reducers.
use crate::{SwarmEvent, SwarmViewState};
impl SwarmViewState {
    /// Apply one event to the cached swarm state.
    ///
    /// # Examples
    /// ```
    /// use codetether_swarm_view::{SwarmEvent,SwarmViewState};
    /// let mut state = SwarmViewState::new();
    /// state.handle_event(SwarmEvent::Error("unavailable".into()));
    /// assert_eq!(state.error.as_deref(), Some("unavailable"));
    /// assert!(state.complete);
    /// ```
    pub fn handle_event(&mut self, event: SwarmEvent) {
        match event {
            event @ (SwarmEvent::Started { .. } | SwarmEvent::Decomposed { .. }) => {
                self.apply_run_start(event)
            }
            event @ (SwarmEvent::SubTaskUpdate { .. } | SwarmEvent::AgentStarted { .. }) => {
                self.apply_agent_start(event)
            }
            event @ (SwarmEvent::AgentToolCall { .. }
            | SwarmEvent::AgentToolCallDetail { .. }
            | SwarmEvent::AgentMessage { .. }) => self.apply_agent_activity(event),
            event @ SwarmEvent::AgentComplete { .. } => self.apply_agent_complete(event),
            event @ (SwarmEvent::AgentOutput { .. } | SwarmEvent::AgentError { .. }) => {
                self.apply_agent_results(event)
            }
            event @ (SwarmEvent::StageComplete { .. }
            | SwarmEvent::Complete { .. }
            | SwarmEvent::Error(..)) => self.apply_run_progress(event),
        }
    }
}
