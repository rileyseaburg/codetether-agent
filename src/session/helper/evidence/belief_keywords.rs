use crate::cognition::beliefs::Belief;

const STOP: &[&str] = &["the", "and", "for", "with", "that", "this", "from"];

pub(crate) fn top_keywords(beliefs: &[Belief], limit: usize) -> Vec<String> {
    let mut words = Vec::new();
    for belief in beliefs.iter().filter(|b| b.confidence >= 0.7) {
        for word in belief.claim.split(|c: char| !c.is_ascii_alphanumeric()) {
            let word = word.to_ascii_lowercase();
            if word.len() > 3 && !STOP.contains(&word.as_str()) && !words.contains(&word) {
                words.push(word);
            }
            if words.len() >= limit {
                return words;
            }
        }
    }
    words
}

#[cfg(test)]
mod tests {
    use super::top_keywords;

    #[test]
    fn empty_beliefs_have_no_keywords() {
        assert!(top_keywords(&[], 4).is_empty());
    }
}
