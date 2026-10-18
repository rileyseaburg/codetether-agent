/// Paste-relevant active client view.
/// # Examples
/// ```
/// match codetether_paste_buffer::routing::View::Chat {
///     codetether_paste_buffer::routing::View::Chat => {},
///     _ => unreachable!(),
/// }
/// ```
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum View {
    /// Protocol bus, with a separately controlled filter input mode.
    Bus,
    /// Model selection filter.
    Model,
    /// Session selection filter.
    Sessions,
    /// Chat composer.
    Chat,
    /// A view that does not consume paste events.
    Other,
}
