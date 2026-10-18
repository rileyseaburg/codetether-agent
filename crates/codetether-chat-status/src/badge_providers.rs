//! Lazy badge callbacks over a frontend's borrowed state.
use crate::BadgeSource;
use ratatui::text::Span;

/// Connects existing rendering functions to the ordered badge assembler.
/// Callbacks run when requested by the assembler, not during construction.
///
/// # Examples
/// ```
/// let source = codetether_chat_status::BadgeProviders {
///     state: &(),
///     auto_apply: |_| vec![], yolo: |_| None, trust: || None,
///     bus: |_| ratatui::text::Span::raw("offline"), images: |_| None,
///     latency: |_| None, turn: |_| None, agents: |_| vec![],
/// };
/// let spans = codetether_chat_status::badge_spans(&source, "session");
/// assert!(spans.iter().any(|span| span.content == "offline"));
/// ```
pub struct BadgeProviders<'a, S> {
    /// Live frontend state borrowed for the duration of rendering.
    pub state: &'a S,
    /// Current automatic-edit policy display.
    pub auto_apply: fn(&S) -> Vec<Span<'static>>,
    /// Current unrestricted-access indicator.
    pub yolo: fn(&S) -> Option<Span<'static>>,
    /// Global trust-policy indicator.
    pub trust: fn() -> Option<Span<'static>>,
    /// Current bus connection indicator.
    pub bus: fn(&S) -> Span<'static>,
    /// Pending image attachment count display.
    pub images: fn(&S) -> Option<Span<'static>>,
    /// Request latency display sampled at rendering time.
    pub latency: fn(&S) -> Option<Vec<Span<'static>>>,
    /// Current or last turn latency display.
    pub turn: fn(&S) -> Option<Span<'static>>,
    /// Registered agent identity rail.
    pub agents: fn(&S) -> Vec<Span<'static>>,
}

impl<S> BadgeSource for BadgeProviders<'_, S> {
    fn auto_apply_spans(&self) -> Vec<Span<'static>> {
        (self.auto_apply)(self.state)
    }
    fn yolo_badge(&self) -> Option<Span<'static>> {
        (self.yolo)(self.state)
    }
    fn trust_status_badge_span(&self) -> Option<Span<'static>> {
        (self.trust)()
    }
    fn bus_status_badge_span(&self) -> Span<'static> {
        (self.bus)(self.state)
    }
    fn pending_images_badge(&self) -> Option<Span<'static>> {
        (self.images)(self.state)
    }
    fn latency_badge_spans(&self) -> Option<Vec<Span<'static>>> {
        (self.latency)(self.state)
    }
    fn turn_latency_badge(&self) -> Option<Span<'static>> {
        (self.turn)(self.state)
    }
    fn agent_rail_spans(&self) -> Vec<Span<'static>> {
        (self.agents)(self.state)
    }
}
