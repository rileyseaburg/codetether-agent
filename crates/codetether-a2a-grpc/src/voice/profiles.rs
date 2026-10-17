//! Voice transport profiles.
use super::*;
pub(super) fn profiles() -> Vec<proto::VoiceProfile> {
    vec![
        proto::VoiceProfile {
            voice_id: "960f89fc".into(),
            name: "Riley".into(),
            language: "english".into(),
            sample_duration_seconds: 10.0,
            created_at: String::new(),
        },
        proto::VoiceProfile {
            voice_id: "puck".into(),
            name: "Puck".into(),
            language: "english".into(),
            sample_duration_seconds: 0.0,
            created_at: String::new(),
        },
        proto::VoiceProfile {
            voice_id: "charon".into(),
            name: "Charon".into(),
            language: "english".into(),
            sample_duration_seconds: 0.0,
            created_at: String::new(),
        },
        proto::VoiceProfile {
            voice_id: "kore".into(),
            name: "Kore".into(),
            language: "english".into(),
            sample_duration_seconds: 0.0,
            created_at: String::new(),
        },
        proto::VoiceProfile {
            voice_id: "fenrir".into(),
            name: "Fenrir".into(),
            language: "english".into(),
            sample_duration_seconds: 0.0,
            created_at: String::new(),
        },
        proto::VoiceProfile {
            voice_id: "aoede".into(),
            name: "Aoede".into(),
            language: "english".into(),
            sample_duration_seconds: 0.0,
            created_at: String::new(),
        },
    ]
}
