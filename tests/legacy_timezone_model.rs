use serde_json::Value;

// Exact published 0.3.2 model declarations and serde attributes. Provenance is
// retained in notes/reviews/legacy-timezone-unknown-sdk-2026-10-30/sdk/legacy-timezone-published.json.
#[allow(dead_code)]
mod released_0_3_2 {
	include!("fixtures/legacy_timezone_0_3_2.rs");
}

#[test]
fn released_timezone_model_decodes_actual_corrected_seven_field_response() {
	let body = include_str!("fixtures/legacy-timezone-response.json");
	let value: Value = serde_json::from_str(body).unwrap();
	assert_eq!(value.as_object().unwrap().len(), 7);
	assert!(value["name"].is_null());
	assert_eq!(value["offset_minutes"].as_i64(), Some(-240));
	let result: released_0_3_2::Timezone = serde_json::from_str(body).unwrap();
	assert_eq!(result.timezone.as_deref(), Some("America/Havana"));
	assert!(result.name.is_none());
	assert_eq!(result.abbreviation.as_deref(), Some("CDT"));
	assert_eq!(result.offset.as_deref(), Some("-04:00"));
	assert_eq!(result.offset_minutes, Some(-240));
	assert_eq!(result.dst, Some(true));
	let next = result.next_dst.unwrap();
	assert_eq!(next.at, value["next_dst"]["at"].as_str().unwrap());
	assert_eq!(next.offset, value["next_dst"]["offset"].as_str().unwrap());
	assert_eq!(next.dst, value["next_dst"]["dst"].as_bool().unwrap());
	assert_eq!(next.abbreviation, value["next_dst"]["abbreviation"].as_str().unwrap());
	// The same released Option decoder also accepts its existing null form.
	let mut no_transition = value;
	no_transition["next_dst"] = Value::Null;
	let result: released_0_3_2::Timezone = serde_json::from_value(no_transition).unwrap();
	assert!(result.name.is_none() && result.next_dst.is_none());
}
