// ralph declarations retain the public enum variant shapes.
macro_rules! bus_variants_4 {
    ($($prefix:tt)*) => {
        bus_variants_5! {
            $($prefix)*
            // ── Ralph loop messages ──────────────────────────────────────────
            /// An agent shares learnings from a Ralph iteration (insights,
            /// blockers, patterns discovered) so subsequent iterations or
            /// co-ordinating agents can build on them.
            RalphLearning {
                prd_id: String,
                story_id: String,
                iteration: usize,
                learnings: Vec<String>,
                context: serde_json::Value,
            },
            
            
            /// Context handoff between sequential Ralph stories.  The finishing
            /// story publishes what the *next* story should know.
            RalphHandoff {
                prd_id: String,
                from_story: String,
                to_story: String,
                context: serde_json::Value,
                progress_summary: String,
            },
            
            
            /// PRD-level progress update (aggregated across all stories).
            RalphProgress {
                prd_id: String,
                passed: usize,
                total: usize,
                iteration: usize,
                status: String,
            },

        }
    };
}
