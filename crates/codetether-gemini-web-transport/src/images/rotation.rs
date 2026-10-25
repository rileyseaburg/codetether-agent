use super::cookies::valid_value;
use serde::{Deserialize, Serialize};

pub(super) const NAMES: [&str; 4] = [
    "__Secure-1PSIDTS",
    "__Secure-3PSIDTS",
    "__Secure-1PSIDCC",
    "__Secure-3PSIDCC",
];
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Rotation {
    pub name: String,
    pub value: String,
    pub expires: u64,
}
impl Rotation {
    pub fn valid(&self) -> bool {
        NAMES.contains(&self.name.as_str()) && valid_value(&self.value)
    }
}
#[path = "rotation_parse.rs"]
mod parse;
pub(super) use parse::from_header;

pub(super) fn apply(current: &mut Vec<Rotation>, incoming: Vec<Rotation>) {
    for update in incoming {
        current.retain(|c| c.name != update.name);
        current.push(update);
    }
}
