//! Pricing conversion helpers.

pub(crate) fn price(model: &crate::provider::ModelInfo) -> super::types::Pricing {
    super::types::Pricing {
        prompt: per_token(model.input_cost_per_million),
        completion: per_token(model.output_cost_per_million),
    }
}

fn per_token(cost_per_million: Option<f64>) -> String {
    match cost_per_million {
        Some(cost) => trim_float(cost / 1_000_000.0),
        None => "0".to_string(),
    }
}

fn trim_float(value: f64) -> String {
    let fixed = format!("{value:.12}");
    fixed
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}
