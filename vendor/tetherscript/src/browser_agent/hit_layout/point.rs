//! Point-in-box test.
pub fn point_in_box(px: i64, py: i64, x: i64, y: i64, w: i64, h: i64) -> bool {
    px >= x && py >= y && px < x + w && py < y + h
}
