//! Core inline layout algorithm.

use super::baseline::align_line;
use super::break_rules::break_text_to_width;
use super::inline_box::{InlineBox, InlineBoxKind};
use super::line_box::{LineBox, PositionedInlineBox};
use super::measure::{measure_inline_box, DEFAULT_FONT_SIZE, LINE_HEIGHT_FACTOR};

/// Lays inline-level boxes into line boxes constrained by a container width.
pub struct InlineLayoutEngine {
    pub container_width: f32,
}

impl InlineLayoutEngine {
    pub fn new(container_width: f32) -> Self {
        Self { container_width }
    }

    pub fn layout(&self, children: &[InlineBox]) -> Vec<LineBox> {
        let (mut lines, mut line, mut y) = (Vec::new(), LineBox::new(0.0), 0.0);
        for child in children.iter().cloned() {
            self.layout_child(child, &mut line, &mut lines, &mut y);
        }
        self.finish_line(&mut line, &mut lines, &mut y, false);
        lines
    }

    fn layout_child(
        &self,
        child: InlineBox,
        line: &mut LineBox,
        lines: &mut Vec<LineBox>,
        y: &mut f32,
    ) {
        match child.kind.clone() {
            InlineBoxKind::LineBreak => self.finish_line(line, lines, y, true),
            InlineBoxKind::Text(run) => self.layout_text(child, run.text, line, lines, y),
            _ => self.layout_atomic(child, line, lines, y),
        }
    }

    fn layout_text(
        &self,
        template: InlineBox,
        mut text: String,
        line: &mut LineBox,
        lines: &mut Vec<LineBox>,
        y: &mut f32,
    ) {
        while !text.is_empty() {
            let avail = (self.container_width - line.width).max(0.0);
            let token = break_text_to_width(&text, avail);
            if token.head.is_empty() && !line.is_empty() {
                self.finish_line(line, lines, y, false);
                continue;
            }
            let mut piece = template.clone();
            if let InlineBoxKind::Text(run) = &mut piece.kind {
                run.text = token.head.clone();
            }
            if !token.head.is_empty() {
                self.push_box(line, piece);
            }
            text = token.tail;
            if token.forced || !text.is_empty() {
                let forced = token.forced;
                self.finish_line(line, lines, y, forced);
                if !forced {
                    text = text.trim_start_matches(char::is_whitespace).to_string();
                }
            }
        }
    }

    fn layout_atomic(
        &self,
        child: InlineBox,
        line: &mut LineBox,
        lines: &mut Vec<LineBox>,
        y: &mut f32,
    ) {
        let (width, _) = measure_inline_box(&child);
        if !line.is_empty() && line.width + width > self.container_width {
            self.finish_line(line, lines, y, false);
        }
        self.push_box(line, child);
    }

    fn push_box(&self, line: &mut LineBox, child: InlineBox) {
        let (width, height) = measure_inline_box(&child);
        let x = line.width;
        line.width += width;
        line.height = line.height.max(height);
        line.children.push(PositionedInlineBox {
            inline_box: child,
            x,
            y: 0.0,
            width,
            height,
            baseline: 0.0,
        });
    }

    fn finish_line(
        &self,
        line: &mut LineBox,
        lines: &mut Vec<LineBox>,
        y: &mut f32,
        keep_empty: bool,
    ) {
        if line.is_empty() && !keep_empty {
            return;
        }
        if line.is_empty() {
            line.height = DEFAULT_FONT_SIZE * LINE_HEIGHT_FACTOR;
        }
        align_line(line);
        line.y = *y;
        *y += line.height;
        lines.push(line.clone());
        *line = LineBox::new(*y);
    }
}

/// Convenience function for inline layout.
pub fn layout_inline(children: &[InlineBox], container_width: f32) -> Vec<LineBox> {
    InlineLayoutEngine::new(container_width).layout(children)
}
