use parseapi::Timezone;
use serde_json::Value;

#[test]
fn approved_compact_places_and_unresolved_choices_decode() {
 let rows: Value = serde_json::from_str(include_str!("fixtures/time-compact.json")).unwrap();
 for row in rows.as_array().unwrap() {
  let body = &row["body"];
  let decoded: Timezone = serde_json::from_value(body.clone()).unwrap();
  if let Some(place) = body.get("location") {
   let actual = decoded.location.unwrap();
   assert_eq!(actual.name.as_deref(), place["name"].as_str(), "{}", row["id"]);
   assert_eq!(actual.id.as_deref(), place["id"].as_str());
   assert_eq!(actual.country.as_deref(), place["country"].as_str());
   assert_eq!(actual.state.as_deref(), place["state"].as_str());
   assert_eq!(actual.latitude, place["latitude"].as_f64());
   assert_eq!(actual.longitude, place["longitude"].as_f64());
   if let Some(status) = place["status"].as_str() { assert_eq!(actual.status, status); }
  } else { assert!(decoded.location.is_none()); }
 }
}
