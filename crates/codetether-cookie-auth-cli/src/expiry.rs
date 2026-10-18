//! Cookie import expiry responsibilities.
use crate::row::CookieRow;

pub(super) fn cookie_expiry_bounds(rows: &[CookieRow]) -> (Option<i64>, Option<i64>) {
    let mut expiries = rows.iter().map(|row| row.expires_epoch).filter(|e| *e > 0);
    let first = expiries.next();
    let Some(mut min_epoch) = first else {
        return (None, None);
    };
    let mut max_epoch = min_epoch;
    for epoch in expiries {
        if epoch < min_epoch {
            min_epoch = epoch;
        }
        if epoch > max_epoch {
            max_epoch = epoch;
        }
    }
    (Some(min_epoch), Some(max_epoch))
}
