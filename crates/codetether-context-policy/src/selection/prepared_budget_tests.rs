use crate::summary_index::{Granularity, SummaryIndex, SummaryNode, SummaryRange};
#[test]
fn affordable_child_summary_survives_an_unaffordable_parent() {
    let mut index = SummaryIndex::new();
    for (end, text) in [(16, "unaffordable"), (8, "usable contract")] {
        index.insert(
            SummaryRange::new(0, end).unwrap(),
            SummaryNode {
                content: text.into(),
                target_tokens: 10,
                granularity: Granularity::Phase,
                generation: 1,
                provenance: None,
            },
        );
    }
    let mut keep = vec![false; 16];
    keep[8..].fill(true);
    let selected =
        crate::selection::pack::select(&index, &mut keep, &[100; 16], vec![], 20, |messages| {
            if serde_json::to_string(messages)
                .unwrap()
                .contains("unaffordable")
            {
                1000
            } else {
                10
            }
        });
    assert_eq!(selected.len(), 1);
    assert_eq!(selected[0].content, "usable contract");
}
