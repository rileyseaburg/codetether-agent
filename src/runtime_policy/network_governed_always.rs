//! Tools whose every invocation can cross a network boundary.

pub(super) fn check(tool: &str) -> bool {
    matches!(
        tool,
        "clone_repo"
            | "forage"
            | "webfetch"
            | "websearch"
            | "search"
            | "browserctl"
            | "image_gen"
            | "imagegen"
            | "context_summarize"
            | "rlm"
            | "voice"
            | "voice_input"
            | "voice_stream"
            | "podcast"
            | "avatar"
            | "youtube"
            | "k8s"
            | "kubernetes"
            | "k8s_tool"
            | "tetherscript_plugin"
            | "ralph"
            | "swarm_execute"
            | "agent"
    )
}
