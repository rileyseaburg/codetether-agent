//! Detect interpreter commands rather than Python names in ordinary arguments.

mod executable;
mod substitutions;
mod tokens;
mod wrappers;

pub(super) fn detected(command: &str) -> bool {
    tokens::commands(command).iter().any(|words| segment(words))
        || substitutions::commands(command)
            .iter()
            .any(|script| detected(script))
}

fn segment(words: &[String]) -> bool {
    let mut words = words.iter().map(String::as_str);
    while let Some(word) = words.next() {
        if let Some(script) = word.strip_prefix("--split-string=") {
            return detected(script);
        }
        if matches!(word, "-S" | "--split-string") {
            return words.next().is_some_and(detected);
        }
        if executable::python(word) {
            return true;
        }
        if wrappers::shell(word) {
            return words
                .next()
                .is_some_and(|flags| flags.starts_with('-') && flags.contains('c'))
                && words.next().is_some_and(detected);
        }
        if wrappers::takes_value(word) {
            words.next();
        } else if !wrappers::allowed(word) {
            return false;
        }
    }
    false
}
