//! Browser-style event dispatch scaffolding.
//!
//! This module models enough of the DOM event flow for host integrations to
//! build deterministic tests and adapters around capture, target, and bubble
//! phases without requiring a concrete DOM implementation.

/// Identifier for nodes that participate in an event path.
pub type NodeId = u64;

/// The phase currently being dispatched.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventPhase {
    /// No dispatch is currently in progress.
    None,
    /// Ancestors are visited from root toward the target.
    Capture,
    /// The target node is being visited.
    Target,
    /// Ancestors are visited from parent back toward the root.
    Bubble,
}

/// A browser-style event with propagation/default-action state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    event_type: String,
    target: NodeId,
    current_target: Option<NodeId>,
    phase: EventPhase,
    path: Vec<NodeId>,
    default_prevented: bool,
    propagation_stopped: bool,
}

impl Event {
    /// Create an event targeted at `target`.
    pub fn new(event_type: impl Into<String>, target: NodeId) -> Self {
        Self {
            event_type: event_type.into(),
            target,
            current_target: None,
            phase: EventPhase::None,
            path: vec![target],
            default_prevented: false,
            propagation_stopped: false,
        }
    }

    pub fn event_type(&self) -> &str {
        &self.event_type
    }

    pub fn target(&self) -> NodeId {
        self.target
    }

    pub fn current_target(&self) -> Option<NodeId> {
        self.current_target
    }

    pub fn phase(&self) -> EventPhase {
        self.phase
    }

    /// Event path ordered from root to target.
    pub fn path(&self) -> &[NodeId] {
        &self.path
    }

    pub fn default_prevented(&self) -> bool {
        self.default_prevented
    }

    pub fn propagation_stopped(&self) -> bool {
        self.propagation_stopped
    }

    pub fn prevent_default(&mut self) {
        self.default_prevented = true;
    }

    pub fn stop_propagation(&mut self) {
        self.propagation_stopped = true;
    }

    fn begin_dispatch(&mut self, path: Vec<NodeId>) {
        debug_assert_eq!(path.last().copied(), Some(self.target));
        self.path = path;
        self.current_target = None;
        self.phase = EventPhase::None;
        self.propagation_stopped = false;
    }

    fn set_dispatch_position(&mut self, phase: EventPhase, current_target: NodeId) {
        self.phase = phase;
        self.current_target = Some(current_target);
    }

    fn finish_dispatch(&mut self) {
        self.phase = EventPhase::None;
        self.current_target = None;
    }
}

/// A listener invocation recorded during dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DispatchStep {
    pub node: NodeId,
    pub phase: EventPhase,
}

/// Dispatch `event` along `path`, which must be ordered from root to target.
///
/// The supplied callback is invoked for each capture, target, and bubble step.
/// Calling [`Event::stop_propagation`] from the callback prevents later steps.
/// Calling [`Event::prevent_default`] persists after dispatch and is reflected in
/// the returned `bool` (`true` means the default action may continue).
pub fn dispatch_event<F>(event: &mut Event, path: &[NodeId], mut listener: F) -> bool
where
    F: FnMut(&mut Event),
{
    assert!(!path.is_empty(), "event path must include the target");
    assert_eq!(
        path.last().copied(),
        Some(event.target()),
        "event path must be ordered root-to-target and end with the event target"
    );

    event.begin_dispatch(path.to_vec());

    for &node in &path[..path.len().saturating_sub(1)] {
        event.set_dispatch_position(EventPhase::Capture, node);
        listener(event);
        if event.propagation_stopped() {
            event.finish_dispatch();
            return !event.default_prevented();
        }
    }

    let target = event.target();
    event.set_dispatch_position(EventPhase::Target, target);
    listener(event);
    if event.propagation_stopped() {
        event.finish_dispatch();
        return !event.default_prevented();
    }

    for &node in path[..path.len().saturating_sub(1)].iter().rev() {
        event.set_dispatch_position(EventPhase::Bubble, node);
        listener(event);
        if event.propagation_stopped() {
            break;
        }
    }

    event.finish_dispatch();
    !event.default_prevented()
}

/// Return the deterministic dispatch order for an event path without mutating an event.
pub fn dispatch_order(path: &[NodeId]) -> Vec<DispatchStep> {
    assert!(!path.is_empty(), "event path must include the target");

    let mut order = Vec::with_capacity(path.len() * 2 - 1);
    for &node in &path[..path.len().saturating_sub(1)] {
        order.push(DispatchStep {
            node,
            phase: EventPhase::Capture,
        });
    }
    order.push(DispatchStep {
        node: *path.last().expect("path is non-empty"),
        phase: EventPhase::Target,
    });
    for &node in path[..path.len().saturating_sub(1)].iter().rev() {
        order.push(DispatchStep {
            node,
            phase: EventPhase::Bubble,
        });
    }
    order
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dispatch_order_captures_targets_then_bubbles() {
        let order = dispatch_order(&[1, 2, 3]);

        assert_eq!(
            order,
            vec![
                DispatchStep {
                    node: 1,
                    phase: EventPhase::Capture,
                },
                DispatchStep {
                    node: 2,
                    phase: EventPhase::Capture,
                },
                DispatchStep {
                    node: 3,
                    phase: EventPhase::Target,
                },
                DispatchStep {
                    node: 2,
                    phase: EventPhase::Bubble,
                },
                DispatchStep {
                    node: 1,
                    phase: EventPhase::Bubble,
                },
            ]
        );
    }

    #[test]
    fn dispatch_event_visits_bubbling_order() {
        let mut event = Event::new("click", 30);
        let mut seen = Vec::new();

        let allow_default = dispatch_event(&mut event, &[10, 20, 30], |event| {
            seen.push(DispatchStep {
                node: event.current_target().expect("current target is set"),
                phase: event.phase(),
            });
        });

        assert!(allow_default);
        assert_eq!(seen, dispatch_order(&[10, 20, 30]));
        assert_eq!(event.path(), &[10, 20, 30]);
        assert_eq!(event.phase(), EventPhase::None);
        assert_eq!(event.current_target(), None);
    }

    #[test]
    fn stop_propagation_prevents_later_bubble_steps() {
        let mut event = Event::new("click", 3);
        let mut seen = Vec::new();

        dispatch_event(&mut event, &[1, 2, 3], |event| {
            let step = DispatchStep {
                node: event.current_target().expect("current target is set"),
                phase: event.phase(),
            };
            seen.push(step);
            if step
                == (DispatchStep {
                    node: 2,
                    phase: EventPhase::Bubble,
                })
            {
                event.stop_propagation();
            }
        });

        assert_eq!(
            seen,
            vec![
                DispatchStep {
                    node: 1,
                    phase: EventPhase::Capture,
                },
                DispatchStep {
                    node: 2,
                    phase: EventPhase::Capture,
                },
                DispatchStep {
                    node: 3,
                    phase: EventPhase::Target,
                },
                DispatchStep {
                    node: 2,
                    phase: EventPhase::Bubble,
                },
            ]
        );
    }

    #[test]
    fn prevent_default_makes_dispatch_return_false() {
        let mut event = Event::new("submit", 1);

        let allow_default = dispatch_event(&mut event, &[1], |event| event.prevent_default());

        assert!(!allow_default);
        assert!(event.default_prevented());
    }
}
