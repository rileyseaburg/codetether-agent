//! Line box model for inline layout.

use super::baseline::align_baselines;
use super::fragment::InlineFragment;

/// A single line containing inline fragments.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LineBox {
    pub y: i64,
    pub width: i64,
    pub height: i64,
    pub baseline: i64,
    pub fragments: Vec<InlineFragment>,
}

impl LineBox {
    /// Create an empty line at the given y offset with the given container width.
    pub fn new(y: i64, width: i64) -> Self {
        Self {
            y,
            width,
            height: 0,
            baseline: 0,
            fragments: Vec::new(),
        }
    }

    /// Total width used by fragments in this line.
    pub fn used_width(&self) -> i64 {
        self.fragments.iter().map(|f| f.width).sum()
    }

    /// Remaining horizontal space in this line.
    pub fn remaining_width(&self) -> i64 {
        self.width - self.used_width()
    }

    /// Whether this line has no fragments yet.
    pub fn is_empty(&self) -> bool {
        self.fragments.is_empty()
    }

    /// Push a fragment, updating height and tracking position.
    pub fn push(&mut self, mut fragment: InlineFragment) {
        fragment.x = self.used_width();
        fragment.y = self.y;
        self.height = self.height.max(fragment.height);
        self.baseline = self.baseline.max(fragment.baseline_offset);
        self.fragments.push(fragment);
    }

    /// Finalize the line: align all fragments to the shared baseline.
    pub fn finalize(mut self) -> Self {
        align_baselines(&mut self);
        self
    }
}
