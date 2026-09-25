//! The verifier's decision on a proposed goal transition.

/// Outcome of an independent verification.
///
/// # Variants
///
/// - `Pass` — the verifier confirmed the claim; the transition is applied.
/// - `Fail` — the verifier rejected the claim or could not reach a decision;
///   the goal stays active and `findings` go back to the worker.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tool::goal::verify::Verdict;
///
/// match Verdict::parse("FAIL — tests — not run\nVERDICT: FAIL") {
///     Verdict::Pass => unreachable!(),
///     Verdict::Fail { findings } => assert!(findings.contains("not run")),
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
}

impl Verdict {
    /// Read the decision from the last `VERDICT:` line of a report.
    ///
    /// Only an exact `VERDICT: PASS` counts as a pass, with surrounding
    /// markdown emphasis or backticks allowed. A qualified pass, a
    /// `VERDICT: FAIL`, or no verdict line at all is a [`Verdict::Fail`].
    ///
    /// # Arguments
    ///
    /// * `report` — The verifier's full text output.
    ///
    /// # Returns
    ///
    /// [`Verdict::Pass`] or [`Verdict::Fail`] carrying the full report.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::tool::goal::verify::Verdict;
    ///
    /// assert_eq!(Verdict::parse("ok\n**VERDICT: PASS**"), Verdict::Pass);
    /// assert_ne!(Verdict::parse("VERDICT: PASS with gaps"), Verdict::Pass);
    /// assert_ne!(Verdict::parse("looks fine"), Verdict::Pass);
    /// ```
    pub fn parse(report: &str) -> Self {
        let decision = report
            .lines()
            .rev()
            .map(|line| line.trim_matches(|c: char| c.is_whitespace() || c == '*' || c == '`'))
            .find_map(|line| line.strip_prefix("VERDICT:"))
            .map(str::trim);
        match decision {
            Some("PASS") => Self::Pass,
            Some(_) | None => Self::Fail {
                findings: report.to_string(),
            },
        }
    }
}
