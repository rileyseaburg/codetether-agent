//! Ordered catalog row groups.
type Row = (&'static str, (&'static str, f64));
mod part_0;
mod part_1;
mod part_2;
pub(super) fn all() -> impl Iterator<Item = &'static Row> {
    part_0::ROWS
        .iter()
        .chain(part_1::ROWS.iter())
        .chain(part_2::ROWS.iter())
}
