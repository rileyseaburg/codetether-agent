/// Stable identifier for a frame in a [`FrameTree`](super::FrameTree).
///
/// # Examples
///
/// ```rust
/// use tetherscript::browser_agent::FrameId;
///
/// let id = FrameId::new(7);
/// assert_eq!(id.get(), 7);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FrameId(u64);

impl FrameId {
    /// Create a frame identifier from a raw numeric value.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Return the raw numeric value.
    pub const fn get(self) -> u64 {
        self.0
    }
}
