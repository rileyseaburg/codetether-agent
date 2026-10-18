use reqwest::Client;
/// Constructed backend handles retained by a thinker client.
#[derive(Clone)]
pub(super) enum Backend<N, B> {
    OpenAICompat { http: Client },
    Registry,
    Candle { runtime: N },
    Bedrock { provider: B },
}
