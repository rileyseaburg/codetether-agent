// speech declarations retain the public enum variant shapes.
macro_rules! bus_variants_3 {
    ($($prefix:tt)*) => {
        bus_variants_4! {
            $($prefix)*
            /// Heartbeat (keep-alive / health signal)
            Heartbeat { agent_id: String, status: String },
            
            
            /// A directed speech act between agents — the unit of the agent
            /// communication language (see [`speech`](crate::speech)).
            ///
            /// Unlike [`AgentMessage`](Self::AgentMessage) (free-form A2A parts),
            /// this carries a typed performative so the recipient knows the intent
            /// (request / inform / propose / claim / blocked / done …) and can react.
            AgentSpeech {
                /// Performative verb (see [`SpeechAct`](crate::speech::SpeechAct)).
                act: String,
                /// Sending agent id.
                from: String,
                /// Recipient: a specific agent id, or `"all"` for broadcast.
                to: String,
                /// Conversation thread id linking a back-and-forth exchange.
                conversation_id: String,
                /// Free-text content the LLM authored / interprets.
                content: String,
            },

        }
    };
}
