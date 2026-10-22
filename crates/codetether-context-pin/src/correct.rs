//! Atomically retain an exact partial correction and its later user source.
use crate::{PinSource, logic::local_index};
use anyhow::{Result, ensure};
use codetether_context_policy::{
    pages::PageKind,
    state_header::{ConstraintCorrection, ConstraintRecord},
};
use serde_json::Value;

pub(super) async fn apply<S: PinSource>(
    session: &mut S,
    old: usize,
    args: &Value,
) -> Result<String> {
    let source = session
        .constraint_source(old)
        .ok_or_else(|| anyhow::anyhow!("original source must be pinned and available"))?;
    let replacement = args["replacement_index"]
        .as_u64()
        .and_then(|index| usize::try_from(index).ok())
        .ok_or_else(|| anyhow::anyhow!("correct requires replacement_index"))?;
    let next = local_index(session, replacement)?;
    let page = session
        .page_kind(next)
        .ok_or_else(|| anyhow::anyhow!("replacement page missing"))?;
    ensure!(
        !matches!(page, PageKind::SupersededConstraint { .. }),
        "replacement source was superseded"
    );
    let message = session
        .source_message(next)
        .ok_or_else(|| anyhow::anyhow!("replacement source missing"))?;
    let target = ConstraintRecord::from_message(replacement, &message, PageKind::Constraint)
        .ok_or_else(|| anyhow::anyhow!("replacement source has no text"))?;
    let quote = args["source_text"]
        .as_str()
        .filter(|text| !text.trim().is_empty())
        .ok_or_else(|| anyhow::anyhow!("correct requires nonempty source_text"))?;
    ensure!(
        args["source_hash"].as_str() == Some(source.text_hash.as_str()),
        "source_hash is stale or missing; reread the pinned source"
    );
    let start = source
        .text
        .find(quote)
        .ok_or_else(|| anyhow::anyhow!("source_text is not an exact source span"))?;
    let after_first = start + source.text[start..].chars().next().unwrap().len_utf8();
    ensure!(
        !source.text[after_first..].contains(quote),
        "source_text is ambiguous; include unique surrounding text"
    );
    let edit = ConstraintCorrection {
        source_turn: old,
        source_hash: source.text_hash.clone(),
        start,
        end: start + quote.len(),
        replacement_turn: replacement,
        replacement_hash: target.text_hash.clone(),
    };
    edit.validate(&source, &target)?;
    if session.corrections().contains(&edit) {
        ensure!(
            page == PageKind::Constraint,
            "replacement pin changed; a stale retry cannot reactivate it"
        );
        return Ok("Exact correction already recorded.".into());
    }
    ensure!(
        !session
            .corrections()
            .iter()
            .any(|prior| prior.source_turn == old
                && prior.start < edit.end
                && edit.start < prior.end),
        "span already replaced; correct the current replacement source instead"
    );
    let previous = session.corrections().to_vec();
    let mut updated = previous.clone();
    updated.push(edit);
    session.set_corrections(updated);
    session.set_page_kind(next, PageKind::Constraint);
    if let Err(error) = session.persist().await {
        session.set_corrections(previous);
        session.set_page_kind(next, page);
        return Err(error);
    }
    Ok(format!(
        "Exact span in source turn {old} replaced by user source turn {replacement}; all other source text remains active."
    ))
}
