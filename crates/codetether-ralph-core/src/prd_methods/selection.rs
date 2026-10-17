//! PRD selection operations.
use super::*;
impl Prd {
    /// Get the next story to work on (not passed, dependencies met)
    ///
    /// # Returns
    /// The lowest-priority-number ready story, breaking ties by complexity; None if none is ready.
    /// # Examples
    /// ```
    /// # fn example(prd: & codetether_ralph_core::Prd) {
    /// assert!(prd.next_story().map(|story| !story.passes).unwrap_or(true));
    /// # }
    /// ```
    pub fn next_story(&self) -> Option<&UserStory> {
        self.user_stories
            .iter()
            .filter(|s| !s.passes)
            .filter(|s| self.dependencies_met(&s.depends_on))
            .min_by_key(|s| (s.priority, s.complexity))
    }

    /// Get all stories ready to be worked on (not passed, dependencies met)
    ///
    /// # Returns
    /// Incomplete stories whose dependencies are met, in source order.
    /// # Examples
    /// ```
    /// # fn example(prd: & codetether_ralph_core::Prd) {
    /// assert!(prd.ready_stories().iter().all(|story| !story.passes));
    /// # }
    /// ```
    pub fn ready_stories(&self) -> Vec<&UserStory> {
        self.user_stories
            .iter()
            .filter(|s| !s.passes)
            .filter(|s| self.dependencies_met(&s.depends_on))
            .collect()
    }
}
