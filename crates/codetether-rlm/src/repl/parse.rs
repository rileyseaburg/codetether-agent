//! Assignment and expression-command dispatch.
use super::*;
impl RlmRepl {
    pub(super) fn try_execute_dsl_line(&mut self, line: &str) -> anyhow::Result<Option<DslResult>> {
        if let Some(result) = self.execute_control_line(line)? {
            return Ok(Some(result));
        }
        // Check for variable assignment
        if let Some(eq_pos) = line.find('=')
            && !line.contains("==")
            && !line.starts_with("if ")
        {
            let var_name = line[..eq_pos]
                .trim()
                .trim_start_matches("let ")
                .trim_start_matches("const ")
                .trim_start_matches("var ")
                .trim();
            let expr = line[eq_pos + 1..].trim().trim_end_matches(';');

            let value = self.evaluate_expression(expr);
            self.set_var(var_name, value);
            return Ok(None);
        }

        // Check for function calls that should output
        if line.starts_with("head(")
            || line.starts_with("tail(")
            || line.starts_with("grep(")
            || line.starts_with("count(")
            || line.starts_with("lines()")
            || line.starts_with("slice(")
            || line.starts_with("chunks(")
            || line.starts_with("ast_query(")
            || line.starts_with("context")
        {
            let result = self.evaluate_expression(line);
            return Ok(Some(DslResult::Output(result)));
        }

        Ok(None)
    }
}
