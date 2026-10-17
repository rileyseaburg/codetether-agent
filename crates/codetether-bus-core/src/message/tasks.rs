// tasks declarations retain the public enum variant shapes.
macro_rules! bus_variants_1 {
    ($($prefix:tt)*) => {
        bus_variants_2! {
            $($prefix)*
            /// Free-form message from one agent to another (mirrors A2A `Message`)
            AgentMessage {
                from: String,
                to: String,
                parts: Vec<Part>,
            },

            /// Task status changed
            TaskUpdate {
                task_id: String,
                state: TaskState,
                message: Option<String>,
            },

            /// A new artifact was produced for a task
            ArtifactUpdate { task_id: String, artifact: Artifact },

            /// A sub-agent published a shared result (replaces raw `ResultStore` publish)
            SharedResult {
                key: String,
                value: serde_json::Value,
                tags: Vec<String>,
            },

        }
    };
}
