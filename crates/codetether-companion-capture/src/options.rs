use std::time::Duration;

/// Local trigger preferences. Remote commands cannot alter these options.
///
/// ```
/// use codetether_companion_capture::Options;
/// assert!(Options::new(15, true, true, false).is_some());
/// assert!(Options::new(14, true, true, true).is_none());
/// ```
#[derive(Clone, Copy)]
pub struct Options {
    pub(crate) interval: Duration,
    pub(crate) periodic: bool,
    pub(crate) right_click: bool,
    pub(crate) double_click: bool,
}
impl Options {
    /// Construct locally selected trigger preferences.
    ///
    /// # Arguments
    /// * `interval_seconds` - Relay-compatible interval, inclusive 15–300.
    /// * `periodic` - Enable periodic capture.
    /// * `right_click` - Enable local right-click capture.
    /// * `double_click` - Enable local double-click capture.
    /// # Returns
    /// `None` for an out-of-range interval; otherwise validated preferences.
    /// # Examples
    /// See the type-level example.
    pub fn new(
        interval_seconds: u64,
        periodic: bool,
        right_click: bool,
        double_click: bool,
    ) -> Option<Self> {
        (15..=300).contains(&interval_seconds).then_some(Self {
            interval: Duration::from_secs(interval_seconds),
            periodic,
            right_click,
            double_click,
        })
    }
}
