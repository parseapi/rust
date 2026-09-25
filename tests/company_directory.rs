use parseapi::*;

const CORE: &str = r#"{"id":"co_222222222222","name":"Example","country":"US","website":null,"listings":[{"exchange":"Future Exchange","symbol":"A/B"}],"address":null}"#;
const PROFILE: &str = r#"{"id":"co_222222222222","name":"Example","country":"US","website":null,"listings":[{"exchange":"Future Exchange","symbol":"A/B"}],"address":null,"deep":{"legal_name":"Example Inc.","aliases":[],"jurisdiction":{"country":"US","state":null},"status":"future-status","websites":[{"domain":"example.com","url":null}],"identifiers":[{"type":"registration","authority":"future:registry","value":"0000123"}],"incorporated":null,"addresses":[{"type":"future-role","street":null,"city":"Example City","state":null,"postal":null,"country":"US"}],"industries":[{"type":"future-scheme","code":"001","name":null}],"parent":null,"description":null,"logo":null,"socials":[],"founded":{"value":"2006","precision":"year"},"sources":[{"type":"website","url":"https://example.com/","fields":["founded"],"observed_at":"2026-09-23T17:35:06.956Z","updated_at":null,"future":true}],"future":true},"future":true}"#;
const SEARCH: &str = r#"{"companies":[{"id":"co_222222222222","name":"Example","country":"US","website":null,"listings":[{"exchange":"Future Exchange","symbol":"A/B"}],"address":null,"deep":{"legal_name":"Example Inc.","aliases":[],"jurisdiction":{"country":"US","state":null},"status":"future-status","websites":[{"domain":"example.com","url":null}],"identifiers":[{"type":"registration","authority":"future:registry","value":"0000123"}],"incorporated":null,"addresses":[{"type":"future-role","street":null,"city":"Example City","state":null,"postal":null,"country":"US"}],"industries":[{"type":"future-scheme","code":"001","name":null}],"parent":null,"description":null,"logo":null,"socials":[],"founded":{"value":"2006","precision":"year"},"sources":[{"type":"website","url":"https://example.com/","fields":["founded"],"observed_at":"2026-09-23T17:35:06.956Z","updated_at":null,"future":true}],"future":true},"future":true,"match":{"field":"future-field","value":"0000123","type":"future-scheme","authority":null,"exchange":null,"future":true}}],"next":"opaque+/="}"#;
const COVERAGE: &str = r#"{"scope":"sample","label":"Company directory","description":"Edition profiles","snapshot_at":"2026-09-23T17:35:06.956Z","companies":0,"countries":[],"with_website":0,"with_listings":0,"with_address":0,"future":true}"#;

#[test]
fn directory_fields_and_open_codes() {
	let profile: CompanyProfile = serde_json::from_str(PROFILE).unwrap();
	let deep = profile.deep.unwrap();
	assert_eq!(deep.identifiers.unwrap()[0].value, "0000123");
	assert_eq!(deep.status.as_deref(), Some("future-status"));
	assert!(deep.socials.unwrap().is_empty());
	assert_eq!(deep.founded.unwrap().precision, "year");
	assert!(deep.sources.unwrap()[0].updated_at.is_none());
	assert!(deep.jurisdiction.unwrap().state.is_none());
	let search: CompanySearch = serde_json::from_str(SEARCH).unwrap();
	assert_eq!(search.next.as_deref(), Some("opaque+/="));
	assert_eq!(
		search.companies[0].r#match.field.as_deref(),
		Some("future-field")
	);
	assert_eq!(
		search.companies[0]
			.deep
			.as_ref()
			.unwrap()
			.legal_name
			.as_deref(),
		Some("Example Inc.")
	);
	let coverage: CompanyCoverage = serde_json::from_str(COVERAGE).unwrap();
	assert_eq!(coverage.companies, 0);
	assert!(coverage.countries.is_empty());
}

#[test]
fn omitted_empty_partial_nullable_deep_and_empty_search() {
	let core: CompanyProfile = serde_json::from_str(CORE).unwrap();
	assert!(core.deep.is_none());
	for deep in [
		"{}",
		r#"{"legal_name":"Old name"}"#,
		r#"{"socials":null,"sources":null,"description":null,"employees":null}"#,
	] {
		let text =
			format!(r#"{{"id":"co_222222222222","name":"Example","listings":[],"deep":{deep}}}"#);
		let profile: CompanyProfile = serde_json::from_str(&text).unwrap();
		let value = profile.deep.unwrap();
		assert!(value.socials.is_none());
		assert!(value.sources.is_none());
		assert!(value.founded.is_none());
		assert!(value.employees.is_none());
	}
	let profile: CompanyProfile = serde_json::from_str(r#"{"id":"co_222222222222","name":"Example","listings":[],"deep":{"socials":[],"sources":[],"founded":{"value":"spring 2006","precision":"season"}}}"#).unwrap();
	let deep = profile.deep.unwrap();
	assert!(deep.socials.unwrap().is_empty());
	assert!(deep.sources.unwrap().is_empty());
	assert_eq!(deep.founded.unwrap().precision, "season");
	let empty: CompanySearch = serde_json::from_str(r#"{"companies":[],"next":null}"#).unwrap();
	assert!(empty.companies.is_empty());
	assert!(empty.next.is_none());
}

#[test]
fn employee_observations_preserve_zero_false_dates_and_open_codes() {
	let cases = [
		(r#"{"count":0,"as_of":"2025-12-31","scope":"legal_entity","method":"reported","approximate":false}"#, 0, "2025-12-31", "legal_entity", "reported", false),
		(r#"{"count":12500,"as_of":"2026-06-30","scope":"consolidated_group","method":"reported","approximate":true}"#, 12500, "2026-06-30", "consolidated_group", "reported", true),
		(r#"{"count":7,"as_of":"2026-01-15","scope":"future_scope","method":"future_method","approximate":false,"future":null}"#, 7, "2026-01-15", "future_scope", "future_method", false),
	];
	for (json, count, date, scope, method, approximate) in cases {
		let text=format!(r#"{{"id":"co_222222222222","name":"Example","deep":{{"employees":{json}}}}}"#);
		let profile: CompanyProfile=serde_json::from_str(&text).unwrap();
		let search_text=format!(r#"{{"companies":[{},"match":{{"field":"name"}}}}],"next":null}}"#, &text[..text.len()-1]);
		let page: CompanySearch=serde_json::from_str(&search_text).unwrap();
		for value in [profile.deep.as_ref().unwrap().employees.as_ref().unwrap(), page.companies[0].deep.as_ref().unwrap().employees.as_ref().unwrap()] {
			assert_eq!(value.count,count);assert_eq!(value.as_of,date);
			assert_eq!(value.scope,scope);assert_eq!(value.method,method);assert_eq!(value.approximate,approximate);
		}
	}
}

#[test]
fn registrations_preserve_source_roles_leading_zeros_and_nulls() {
 let item = r#"{"authority":"RA000599","number":"0001234567","jurisdiction":{"country":"US","state":"CO"},"role":"domestic","legal_form":{"code":"DNC","name":"Domestic Non-profit Corporation"},"status":"Good Standing","formation_date":"2004-02-29","address":{"kind":"principal","line1":"12 Main St.","line2":"Suite 2","city":"Example","state":"CO","postal":"00123-0001","country_raw":"US"},"future":"retained"}"#;
 for deep in ["{}".to_string(),r#"{"registrations":null}"#.to_string(),r#"{"registrations":[]}"#.to_string(),format!(r#"{{"registrations":[{item}]}}"#)] {
  let text=format!(r#"{{"id":"co_222222222222","name":"Example","deep":{deep}}}"#);
  let profile:CompanyProfile=serde_json::from_str(&text).unwrap();
  let search=format!(r#"{{"companies":[{},"match":{{"field":"identifier"}}}}],"next":null}}"#,&text[..text.len()-1]);
  let page:CompanySearch=serde_json::from_str(&search).unwrap();
  for d in [profile.deep.as_ref().unwrap(),page.companies[0].deep.as_ref().unwrap()] {
   if let Some(rows)=&d.registrations {if let Some(r)=rows.first() {assert_eq!(r.number,"0001234567");assert_eq!(r.jurisdiction.country,"US");assert_eq!(r.formation_date.as_deref(),Some("2004-02-29"));assert_eq!(r.address.as_ref().unwrap().postal.as_deref(),Some("00123-0001"));assert_eq!(r.address.as_ref().unwrap().kind,"principal");}}
   if deep==r#"{"registrations":[]}"# {assert!(d.registrations.as_ref().unwrap().is_empty());}
  }
 }
 let p:CompanyProfile=serde_json::from_str(r#"{"id":"co_222222222222","name":"Example","deep":{"registrations":[{"role":"future_role","formation_date":null,"address":null}]}}"#).unwrap();
 let r=&p.deep.as_ref().unwrap().registrations.as_ref().unwrap()[0];assert_eq!(r.role,"future_role");assert!(r.formation_date.is_none()&&r.address.is_none());
}
