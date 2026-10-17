//! Handle a text response through the legacy DSL and final-answer policy.
use super::*;
impl RlmExecutor {
    pub(super) async fn execute_analysis_dsl(
        &mut self,
        response: &codetether_provider_core::CompletionResponse,
        run: &mut AnalysisRun,
    ) -> Result<()> {
        // ── Legacy DSL path (fallback) ───────────────────────────────
        // If no structured tool calls were produced, fall back to the
        // original regex-parsed code-block execution.
        let (code, execution_result) = self
            .execute_dsl_step(response, &mut run.messages, run.iterations)
            .await?;
        match self.check_dsl_final(
            &code,
            &execution_result,
            run.iterations,
            run.requires_pattern_evidence,
            &mut run.messages,
        ) {
            DslControl::NoFinal => {}
            DslControl::Rejected => return Ok(()),
            DslControl::Accepted(answer) => {
                run.final_answer = Some(answer);
                return Ok(());
            }
        }
        self.record_dsl_output(&execution_result, run.iterations, &mut run.messages);

        Ok(())
    }
}
