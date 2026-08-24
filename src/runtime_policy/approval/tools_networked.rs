//! Network and provider tools that claim at their own dispatch boundary.

pub(super) fn self_verifying(tool_name: &str) -> bool {
    matches!(
        tool_name,
        "webfetch"
            | "websearch"
            | "search"
            | "image"
            | "lsp"
            | "image_gen"
            | "imagegen"
            | "voice"
            | "voice_input"
            | "voice_stream"
            | "podcast"
            | "avatar"
            | "youtube"
            | "k8s"
            | "kubernetes"
            | "k8s_tool"
            | "context_summarize"
            | "rlm"
            | "session_recall"
    )
}
