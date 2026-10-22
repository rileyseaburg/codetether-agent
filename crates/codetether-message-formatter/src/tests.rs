//! Existing formatter regression cases grouped by behavior.
#[path = "code_block_detection_tests.rs"]
mod code_block_detection_tests;
#[path = "inline_math_styled_separately_tests.rs"]
mod inline_math_styled_separately_tests;
#[path = "math_block_dollar_dollar_delimiters_tests.rs"]
mod math_block_dollar_dollar_delimiters_tests;
#[path = "math_display_block_is_boxed_tests.rs"]
mod math_display_block_is_boxed_tests;
#[path = "same_line_math_block_tests.rs"]
mod same_line_math_block_tests;
#[path = "take_fit_breaks_on_whitespace_tests.rs"]
mod take_fit_breaks_on_whitespace_tests;
#[path = "take_fit_hard_breaks_long_token_tests.rs"]
mod take_fit_hard_breaks_long_token_tests;
#[path = "take_fit_trims_leading_ws_at_start_tests.rs"]
mod take_fit_trims_leading_ws_at_start_tests;
#[path = "take_fit_whole_input_fits_tests.rs"]
mod take_fit_whole_input_fits_tests;
#[path = "unclosed_math_block_still_renders_tests.rs"]
mod unclosed_math_block_still_renders_tests;
#[path = "wrap_line_empty_returns_single_blank_tests.rs"]
mod wrap_line_empty_returns_single_blank_tests;
#[path = "wrap_line_is_bounded_by_input_chars_tests.rs"]
mod wrap_line_is_bounded_by_input_chars_tests;
#[path = "wrap_line_preserves_style_across_wraps_tests.rs"]
mod wrap_line_preserves_style_across_wraps_tests;
#[path = "wrap_line_splits_at_whitespace_tests.rs"]
mod wrap_line_splits_at_whitespace_tests;
#[path = "wrap_line_width_zero_is_noop_tests.rs"]
mod wrap_line_width_zero_is_noop_tests;
