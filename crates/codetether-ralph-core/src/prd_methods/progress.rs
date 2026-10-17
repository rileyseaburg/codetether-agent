//! PRD progress operations.
use super::*;
impl Prd {
    /// Get count of passed stories
    ///
    /// # Returns
    /// The number of stories whose passes flag is true.
    /// # Examples
    /// ```
    /// # fn example(prd: & codetether_ralph_core::Prd) {
    /// assert!(prd.passed_count() <= prd.user_stories.len());
    /// # }
    /// ```
    pub fn passed_count(&self) -> usize {
        self.user_stories.iter().filter(|s| s.passes).count()
    }

    /// Check if all stories are complete
    ///
    /// # Returns
    /// True when all stories pass, including an empty PRD.
    /// # Examples
    /// ```
    /// # fn example(prd: & codetether_ralph_core::Prd) {
    /// assert_eq!(prd.is_complete(), prd.user_stories.iter().all(|story| story.passes));
    /// # }
    /// ```
    pub fn is_complete(&self) -> bool {
        self.user_stories.iter().all(|s| s.passes)
    }

    /// Mark a story as passed
    ///
    /// # Arguments
    /// * `story_id` - Identifier of the first story to mark passed.
    /// # Returns
    /// Unit; updates the first matching story if found.
    /// # Examples
    /// ```
    /// # fn example(prd: &mut codetether_ralph_core::Prd) {
    /// prd.mark_passed("US-1");
    /// assert!(prd.user_stories.iter().filter(|story| story.id == "US-1").all(|story| story.passes));
    /// # }
    /// ```
    pub fn mark_passed(&mut self, story_id: &str) {
        if let Some(story) = self.user_stories.iter_mut().find(|s| s.id == *story_id) {
            story.passes = true;
        }
    }
}
