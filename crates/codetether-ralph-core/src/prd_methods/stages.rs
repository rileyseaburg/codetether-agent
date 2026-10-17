//! PRD stages operations.
use super::*;
impl Prd {
    /// Group stories into parallel execution stages based on dependencies
    /// Returns a Vec of stages, where each stage is a Vec of stories that can run in parallel
    ///
    /// # Returns
    /// Ordered parallel stages; unresolved or cyclic dependencies form the final stage.
    /// # Examples
    /// ```
    /// # fn example(prd: & codetether_ralph_core::Prd) {
    /// let stages = prd.stages();
    /// assert!(stages.iter().flatten().all(|story| !story.passes));
    /// # }
    /// ```
    pub fn stages(&self) -> Vec<Vec<&UserStory>> {
        let mut stages: Vec<Vec<&UserStory>> = Vec::new();
        let mut completed: std::collections::HashSet<String> = self
            .user_stories
            .iter()
            .filter(|s| s.passes)
            .map(|s| s.id.clone())
            .collect();

        let mut remaining: Vec<&UserStory> =
            self.user_stories.iter().filter(|s| !s.passes).collect();

        while !remaining.is_empty() {
            // Find all stories whose dependencies are met
            let (ready, not_ready): (Vec<_>, Vec<_>) = remaining
                .into_iter()
                .partition(|s| s.depends_on.iter().all(|dep| completed.contains(dep)));

            if ready.is_empty() {
                // Circular dependency or missing deps - just take remaining
                if !not_ready.is_empty() {
                    stages.push(not_ready);
                }
                break;
            }

            // Mark these as "will be completed" for next iteration
            for story in &ready {
                completed.insert(story.id.clone());
            }

            stages.push(ready);
            remaining = not_ready;
        }

        stages
    }
}
