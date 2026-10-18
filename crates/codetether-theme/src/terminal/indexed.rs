//! Map indexed colors to named terminal colors.

/// Convert indexed color to named color
pub(super) fn indexed_to_named(index: u8) -> String {
    match index {
        0..=15 => {
            // Standard 16 colors
            match index {
                0 => "black",
                1 => "red",
                2 => "green",
                3 => "yellow",
                4 => "blue",
                5 => "magenta",
                6 => "cyan",
                7 => "white",
                8 => "darkgray",
                9 => "lightred",
                10 => "lightgreen",
                11 => "lightyellow",
                12 => "lightblue",
                13 => "lightmagenta",
                14 => "lightcyan",
                15 => "lightgray",
                _ => "white",
            }
        }
        _ => "white",
    }
    .to_string()
}
