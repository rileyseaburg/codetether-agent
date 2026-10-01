//! Extract evaluated command substitutions, excluding single-quoted literals.

pub(super) fn commands(command: &str) -> Vec<String> {
    let mut commands = Vec::new();
    let mut chars = command.chars().peekable();
    let mut quote = None;
    while let Some(ch) = chars.next() {
        if ch == '\\' && quote != Some('\'') {
            chars.next();
            continue;
        }
        if matches!(ch, '\'' | '"') && quote == Some(ch) {
            quote = None;
            continue;
        }
        if matches!(ch, '\'' | '"') && quote.is_none() {
            quote = Some(ch);
            continue;
        }
        if quote == Some('\'') {
            continue;
        }
        if ch == '$' && chars.next_if_eq(&'(').is_some() {
            commands.push(chars.by_ref().take_while(|ch| *ch != ')').collect());
        } else if ch == '`' {
            commands.push(chars.by_ref().take_while(|ch| *ch != '`').collect());
        } else if ch == '#' && quote.is_none() {
            for ch in chars.by_ref() {
                if ch == '\n' {
                    break;
                }
            }
        }
    }
    commands
}
