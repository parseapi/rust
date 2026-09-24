use parseapi::{Tariff, TariffSearch};

#[test]
fn tariff_preserves_unresolved_empty_and_populated_measures() {
	let core: Tariff = serde_json::from_str(r#"{"hts":"0101"}"#).unwrap();
	assert!(core.deep.is_none());
	for detail in ["{}", r#"{"measures":null,"origin":null,"effective_rate":null}"#] {
		let result: Tariff = serde_json::from_str(&format!(r#"{{"hts":"0101","deep":{detail}}}"#)).unwrap();
		let deep = result.deep.unwrap();
		assert!(deep.measures.is_none());
		assert!(deep.effective_rate.is_none());
	}
	let result: Tariff = serde_json::from_str(r#"{"hts":"0101.21.00.10","deep":{"origin":"CA","effective_rate":0,"units":[],"measures":[]}}"#).unwrap();
	let deep = result.deep.unwrap();
	assert_eq!(deep.origin.as_deref(), Some("CA"));
	assert_eq!(deep.effective_rate, Some(0.0));
	assert!(deep.measures.unwrap().is_empty());
	assert!(deep.units.unwrap().is_empty());
	let result: Tariff = serde_json::from_str(r#"{"hts":"0101.21.00.10","deep":{"measures":[{"heading":"9903.01.24","description":"Fixture condition","rate":null,"from":null,"until":null,"conditional":true,"future":true}]}}"#).unwrap();
	let measure = result.deep.unwrap().measures.unwrap().remove(0);
	assert_eq!(measure.conditional, Some(true));
	assert!(measure.rate.is_none());
}

#[test]
fn tariff_search_preserves_parent_context_and_decodes_older_responses() {
	for extra in ["", r#", "lineage":null"#] {
		let result: TariffSearch = serde_json::from_str(&format!(r#"{{"q":"horses","revision":"fixture","lines":[{{"hts":"0101.29.00.90","description":"Other","general":null{extra}}}]}}"#)).unwrap();
		assert!(result.lines[0].lineage.is_none());
	}
	let result: TariffSearch = serde_json::from_str(r#"{"q":"horses","revision":"fixture","lines":[{"hts":"0101.29.00.90","description":"Other","general":null,"lineage":["Live horses","Other horses"],"future":true},{"hts":"0101","description":"Live horses","general":null,"lineage":[]}] }"#).unwrap();
	assert_eq!(result.lines[0].lineage.as_deref().unwrap(), ["Live horses", "Other horses"]);
	assert!(result.lines[1].lineage.as_ref().unwrap().is_empty());
}

#[test]
fn tariff_edition_date_and_open_reason_remain_optional() {
	let old: Tariff = serde_json::from_str(r#"{"hts":"0101","deep":{}}"#).unwrap();
	assert!(old.edition.is_none() && old.date.is_none() && old.deep.unwrap().reason.is_none());
	let edition = "a".repeat(64);
	let result: Tariff = serde_json::from_str(&format!(r#"{{"edition":"{edition}","date":null,"deep":{{"reason":"future_reason","effective_rate":null,"measures":[]}}}}"#)).unwrap();
	assert_eq!(result.edition.as_deref(), Some(edition.as_str()));
	assert!(result.date.is_none());
	let deep = result.deep.unwrap();
	assert_eq!(deep.reason.as_deref(), Some("future_reason"));
	assert!(deep.effective_rate.is_none());
	let search: TariffSearch = serde_json::from_str(&format!(r#"{{"edition":"{edition}","date":"2026-09-15","lines":[]}}"#)).unwrap();
	assert_eq!(search.edition.as_deref(), Some(edition.as_str()));
	assert_eq!(search.date.as_deref(), Some("2026-09-15"));
}
