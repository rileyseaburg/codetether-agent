use crate::session::Session;

pub fn fork_if_truncated(session: &mut Session, dropped: usize) -> Option<String> {
    if dropped == 0 {
        return None;
    }
    let original = session.id.clone();
    let title = session
        .title
        .clone()
        .unwrap_or_else(|| "large session".to_string());
    session.id = uuid::Uuid::new_v4().to_string();
    session.title = Some(format!("{title} (continued)"));
    Some(original)
}
