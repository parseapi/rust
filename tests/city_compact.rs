use parseapi::CityResource;
use serde_json::Value;

#[test]
fn approved_city_matches_decode_without_source_or_score() {
 let rows: Value = serde_json::from_str(include_str!("fixtures/noise-approved.json")).unwrap();
 for row in rows.as_array().unwrap() {
  if let Some(cities) = row["body"]["data"].as_array() { for city in cities {
   let actual: CityResource = serde_json::from_value(city.clone()).unwrap();
   let matched = actual.r#match.unwrap();
   assert_eq!(matched.r#type, city["match"]["type"].as_str().unwrap());
   assert_eq!(matched.value, city["match"]["value"].as_str().unwrap());
   assert_eq!(matched.language.as_deref(), city["match"]["language"].as_str());
   assert!(matched.source.is_empty()); assert!(matched.edit_distance.is_none());
  } }
 }
}
