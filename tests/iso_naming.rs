use parseapi::{Country, CountryDeep, Currency, Language, State};

#[test]
fn released_and_canonical_iso_responses_preserve_native_members() {
 for body in [r#"{"iso_alpha3":"USA","iso_numeric":840}"#] {
  let value:Country=serde_json::from_str(body).unwrap();
  assert_eq!(value.iso3.as_deref(),Some("USA"));assert_eq!(value.numeric,Some(840));
  let deep:CountryDeep=serde_json::from_str(body).unwrap();assert_eq!(deep.iso3.as_deref(),Some("USA"));assert_eq!(deep.numeric,Some(840));
 }
 for body in [r#"{"iso3":"hrv"}"#,r#"{"iso_alpha3":"hrv"}"#] {let value:Language=serde_json::from_str(body).unwrap();assert_eq!(value.iso3.as_deref(),Some("hrv"));}
 for body in [r#"{"iso":"US-CA"}"#,r#"{"iso_3166_2":"US-CA"}"#,r#"{"iso_subdivision":"US-CA"}"#] {let value:State=serde_json::from_str(body).unwrap();assert_eq!(value.iso.as_deref(),Some("US-CA"));}
 for body in [r#"{"numeric":0}"#,r#"{"iso_numeric":0}"#] {
  let value:Currency=serde_json::from_str(body).unwrap();assert_eq!(value.iso_numeric,Some(0));
 }
}

#[test]
fn canonical_explicit_null_wins_in_either_key_order() {
 for body in [r#"{"iso_alpha3":null,"iso_numeric":null,"iso3":"USA","numeric":840}"#,r#"{"iso3":"USA","numeric":840,"iso_alpha3":null,"iso_numeric":null}"#] {
  let value:Country=serde_json::from_str(body).unwrap();assert!(value.iso3.is_none());assert!(value.numeric.is_none());
  let deep:CountryDeep=serde_json::from_str(body).unwrap();assert!(deep.iso3.is_none());assert!(deep.numeric.is_none());
 }
 for body in [r#"{"iso_alpha3":null,"iso3":"hrv"}"#,r#"{"iso3":"hrv","iso_alpha3":null}"#] {let value:Language=serde_json::from_str(body).unwrap();assert!(value.iso3.is_none());}
 for body in [r#"{"iso_subdivision":null,"iso":"US-CA","iso_3166_2":"US-TX"}"#,r#"{"iso":"US-CA","iso_3166_2":"US-TX","iso_subdivision":null}"#] {let value:State=serde_json::from_str(body).unwrap();assert!(value.iso.is_none());}
 for body in [r#"{"iso_numeric":null,"numeric":840}"#,r#"{"numeric":840,"iso_numeric":null}"#] {let value:Currency=serde_json::from_str(body).unwrap();assert!(value.iso_numeric.is_none());}
}

#[test]
fn historical_deep_read_members_remain_available() {
 let language:parseapi::LanguageDeep=serde_json::from_str(r#"{"iso3":"eng","countries":["US"]}"#).unwrap();
 assert_eq!(language.iso3.as_deref(),Some("eng"));assert_eq!(language.countries.as_ref().unwrap(),&vec!["US".to_owned()]);
 let state:parseapi::StateDeep=serde_json::from_str(r#"{"fips":"06","capital":"Sacramento","area_codes":["916"],"tax":"Sales tax","tax_rate":0}"#).unwrap();
 assert_eq!(state.fips.as_deref(),Some("06"));assert_eq!(state.capital.as_deref(),Some("Sacramento"));assert_eq!(state.area_codes.as_ref().unwrap(),&vec!["916".to_owned()]);assert_eq!(state.tax_rate,Some(0.0));assert_eq!(state.tax.as_ref().unwrap().name.as_deref(),Some("Sales tax"));
}

#[test]
fn state_county_iso_code_keeps_canonical_null_and_exact_place() {
 for key in ["iso_code","iso_subdivision","iso","iso_3166_2"] {
  let value:State=serde_json::from_value(serde_json::json!({(key):"US-CO"})).unwrap();
  assert_eq!(value.iso_code.as_deref(),Some("US-CO"));
  assert_eq!(value.iso,value.iso_code);assert_eq!(value.iso_3166_2,value.iso_code);
 }
 for body in [r#"{"iso_code":null,"iso_subdivision":"US-CA","iso":"US-TX","iso_3166_2":"US-NY"}"#,r#"{"iso_subdivision":"US-CA","iso":"US-TX","iso_3166_2":"US-NY","iso_code":null}"#] {
  let value:State=serde_json::from_str(body).unwrap();
  assert!(value.iso_code.is_none()&&value.iso.is_none()&&value.iso_3166_2.is_none());
 }
 let feature=serde_json::json!({"type":"Feature","properties":{"county":"E09000033","iso_code":"GB-WSM","type":"district"},"geometry":null});
 let county:parseapi::County=serde_json::from_value(feature["properties"].clone()).unwrap();
 assert_eq!(county.iso_code.as_deref(),Some("GB-WSM"));assert_eq!(county.county,"E09000033");assert_eq!(county.kind.as_deref(),Some("district"));
 let list:parseapi::StateCounties=serde_json::from_str(r#"{"counties":[{"county":"29","iso_code":"FR-29"},{"county":"08031","iso_code":null},{"county":"old"}]}"#).unwrap();
 assert_eq!(list.counties[0].iso_code.as_deref(),Some("FR-29"));assert!(list.counties[1].iso_code.is_none());assert!(list.counties[2].iso_code.is_none());
}
