// lifecycle declarations retain the public enum variant shapes.
macro_rules! bus_variants_0 {
    ($($prefix:tt)*) => {
        bus_variants_1! {
            $($prefix)*
            /// An agent has come online and is ready to receive work
            AgentReady {
                agent_id: String,
                capabilities: Vec<String>,
            },
            
            /// The concrete tool schemas an agent had available for its turns.
            AgentToolCatalog {
                agent_id: String,
                tools: Vec<tool_catalog::ToolSchema>,
            },
            
            /// An agent is shutting down
            AgentShutdown { agent_id: String },

        }
    };
}
