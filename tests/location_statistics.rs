use parseapi::*;

#[test]
fn location_statistics_preserve_unknown_and_zero() {
 for body in [r#"{}"#, r#"{"population":null,"population_period":null,"property_tax":null}"#] {
  let place: PostalDeep = serde_json::from_str(body).unwrap();
  assert!(place.property_tax.is_none() && place.population_period.is_none() && place.population.is_none());
 }
 let body = r#"{"population":0,"population_period":"2020-2024","property_tax":{"annual_median":0,"currency":"USD","period":"2020-2024"}}"#;
 let place: PostalDeep = serde_json::from_str(body).unwrap();
 assert_eq!(place.population, Some(0));
 assert_eq!(place.population_period.as_deref(), Some("2020-2024"));
 let tax = place.property_tax.unwrap();
 assert_eq!(tax.annual_median, 0.0);
 assert_eq!(tax.currency, "USD");
 assert_eq!(tax.period, "2020-2024");
 let district: DistrictDeep = serde_json::from_str(body).unwrap();
 assert_eq!(district.property_tax.unwrap().period, "2020-2024");
 for body in [r#"{"q":"a","addresses":[]}"#, r#"{"q":"a","addresses":[],"reason":null}"#] {
  let result: AddressSearch = serde_json::from_str(body).unwrap();
  assert!(result.reason.is_none());
 }
 let result: AddressSearch = serde_json::from_str(r#"{"q":"a","addresses":[],"reason":"future_reason"}"#).unwrap();
 assert_eq!(result.reason.as_deref(), Some("future_reason"));
}
