//! Standard relay role profiles in their original execution order.
//!
//! build_implementation_relay_profiles returns agent names, system prompts and
//! capability tags, limiting the selected roles to the requested count.
mod coder;
mod planner;
mod refactorer;
mod reviewer;
mod tester;
/// Build the default implementation team profiles.
///
/// # Arguments
/// * `max_agents` - Maximum number of profiles to return.
/// * `working_dir` - Workspace path embedded in each role prompt.
/// # Returns
/// Ordered planner, coder, reviewer, tester and refactorer profiles as requested.
/// # Examples
/// ```
/// let profiles = codetether_ralph_core::relay_profiles::build_implementation_relay_profiles(2, std::path::Path::new("."));
/// assert_eq!(profiles.len(), 2);
/// assert_eq!(profiles[0].0, "auto-planner");
/// ```

/// Build default relay profiles for code implementation tasks.
pub fn build_implementation_relay_profiles(
    max_agents: usize,
    working_dir: &std::path::Path,
) -> Vec<(String, String, Vec<String>)> {
    let mut profiles = vec![
        planner::profile(working_dir),
        coder::profile(working_dir),
        reviewer::profile(working_dir),
    ];

    if max_agents >= 4 {
        profiles.push(tester::profile(working_dir));
    }

    if max_agents >= 5 {
        profiles.push(refactorer::profile(working_dir));
    }

    profiles.truncate(max_agents);
    profiles
}
