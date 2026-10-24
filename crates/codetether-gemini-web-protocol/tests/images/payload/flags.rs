use super::inner;
use serde_json::json;

#[test]
fn image_flag_slots_match_the_node_payload() {
    let value = inner("draw", false);
    let flags = [
        (7, 1),
        (10, 1),
        (11, 0),
        (18, 0),
        (27, 1),
        (49, 14),
        (53, 0),
        (67, 0),
        (68, 1),
        (79, 5),
        (91, 1),
        (96, 0),
        (98, 1),
    ];
    for (slot, expected) in flags {
        assert_eq!(value[slot], json!(expected));
    }
    let populated = [
        0, 1, 6, 7, 10, 11, 17, 18, 27, 30, 49, 53, 54, 55, 59, 61, 67, 68, 79, 91, 96, 98,
    ];
    for slot in 0..99 {
        assert_eq!(!value[slot].is_null(), populated.contains(&slot));
    }
}
