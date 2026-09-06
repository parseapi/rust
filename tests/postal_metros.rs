use parseapi::{Postal, PostalDistance, PostalMetro, PostalNearby};
use serde_json::{json, Value};

#[test]
fn metro_observation_states_survive_every_postal_decoder() {
	for observation in [None, Some(Value::Null), Some(json!([])), Some(json!([{
		"code": "12345", "name": "Example area", "type": "future-area-type",
		"share": 0.75, "residential_share": 0, "business_share": 1, "other_share": null, "future": true
	}]))] {
		let mut member = json!({"postal":"12345", "country":"US", "city":null, "future":true});
		if let Some(value) = &observation { member["metros"] = value.clone(); }
		let postal: Postal = serde_json::from_value(member.clone()).unwrap();
		let mut nearby_json = member.clone();
		nearby_json["nearby"] = json!([member.clone()]);
		let nearby: PostalNearby = serde_json::from_value(nearby_json).unwrap();
		let distance: PostalDistance = serde_json::from_value(json!({"country":"US", "from":member.clone(), "to":member})).unwrap();
		let expected = observation.as_ref().and_then(Value::as_array);
		for value in [&postal.metros, &nearby.metros, &nearby.nearby[0].metros, &distance.from.metros, &distance.to.metros] {
			assert_eq!(value.as_ref().map(Vec::len), expected.map(Vec::len));
			if let Some(metro) = value.as_ref().and_then(|items| items.first()) {
				assert_eq!(metro.code, "12345");
				assert_eq!(metro.name, "Example area");
				assert_eq!(metro.r#type, "future-area-type");
				assert_eq!(metro.share, Some(0.75));
				assert_eq!(metro.residential_share, Some(0.0));
				assert_eq!(metro.business_share, Some(1.0));
				assert_eq!(metro.other_share, None);
			}
		}
	}
}

#[test]
fn missing_category_shares_remain_unknown() {
	let metro: PostalMetro = serde_json::from_value(json!({"code":"12345", "name":"Example area", "type":"metropolitan"})).unwrap();
	assert!(metro.share.is_none() && metro.residential_share.is_none() && metro.business_share.is_none() && metro.other_share.is_none());
}
