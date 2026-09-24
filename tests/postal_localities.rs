use parseapi::Postal;
use serde_json::{json, Value};

#[test]
fn choices_preserve_observation_without_inferring_city() {
	let choice = json!({"city":"SYDNEY","state":"NSW","state_name":"New South Wales","future":true});
	for observation in [None, Some(Value::Null), Some(json!([])), Some(json!([choice.clone()])), Some(json!([choice, {"city":"HAYMARKET","state":"NSW","state_name":"New South Wales"}]))] {
		let mut body = json!({"postal":"2000","country":"AU","city":null});
		if let Some(value) = &observation { body["localities"] = value.clone(); }
		let postal: Postal = serde_json::from_value(body).unwrap();
		assert!(postal.city.is_none());
		assert_eq!(postal.localities.as_ref().map(Vec::len), observation.as_ref().and_then(Value::as_array).map(Vec::len));
		if let Some(choice) = postal.localities.as_ref().and_then(|items| items.first()) {
			assert_eq!(choice.city, "SYDNEY");
			assert_eq!(choice.state, "NSW");
			assert_eq!(choice.state_name, "New South Wales");
		}
	}
}
