// voice declarations retain the public enum variant shapes.
macro_rules! bus_variants_6 {
    ($($prefix:tt)*) => {
        finish_bus_message! {
            $($prefix)*
            // ── Voice session messages ───────────────────────────────────────
            /// A voice session (LiveKit room) has been created.
            VoiceSessionStarted {
                room_name: String,
                agent_id: String,
                voice_id: String,
            },
            
            
            /// A transcript fragment from a voice session.
            VoiceTranscript {
                room_name: String,
                text: String,
                /// "user" or "agent"
                role: String,
                is_final: bool,
            },
            
            
            /// The voice agent's state changed (e.g. listening, thinking, speaking).
            VoiceAgentStateChanged { room_name: String, state: String },
            
            
            /// A voice session has ended.
            VoiceSessionEnded { room_name: String, reason: String },

        }
    };
}
