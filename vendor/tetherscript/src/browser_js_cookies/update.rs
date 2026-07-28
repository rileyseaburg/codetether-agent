//! Visible JS cookie pair updates.

use super::COOKIE_JAR;

pub(crate) fn visible(name: &str, value: &str, delete: bool) {
    COOKIE_JAR.with(|cookies| {
        let mut cookies = cookies.borrow_mut();
        cookies.retain(|(candidate, _)| candidate != name);
        if !delete {
            cookies.push((name.into(), value.into()));
        }
    });
}
