//! Usage hint strings for slash commands.
//!
//! Returned by [`AppState::current_slash_hint`](super::AppState::current_slash_hint)
//! once the user has typed past the command name and the regular
//! prefix-match suggestion list has been exhausted. The chat-view
//! suggestions panel renders the hint inline so users learn the
//! `<args>` shape without having to consult the help screen.

/// Returns a single-line usage hint for `cmd`, or `None` if the command
/// has no documented arguments worth surfacing inline.
///
/// `cmd` is matched verbatim — pass the first whitespace-separated token
/// of the input (including the leading `/`).
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::app::state::slash_hints::usage_hint;
///
/// assert!(usage_hint("/spawn").is_some());
/// assert!(usage_hint("/totally-not-a-command").is_none());
/// ```
pub fn usage_hint(cmd: &str) -> Option<&'static str> {
    match cmd {
        "/status" => Some("/status  - show session, policy, model, and context state"),
        "/permissions" => Some("/permissions [ask|approve|full]  - show or set tool policy"),
        "/access-mode" => Some("/access-mode <ask|approve|full>  - switch live tool policy"),
        "/sandbox-mode" => Some("/sandbox-mode <read-only|workspace-write|danger-full-access>"),
        "/diff" => Some("/diff  - show git status and diff stats"),
        "/copy" => Some("/copy  - copy the latest assistant reply"),
        "/review" => Some("/review  - prepare a code-review prompt for current changes"),
        "/clear" => Some("/clear  - start a fresh chat session"),
        "/resume" => Some("/resume  - open the session picker"),
        "/continue" => Some("/continue  - resume a stalled or idle turn"),
        "/goal" => Some("/goal [set|edit|pause|resume|done|clear|show]  - manage persistent work"),
        "/spawn" => Some("/spawn <name> [instructions]  — create a sub-agent"),
        "/kill" => Some("/kill <name>  — terminate a sub-agent"),
        "/detach" => Some("/detach  — continue this thread in a new terminal subprocess"),
        "/agent" => Some("/agent <name> [msg]  — focus or message a sub-agent"),
        "/talk" => Some("/talk <name> <msg>  — send to a sub-agent"),
        "/add" => Some("/add <name>  — alias for /spawn"),
        "/ask" => Some("/ask <question>  — ephemeral side question"),
        "/approve" => Some("/approve <id> [reason]  — approve a blocked tool request"),
        "/deny" => Some("/deny <id> [reason]  — deny a blocked tool request"),
        "/file" => Some("/file [path]  — attach a file (no path opens the browser)"),
        "/image" => Some("/image <path>  — attach an image"),
        "/autochat" => Some("/autochat [count] <task>  — multi-agent relay"),
        "/ralph" => Some("/ralph <subcommand>  — run/submit/status"),
        "/forage" => Some("/forage [execute] [N]  — scan OKRs, optionally run top N"),
        "/model" => Some("/model <name>  — switch LLM model"),
        "/focus" => Some("/focus <agent>  — switch to agent's chat"),
        "/ls" | "/list" => Some("/ls  — list spawned agents"),
        "/rm" | "/remove" => Some("/rm [agent]  — undo turn, or remove agent"),
        _ => None,
    }
}
