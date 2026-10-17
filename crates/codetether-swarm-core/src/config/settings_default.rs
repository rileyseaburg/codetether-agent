use super::*;

impl Default for SwarmConfig {
    fn default() -> Self {
        Self {
            max_subagents: MAX_SUBAGENTS,
            max_steps_per_subagent: 100,
            max_total_steps: MAX_TOOL_CALLS,
            subagent_timeout_secs: 600, // 10 minutes for complex tasks
            parallel_enabled: true,
            critical_path_threshold: 10,
            model: None,
            // Concurrency tuned for modern paid tiers and multi-agent swarms.
            // Override via SwarmConfigBuilder for conservative free-tier
            // accounts (e.g. 3 concurrent, 1000ms delay).
            max_concurrent_requests: 8,
            request_delay_ms: 250,
            worktree_enabled: true,    // Enable worktree isolation by default
            worktree_auto_merge: true, // Auto-merge on success
            collapse_enabled: false,
            working_dir: None,
            execution_mode: ExecutionMode::LocalThread,
            k8s_pod_budget: 8,
            k8s_subagent_image: None,
            max_retries: 3,
            base_delay_ms: 500,
            max_delay_ms: 30_000,
        }
    }
}
