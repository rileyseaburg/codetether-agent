use super::config_issues::{config_error, config_warning};
use super::{SwarmValidator, ValidationIssue};

impl SwarmValidator {
    pub(super) fn validate_configuration(&self, issues: &mut Vec<ValidationIssue>) {
        self.validate_max_subagents(issues);
        self.validate_timeout(issues);
        self.validate_max_steps(issues);
    }

    fn validate_max_subagents(&self, issues: &mut Vec<ValidationIssue>) {
        match self.config.max_subagents {
            0 => issues.push(config_error(
                "max_subagents must be greater than 0",
                "Set max_subagents to at least 1",
            )),
            count if count > 100 => issues.push(config_warning(
                format!("max_subagents ({count}) is very high and may cause rate limiting issues"),
                "Consider reducing max_subagents to 50 or less",
            )),
            _ => {}
        }
    }

    fn validate_timeout(&self, issues: &mut Vec<ValidationIssue>) {
        if self.config.subagent_timeout_secs == 0 {
            issues.push(config_error(
                "subagent_timeout_secs must be greater than 0",
                "Set subagent_timeout_secs to at least 60",
            ));
        }
    }

    fn validate_max_steps(&self, issues: &mut Vec<ValidationIssue>) {
        if self.config.max_steps_per_subagent == 0 {
            issues.push(config_error(
                "max_steps_per_subagent must be greater than 0",
                "Set max_steps_per_subagent to at least 10",
            ));
        }
    }
}
