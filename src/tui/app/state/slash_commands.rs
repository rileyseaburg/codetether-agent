//! Slash command constants used for autocomplete suggestions.

/// All recognized slash commands (including easy-mode aliases).
#[rustfmt::skip]
pub const SLASH_COMMANDS: &[&str] = &["/help", "/status", "/permissions", "/access-mode", "/sandbox-mode", "/sessions", "/resume", "/clear", "/continue", "/review", "/copy", "/diff", "/goal", "/swarm", "/ralph", "/forage", "/bus", "/protocol", "/file", "/image", "/autoapply", "/approve", "/deny", "/network", "/autocomplete", "/ask", "/mcp", "/mux", "/model", "/settings", "/lsp", "/rlm", "/latency", "/inspector", "/symbols", "/chat", "/new", "/keys", "/spawn", "/kill", "/detach", "/agents", "/agent", "/add", "/talk", "/say", "/list", "/ls", "/remove", "/rm", "/focus", "/home", "/main", "/webview", "/classic", "/autochat", "/recall", "/plugin", "/audit", "/git", "/edit"];
