// observability declarations retain the public enum variant shapes.
macro_rules! bus_variants_5 {
    ($($prefix:tt)*) => {
        bus_variants_6! {
            $($prefix)*
            // ── Full tool output ─────────────────────────────────────────────
            /// Full, untruncated tool output from an agent loop step.
            /// Published *before* the result is truncated for the LLM context
            /// window, so other agents and the bus log see the complete output.
            ToolOutputFull {
                agent_id: String,
                tool_name: String,
                output: String,
                success: bool,
                step: usize,
            },


            /// Internal reasoning / chain-of-thought from an agent step.
            /// Published so the training pipeline captures the model's
            /// thinking process alongside its actions.
            AgentThinking {
                agent_id: String,
                thinking: String,
                step: usize,
            },


            /// User prompt that initiated an agent turn (SFT training trace).
            UserPrompt {
                agent_id: String,
                text: String,
                workspace: String,
                session_id: String,
            },

        }
    };
}
