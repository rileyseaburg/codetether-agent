//! Detection of direct `git worktree add` shell invocations.

pub(super) fn detected(command: &str) -> bool {
    command
        .split([';', '|', '&', '\n', '\r'])
        .any(segment_adds_worktree)
}

fn segment_adds_worktree(segment: &str) -> bool {
    let words: Vec<&str> = segment.split_whitespace().collect();
    let Some(git) = words.iter().position(|word| executable(word) == "git") else {
        return false;
    };
    if words[..git].iter().any(|word| !wrapper_word(word)) {
        return false;
    }
    words[git + 1..]
        .windows(2)
        .any(|pair| clean(pair[0]) == "worktree" && clean(pair[1]) == "add")
}

fn executable(word: &str) -> &str {
    clean(word).rsplit('/').next().unwrap_or("")
}

fn clean(word: &str) -> &str {
    word.trim_matches(['\'', '"', '(', ')'])
}

fn wrapper_word(word: &str) -> bool {
    matches!(clean(word), "command" | "env" | "sudo" | "-n") || word.contains('=')
}
