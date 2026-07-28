pub mod baseline;
pub mod fragment;
pub mod line;
pub mod metrics;
pub mod wrap;

pub use self::baseline::align_baselines;
pub use self::fragment::{FragmentKind, InlineFragment};
pub use self::line::LineBox;
pub use self::metrics::{measure_line_height, measure_text_width};
pub use self::wrap::{break_lines, layout_inline};

#[cfg(test)]
mod tests;
