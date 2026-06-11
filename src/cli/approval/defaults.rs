pub(crate) fn actor(actor: Option<String>) -> String {
    actor.unwrap_or_else(|| {
        std::env::var("USER")
            .or_else(|_| std::env::var("USERNAME"))
            .unwrap_or_else(|_| "operator".to_string())
    })
}

pub(crate) fn reason(reason: Option<String>, action: &str) -> String {
    reason.unwrap_or_else(|| format!("{action} from cli"))
}
