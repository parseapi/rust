use parseapi::Postal;
use std::collections::BTreeMap;

#[test]
fn already_served_postal_fields_preserve_old_payloads_and_values() {
 let cases: BTreeMap<String, Postal> = serde_json::from_str(include_str!("fixtures/postal-additive.json")).unwrap();
 for (name, postal) in &cases {
  let metro = &postal.deep.as_ref().unwrap().metros.as_ref().unwrap()[0];
  assert_eq!(metro.share, Some(0.125), "{}", name);
  if name == "missing" || name == "null" { assert!(postal.r#type.is_none()); assert!(metro.share_percent.is_none()); }
 }
 assert_eq!(cases["current"].r#type.as_deref(), Some("standard"));
 assert_eq!(cases["current"].deep.as_ref().unwrap().metros.as_ref().unwrap()[0].share_percent, Some(12.5));
 assert_eq!(cases["zero"].deep.as_ref().unwrap().metros.as_ref().unwrap()[0].share_percent, Some(0.0));
 assert_eq!(cases["future"].r#type.as_deref(), Some("future_custom"));
 assert_eq!(cases["future"].deep.as_ref().unwrap().metros.as_ref().unwrap()[0].share_percent, Some(100.0));
}
