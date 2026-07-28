use super::*;

fn item(w: f32, h: f32) -> FlexItem {
    FlexItem {
        content_size: Size {
            width: w,
            height: h,
        },
        desired_size: Size {
            width: w,
            height: h,
        },
        ..FlexItem::default()
    }
}

fn cons(w: f32, h: f32) -> FlexConstraints {
    FlexConstraints {
        available: Size {
            width: w,
            height: h,
        },
    }
}

mod direction;
mod distribution;
mod positioning;
