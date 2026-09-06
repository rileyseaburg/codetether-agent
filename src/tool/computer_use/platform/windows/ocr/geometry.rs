//! Image-relative word boxes and enclosing line boxes without coordinate rounding.

use serde_json::{Value, json};
use windows::Foundation::Rect;

pub(super) fn bounds(rect: Rect) -> Value {
    json!({"x": rect.X, "y": rect.Y, "width": rect.Width, "height": rect.Height})
}

pub(super) fn union(current: Option<Rect>, next: Rect) -> Rect {
    let Some(current) = current else { return next };
    let (x, y) = (current.X.min(next.X), current.Y.min(next.Y));
    Rect {
        X: x,
        Y: y,
        Width: (current.X + current.Width).max(next.X + next.Width) - x,
        Height: (current.Y + current.Height).max(next.Y + next.Height) - y,
    }
}
