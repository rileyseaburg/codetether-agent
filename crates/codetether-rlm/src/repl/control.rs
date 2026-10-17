//! Parse final-answer and print commands before ordinary expressions.
use super::*;
impl RlmRepl {
    pub(super) fn execute_control_line(&mut self, line: &str) -> anyhow::Result<Option<DslResult>> {
        // Check for FINAL
        if line.starts_with("FINAL(") || line.starts_with("FINAL!(") {
            let start = line
                .find('(')
                .ok_or_else(|| anyhow::anyhow!("missing paren"))?
                + 1;
            let end = line.rfind(')').unwrap_or(line.len());
            let answer = line[start..end]
                .trim()
                .trim_matches(|c| c == '"' || c == '\'' || c == '`');
            return Ok(Some(DslResult::Final(answer.to_string())));
        }

        // Check for print/console.log
        if line.starts_with("print(")
            || line.starts_with("println!(")
            || line.starts_with("console.log(")
        {
            let start = line
                .find('(')
                .ok_or_else(|| anyhow::anyhow!("missing paren"))?
                + 1;
            let end = line.rfind(')').unwrap_or(line.len());
            let content = line[start..end]
                .trim()
                .trim_matches(|c| c == '"' || c == '\'' || c == '`');

            // Expand variables
            let expanded = self.expand_expression(content);
            return Ok(Some(DslResult::Output(expanded)));
        }

        Ok(None)
    }
}
