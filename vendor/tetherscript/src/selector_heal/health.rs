//! Selector health analysis.

/// Selector quality report.
#[derive(Clone, Debug)]
pub struct SelectorHealth {
    pub stability_score: f32,
    pub specificity_score: f32,
    pub robustness_score: f32,
    pub recommendation: SelectorRecommendation,
}

/// Recommendation for improving selector quality.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SelectorRecommendation {
    Keep,
    PreferTestId,
    PreferAriaLabel,
    AvoidDynamicId,
    AvoidGeneratedClass,
    AvoidNthChild,
    TooGeneric,
}

impl SelectorHealth {
    /// Analyze selector health using static heuristics.
    pub fn check(selector: &str) -> Self {
        let stability = calc_stability(selector);
        let specificity = calc_specificity(selector);
        let robustness = calc_robustness(selector);
        let recommendation = calc_recommendation(selector, specificity);
        Self {
            stability_score: stability,
            specificity_score: specificity,
            robustness_score: robustness,
            recommendation,
        }
    }

    /// Overall score.
    pub fn score(&self) -> f32 {
        (self.stability_score + self.specificity_score + self.robustness_score) / 3.0
    }
}

fn calc_stability(s: &str) -> f32 {
    if s.contains("data-testid") {
        0.98
    } else if s.contains("aria-label") || s.contains("[role=") {
        0.9
    } else if s.contains(":nth-child") {
        0.35
    } else if s.starts_with('#') && is_dynamic(s) {
        0.4
    } else if s.contains('.') && is_generated(s) {
        0.45
    } else {
        0.7
    }
}

fn calc_specificity(s: &str) -> f32 {
    if s == "*" || ["div", "span", "button", "input"].contains(&s) {
        0.25
    } else if s.contains('[') || s.starts_with('#') {
        0.9
    } else if s.contains('.') {
        0.75
    } else {
        0.5
    }
}

fn calc_robustness(s: &str) -> f32 {
    if s.contains("data-testid") || s.contains("aria-label") {
        0.95
    } else if s.contains(":nth-child") {
        0.25
    } else if is_generated(s) || is_dynamic(s) {
        0.35
    } else {
        0.65
    }
}

fn calc_recommendation(s: &str, spec: f32) -> SelectorRecommendation {
    if s.contains("data-testid") {
        SelectorRecommendation::Keep
    } else if is_dynamic(s) {
        SelectorRecommendation::AvoidDynamicId
    } else if is_generated(s) {
        SelectorRecommendation::AvoidGeneratedClass
    } else if s.contains(":nth-child") {
        SelectorRecommendation::AvoidNthChild
    } else if spec < 0.4 {
        SelectorRecommendation::TooGeneric
    } else if !s.contains("aria-label") {
        SelectorRecommendation::PreferTestId
    } else {
        SelectorRecommendation::Keep
    }
}

fn is_dynamic(s: &str) -> bool {
    s.chars().filter(|c| c.is_ascii_digit()).count() > 4
}

fn is_generated(s: &str) -> bool {
    s.contains('_') || s.contains("css-") || s.contains("sc-")
}
