//! Special model entries merged into Z.AI discovered models.
//!
//! These models exist outside the `/models` API endpoint
//! (coding-plan exclusives, etc.) and must be added manually
//! when discovery succeeds.

use super::ModelInfo;

/// Push special models into `models` if not already present.
pub fn merge_special(models: &mut Vec<ModelInfo>) {
    let specials = [glm52(), glm47_flash(), pony_alpha_2()];
    for info in specials {
        if !models.iter().any(|m| m.id == info.id) {
            models.push(info);
        }
    }
}

fn info(id: &str, name: &str, ctx: usize, max_out: usize) -> ModelInfo {
    ModelInfo {
        id: id.into(), name: name.into(), provider: "zai".into(),
        context_window: ctx, max_output_tokens: Some(max_out),
        supports_vision: false, supports_tools: true, supports_streaming: true,
        input_cost_per_million: None, output_cost_per_million: None,
    }
}

fn glm52() -> ModelInfo { info("glm-5.2", "GLM-5.2", 1_000_000, 128_000) }

fn glm47_flash() -> ModelInfo { info("glm-4.7-flash", "GLM-4.7 Flash", 128_000, 128_000) }

fn pony_alpha_2() -> ModelInfo {
    info(super::zai::PONY_ALPHA_2_MODEL, "Pony Alpha 2", 128_000, 16_384)
}