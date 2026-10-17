//! Clone dependency stages before application scheduling begins.
use crate::{Prd, UserStory};
/// Snapshot the PRD dependency stages as owned stories.
///
/// # Arguments
/// `prd` supplies dependency ordering and story data.
/// # Returns
/// Cloned stages in the original PRD scheduling order.
/// # Examples
/// ```
/// let prd = codetether_ralph_core::create_prd_template("demo", "feature");
/// let stages = codetether_ralph_core::scheduling::owned_stages(&prd);
/// assert_eq!(stages.len(), prd.stages().len());
/// ```
pub fn owned_stages(prd: &Prd) -> Vec<Vec<UserStory>> {
    prd.stages()
        .into_iter()
        .map(|stage| stage.into_iter().cloned().collect())
        .collect()
}
