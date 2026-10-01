//! Split literal shell commands while keeping quoted arguments opaque.

pub(super) fn commands(command: &str) -> Vec<Vec<String>> {
    let mut commands = Vec::new();
    let mut words = Vec::new();
    let mut word = String::new();
    let mut quote = None;
    let mut escaped = false;
    for ch in command.chars() {
        if escaped {
            if ch != '\n' {
                word.push(ch);
            }
            escaped = false;
            continue;
        }
        match ch {
            '\\' if quote != Some('\'') => escaped = true,
            '\'' | '"' if quote == Some(ch) => quote = None,
            '\'' | '"' if quote.is_none() => quote = Some(ch),
            ' ' | '\t' | '\r' if quote.is_none() => flush(&mut words, &mut word),
            ';' | '|' | '&' | '\n' | '(' | ')' | '`' if quote.is_none() => {
                flush(&mut words, &mut word);
                commands.push(std::mem::take(&mut words));
            }
            _ => word.push(ch),
        }
    }
    flush(&mut words, &mut word);
    commands.push(words);
    commands
}

fn flush(words: &mut Vec<String>, word: &mut String) {
    if !word.is_empty() {
        words.push(std::mem::take(word));
    }
}
