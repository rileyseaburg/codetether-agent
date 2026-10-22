//! Version the producer cursor together with the derived summary view.
/// Return the storage name for this summary policy's cursor and projection.
/// A policy change must replay source messages rather than reuse the previous
/// worker's acknowledged position. Older views remain available as evidence.
/// # Arguments
/// None; the name follows [`super::SUMMARY_POLICY_VERSION`].
/// # Returns
/// A stable name shared by the summary producer and projection reader.
/// # Examples
/// ```
/// use codetether_context_policy::summary_index::{projection_name, SUMMARY_POLICY_VERSION};
/// assert_eq!(projection_name(), format!("rlm:{SUMMARY_POLICY_VERSION}"));
/// ```
pub fn projection_name() -> String {
    format!("rlm:{}", super::SUMMARY_POLICY_VERSION)
}
