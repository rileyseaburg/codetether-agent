use std::fmt;

use serde::de::{Deserialize, Deserializer, IgnoredAny, SeqAccess, Visitor};

#[derive(Debug, Default)]
pub(super) struct CountSeq(pub usize);

impl<'de> Deserialize<'de> for CountSeq {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(CountVisitor)
    }
}

struct CountVisitor;

impl<'de> Visitor<'de> for CountVisitor {
    type Value = CountSeq;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a JSON array")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut count = 0usize;
        while seq.next_element::<IgnoredAny>()?.is_some() {
            count = count.saturating_add(1);
        }
        Ok(CountSeq(count))
    }
}
