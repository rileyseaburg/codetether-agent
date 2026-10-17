//! Shared forage prompt rules.
use crate::{MoonshotRubric, prompt_fields::normalize_prompt_field};
use codetether_okr::{KeyResult, Okr};
const MAX_PROMPT_FIELD_CHARS: usize = 1_200;

pub(super) fn build_execution_prompt(
    okr: &Okr,
    kr: &KeyResult,
    moonshots: &MoonshotRubric,
    moonshot_alignment: f64,
) -> String {
    let objective = normalize_prompt_field(&okr.title, MAX_PROMPT_FIELD_CHARS);
    let objective_description = normalize_prompt_field(&okr.description, MAX_PROMPT_FIELD_CHARS);
    let key_result = normalize_prompt_field(&kr.title, MAX_PROMPT_FIELD_CHARS);
    let key_result_description = normalize_prompt_field(&kr.description, MAX_PROMPT_FIELD_CHARS);
    let moonshot_section = if moonshots.goals.is_empty() {
        String::new()
    } else {
        format!(
            "\nMoonshot Rubric (strategy filter):\n- {}\nCurrent alignment score: {:.1}%\nDecision rule: prioritize changes that clearly advance one or more moonshots and explain which mission the change moves.",
            moonshots
                .goals
                .iter()
                .map(|g| normalize_prompt_field(g, 160))
                .collect::<Vec<_>>()
                .join("\n- "),
            moonshot_alignment * 100.0
        )
    };

    format!(
        "Business-goal execution task.\n\
Objective: {}\n\
Objective Description: {}\n\
Key Result: {}\n\
KR Description: {}\n\
Current: {:.3} {} | Target: {:.3} {}\n\n\
Execute one concrete, behavior-preserving code change that measurably advances this key result. \
Use tools, validate the change, and report exact evidence tied to the KR.\n\
Focus on local repository changes first; do not do broad web research unless required by the KR.\n\
Return exact changed file paths and at least one verification command result.{}",
        objective,
        objective_description,
        key_result,
        key_result_description,
        kr.current_value,
        kr.unit,
        kr.target_value,
        kr.unit,
        moonshot_section
    )
}
