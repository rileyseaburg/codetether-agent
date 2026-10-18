//! Ralph item quality presentation.
use crate::{RalphStoryInfo, RalphStoryStatus};
use ratatui::style::{Color, Style};
use ratatui::text::Span;
pub(super) fn append<'a>(story: &'a RalphStoryInfo, spans: &mut Vec<Span<'a>>) {
    // Show quality check status
    if story.status == RalphStoryStatus::QualityCheck {
        let qc_summary: Vec<&str> = story
            .quality_checks
            .iter()
            .map(|(name, _passed)| name.as_str())
            .collect();
        if !qc_summary.is_empty() {
            let checks: String = story
                .quality_checks
                .iter()
                .map(|(name, passed)| {
                    if *passed {
                        format!("✓{}", name)
                    } else {
                        format!("✗{}", name)
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");
            spans.push(Span::styled(
                format!(" [{}]", checks),
                Style::default().fg(Color::Magenta),
            ));
        }
    }
}
