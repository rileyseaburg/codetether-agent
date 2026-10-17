// tools declarations retain the public enum variant shapes.
macro_rules! bus_variants_2 {
    ($($prefix:tt)*) => {
        bus_variants_3! {
            $($prefix)*
            /// Tool execution request (for shared tool dispatch)
            ToolRequest {
                request_id: String,
                agent_id: String,
                tool_name: String,
                arguments: serde_json::Value,
                step: usize,
            },
            
            /// Tool execution response
            ToolResponse {
                request_id: String,
                agent_id: String,
                tool_name: String,
                result: String,
                success: bool,
                step: usize,
            },

        }
    };
}
