use crate::Bounds;

/// Monitor snapshot for explicit local selection, never a remote monitor ID.
/// There is deliberately no public constructor: choices come from [`crate::monitors`].
///
/// ```no_run
/// # fn main() -> Result<(), codetether_companion_desktop::Error> {
/// for monitor in codetether_companion_desktop::monitors()? {
///     assert!(monitor.bounds().width() > 0);
/// }
/// # Ok(()) }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Monitor {
    pub(crate) device: String,
    pub(crate) bounds: Bounds,
    pub(crate) primary: bool,
}
impl Monitor {
    /// OS display name, for local UI only; not a durable physical monitor ID.
    pub fn device(&self) -> &str {
        &self.device
    }
    /// Physical-pixel bounds for selected-monitor capture and click filtering.
    pub fn bounds(&self) -> Bounds {
        self.bounds
    }
    /// Whether Windows identified this monitor as the primary display.
    pub fn is_primary(&self) -> bool {
        self.primary
    }
}
