//! End state of one terminal attachment.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Why an interactive PTY attachment ended.
/// # Examples
/// ```
/// assert_ne!(codetether_mux_attachment::Outcome::Detached, codetether_mux_attachment::Outcome::Exited);
/// ```
pub enum Outcome {
    /// Input ended or the user requested detachment.
    Detached,
    /// The remote program exited and all available output was delivered.
    Exited,
}
