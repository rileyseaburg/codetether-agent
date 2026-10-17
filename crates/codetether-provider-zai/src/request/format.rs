//! Buffered request field conventions retained for each caller.

/// Fields emitted by the existing buffered completion clients.
///
/// # Examples
/// ```
/// use codetether_provider_zai::request::BufferedFormat;
/// let format = BufferedFormat::Application;
/// assert_eq!(match format {
///     BufferedFormat::Application => 0,
///     BufferedFormat::ExplicitControls => 1,
/// }, 0);
/// ```
#[derive(Clone, Copy)]
pub enum BufferedFormat {
    /// Omit explicit stream, top-p, and stop fields, as the application does.
    Application,
    /// Include stream=false and the supplied top-p and stop fields.
    ExplicitControls,
}
