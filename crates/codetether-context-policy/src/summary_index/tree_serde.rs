//! JSON-safe serialization for summary trees with structured range keys.

use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::{SummaryNode, SummaryRange};

#[derive(Deserialize)]
#[serde(untagged)]
enum TreeRepresentation {
    Entries(Vec<(SummaryRange, SummaryNode)>),
    Legacy(BTreeMap<SummaryRange, SummaryNode>),
}

pub(super) fn serialize<S>(
    tree: &BTreeMap<SummaryRange, SummaryNode>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    tree.iter().collect::<Vec<_>>().serialize(serializer)
}

pub(super) fn deserialize<'de, D>(
    deserializer: D,
) -> Result<BTreeMap<SummaryRange, SummaryNode>, D::Error>
where
    D: Deserializer<'de>,
{
    let representation = TreeRepresentation::deserialize(deserializer)?;
    Ok(match representation {
        TreeRepresentation::Entries(entries) => entries.into_iter().collect(),
        TreeRepresentation::Legacy(tree) => tree,
    })
}
