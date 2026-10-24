use anyhow::{Result, bail};
use regex::Regex;
use std::sync::LazyLock;

static ANSWER_ERROR: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        "(?i)sorry,? something went wrong|encounter(?:ed|ing) an error|",
        "having a hard time fulfilling|unable to (?:fulfill|process)|",
        "could(?:n't| not) (?:fulfill|process)"
    ))
    .expect("constant answer error pattern")
});
static UNAVAILABLE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        "(?i)image creation (?:isn't|may not be) available|",
        "can't create (?:it right now|any for you at the moment)"
    ))
    .expect("constant image availability pattern")
});

pub(super) fn check(answer: &str, image_count: usize) -> Result<()> {
    if ANSWER_ERROR.is_match(answer) {
        bail!("generation_answer_error");
    }
    if image_count == 0 {
        if UNAVAILABLE.is_match(answer) {
            bail!("generation_image_creation_unavailable");
        }
        bail!("generation_no_images");
    }
    if image_count > 8 {
        bail!("generation_image_limit");
    }
    Ok(())
}
