use serde_json::{Value, json};

#[path = "match_candidate.rs"]
mod match_candidate;
#[cfg(test)]
#[path = "matcher_tests.rs"]
mod matcher_tests;
use match_candidate::{nearest, tolerant_match};

#[derive(Debug)]
pub enum MatchPlan<'a> {
    Exact { count: usize },
    Tolerant { matched: &'a str },
}

pub fn plan<'a>(content: &'a str, old: &str, replace_all: bool) -> Result<MatchPlan<'a>, Value> {
    let count = content.matches(old).count();
    if count == 1 || count > 1 && replace_all {
        return Ok(MatchPlan::Exact { count });
    }
    if count > 1 {
        return Err(json!({"code":"AMBIGUOUS_MATCH","matches_found":count,
            "hint":"Set replace_all=true to replace every occurrence, or add context."}));
    }
    if let Some(matched) = tolerant_match(content, old) {
        return Ok(MatchPlan::Tolerant { matched });
    }
    Err(
        json!({"code":"NOT_FOUND","closest_candidate": nearest(content, old),
        "hint":"No exact or whitespace-tolerant match found; compare old_string to closest_candidate."}),
    )
}

pub fn apply(
    content: &str,
    plan: MatchPlan<'_>,
    old: &str,
    new: &str,
) -> (String, usize, &'static str) {
    match plan {
        MatchPlan::Exact { count } if count > 1 => (content.replace(old, new), count, "exact_all"),
        MatchPlan::Exact { .. } => (content.replacen(old, new, 1), 1, "exact"),
        MatchPlan::Tolerant { matched } => {
            (content.replacen(matched, new, 1), 1, "whitespace_tolerant")
        }
    }
}
