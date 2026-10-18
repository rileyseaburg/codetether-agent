//! Find the closest named color by squared RGB distance.

/// Convert RGB to closest named color
pub(super) fn rgb_to_named(r: u8, g: u8, b: u8) -> String {
    // Simple mapping to standard 8 colors
    let colors = [
        ("black", (0, 0, 0)),
        ("red", (128, 0, 0)),
        ("green", (0, 128, 0)),
        ("yellow", (128, 128, 0)),
        ("blue", (0, 0, 128)),
        ("magenta", (128, 0, 128)),
        ("cyan", (0, 128, 128)),
        ("white", (192, 192, 192)),
    ];

    let mut closest = "white";
    let mut min_distance = u32::MAX;

    for (name, (cr, cg, cb)) in colors.iter() {
        let dr = (r as i32 - *cr).unsigned_abs();
        let dg = (g as i32 - *cg).unsigned_abs();
        let db = (b as i32 - *cb).unsigned_abs();
        let distance = dr * dr + dg * dg + db * db;

        if distance < min_distance {
            min_distance = distance;
            closest = *name;
        }
    }

    closest.to_string()
}
