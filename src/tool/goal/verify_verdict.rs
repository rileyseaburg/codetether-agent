//! The verifier's decision on a proposed goal transition.

/// Outcome of an independent verification.
///
/// # Variants
///
/// - `Pass` — the verifier confirmed the claim; the transition is applied.
/// - `Fail` — the verifier explicitly rejected the claim; counts toward the cap.
/// - `Unavailable` — runtime/protocol failure, not a rejection of the work.
///
/// Neither non-PASS outcome authorizes a lifecycle transition.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tool::goal::verify::Verdict;
///
/// match Verdict::parse("FAIL — tests — not run\nVERDICT: FAIL") {
///     Verdict::Pass => unreachable!(),
///     Verdict::Fail { findings } => assert!(findings.contains("not run")),
///     Verdict::Unavailable { .. } => unreachable!(),
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Verdict {
    /// The claim holds; apply the transition.
    Pass,
    /// The claim does not hold; keep the goal active.
    Fail {
        /// Verifier report describing the remaining work.
        findings: String,
    },
    /// The verifier did not produce a usable decision; do not count a rejection.
    Unavailable {
        /// Runtime failure or malformed report; not a judgment of the work.
        findings: String,
    },
}

impl Verdict {
    /// Read the decision from the final non-empty line of a report.
    ///
    /// Only an exact `VERDICT: PASS` on the last non-empty line counts as a
    /// pass, with surrounding markdown emphasis or backticks allowed. A
    /// trailing swarm deliverable footer (`STATUS: ...`, required of every
    /// sub-agent) is skipped first. Only an exact `VERDICT: FAIL` is a
    /// [`Verdict::Fail`]. Missing, qualified, or trailing-text verdicts are
    /// [`Verdict::Unavailable`], not substantive rejections.
    ///
    /// # Arguments
    ///
    /// * `report` — The verifier's full text output.
    ///
    /// # Returns
    ///
    /// PASS, explicit FAIL, or an unavailable decision carrying the report.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::tool::goal::verify::Verdict;
    ///
    /// assert_eq!(Verdict::parse("ok\n**VERDICT: PASS**\n"), Verdict::Pass);
    /// assert_eq!(Verdict::parse("VERDICT: PASS\n\nSTATUS: completed"), Verdict::Pass);
    /// assert_ne!(Verdict::parse("VERDICT: PASS with gaps"), Verdict::Pass);
    /// assert_ne!(Verdict::parse("VERDICT: PASS\nbut tests were skipped"), Verdict::Pass);
    /// assert_ne!(Verdict::parse("looks fine"), Verdict::Pass);
    /// ```
    pub fn parse(report: &str) -> Self {
        let decision = report
            .lines()
            .map(|line| line.trim_matches(|c: char| c.is_whitespace() || c == '*' || c == '`'))
            .rfind(|line| !line.is_empty() && !line.starts_with("STATUS:"))
            .and_then(|line| line.strip_prefix("VERDICT:"))
            .map(str::trim);
        match decision {
            Some("PASS") => Self::Pass,
            Some("FAIL") => Self::Fail {
                findings: report.to_string(),
            },
            Some(_) | None => Self::Unavailable {
                findings: report.to_string(),
            },
        }
    }
}

#[cfg(test)]
#[path = "verify_verdict_tests.rs"]
mod tests;
