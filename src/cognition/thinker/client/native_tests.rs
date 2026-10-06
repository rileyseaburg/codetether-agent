//! Non-native backends must not accept a misleading native provider identity.

use super::{ThinkerClient, ThinkerClientBackend};
use crate::cognition::thinker::ThinkerConfig;

#[tokio::test]
async fn thinker_identity_override_rejects_non_native_backends_before_dispatch() {
    let backends = [
        ThinkerClientBackend::Registry,
        ThinkerClientBackend::OpenAICompat {
            http: reqwest::Client::new(),
        },
    ];
    for backend in backends {
        let client = ThinkerClient {
            config: ThinkerConfig::default(),
            backend,
        };
        let error = client
            .think_as_provider("local_cuda", "caller instructions", "user prompt")
            .await
            .expect_err("a non-native backend must reject the identity override");
        assert_eq!(
            error.to_string(),
            "provider identity override requires a native backend"
        );
    }
}
