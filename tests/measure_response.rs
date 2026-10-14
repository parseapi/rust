use parseapi::Measure;

#[test]
fn clean_invalid_ambiguous_and_legacy_measure_responses_decode() {
    let cases: Vec<serde_json::Value> = serde_json::from_str(include_str!("fixtures/measure-response.json")).unwrap();
    for body in cases {
        let answer: Measure = serde_json::from_value(body.clone()).unwrap();
        assert_eq!(answer.amount.as_deref(), body["amount"].as_str());
        assert_eq!(answer.reason.as_deref(), body["reason"].as_str());
        assert_eq!(answer.choices.len(), body["choices"].as_array().map_or(0, Vec::len));
        if body["reason"] == "ambiguous_unit" {
            assert_eq!(answer.choices[0].unit, "gal_us");
            assert_eq!(answer.choices[1].unit, "gal_imp");
        }
    }
}
