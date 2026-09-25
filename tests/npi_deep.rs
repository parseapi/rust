use parseapi::*;
#[test]
fn npi_published_detail() {
 let value: Provider = serde_json::from_str(r#"{"npi":"1881018208","valid":true,"sources":{"nppes":{"edition":"example","published_at":null,"through":"2026-09-20"}},"deep":{"updated_at":"2026-09-18","taxonomies":[{"taxonomy":"207Q00000X","primary":true,"license":"000123"}]}}"#).unwrap();
 assert!(value.sources.unwrap().nppes.unwrap().published_at.is_none());
 let deep=value.deep.unwrap();assert_eq!(deep.updated_at.as_deref(),Some("2026-09-18"));assert_eq!(deep.taxonomies.unwrap()[0].license.as_deref(),Some("000123"));
 let value: Provider = serde_json::from_str(r#"{"deep":{"taxonomies":[]}}"#).unwrap();assert_eq!(value.deep.unwrap().taxonomies.unwrap().len(),0);
 let value: Provider = serde_json::from_str(r#"{"deep":{"taxonomies":null}}"#).unwrap();assert!(value.deep.unwrap().taxonomies.is_none());
}
