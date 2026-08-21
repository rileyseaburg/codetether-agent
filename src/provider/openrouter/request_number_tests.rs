use super::temperature;

#[test]
fn serializes_common_temperature_without_f32_artifacts() {
    let encoded = serde_json::to_string(&temperature(0.7)).expect("temperature JSON");

    assert_eq!(temperature(0.2), 0.2);
    assert_eq!(temperature(1.0), 1.0);
    assert_eq!(encoded, "0.7");
}
