//! Parent activity notification values.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Parent activity that can wake a collaboration wait.
///
/// # Examples
/// ```
/// let activity=codetether_collaboration_tools::Activity::Mailbox;
/// assert_eq!(activity,codetether_collaboration_tools::Activity::Mailbox);
/// ```
pub enum Activity {
    /// A child mailbox has an update.
    Mailbox,
    /// New parent input identified by its delivery ID.
    Steered(u64),
}
