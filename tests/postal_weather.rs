use parseapi::*;

#[test]
fn postal_compact_tax_preserves_legacy_metadata_and_district_shape() {
    for body in [r#"{}"#, r#"{"deep":{}}"#, r#"{"deep":{"property_tax":null}}"#] {
        let postal: Postal = serde_json::from_str(body).unwrap();
        assert!(postal.deep.as_ref().and_then(|deep| deep.property_tax.as_ref()).is_none());
    }
    for amount in [0.0, 1234.5] {
        let postal: Postal = serde_json::from_value(serde_json::json!({"deep":{"currency":"USD","property_tax":{"annual_median":amount},"future":true}})).unwrap();
        let deep = postal.deep.unwrap();
        assert_eq!(deep.currency.as_deref(), Some("USD"));
        let tax = deep.property_tax.unwrap();
        assert_eq!(tax.annual_median, amount);
        assert!(tax.currency.is_none() && tax.period.is_none());
        assert_eq!(serde_json::to_value(tax).unwrap(), serde_json::json!({"annual_median":amount}));
    }
    let body = r#"{"property_tax":{"annual_median":0,"currency":"USD","period":"2020-2024"}}"#;
    let postal: PostalDeep = serde_json::from_str(body).unwrap();
    let tax = postal.property_tax.unwrap();
    assert_eq!(tax.currency.as_deref(), Some("USD"));
    assert_eq!(tax.period.as_deref(), Some("2020-2024"));
    let district: DistrictDeep = serde_json::from_str(body).unwrap();
    assert_eq!(district.property_tax.unwrap().period, "2020-2024");
    assert!(serde_json::from_str::<PropertyTax>(r#"{"annual_median":0}"#).is_err());
}

#[test]
fn weather_minute_canonical_presence_wins_over_old_payload_values() {
    for (body, rate, inches) in [
        (r#"{}"#, None, None),
        (r#"{"precipitation_rate":1.8,"precipitation_rate_in":0.071}"#, Some(1.8), Some(0.071)),
        (r#"{"precipitation":1.8,"precipitation_in":0.071}"#, Some(1.8), Some(0.071)),
        (r#"{"precipitation_rate":0,"precipitation_rate_in":0,"precipitation":9,"precipitation_in":9}"#, Some(0.0), Some(0.0)),
        (r#"{"precipitation_rate":null,"precipitation_rate_in":null,"precipitation":9,"precipitation_in":9}"#, None, None),
        (r#"{"precipitation_rate":null,"precipitation_in":0.071}"#, None, Some(0.071)),
    ] {
        let minute: WeatherMinute = serde_json::from_str(body).unwrap();
        assert_eq!(minute.precipitation_rate, rate, "{body}");
        assert_eq!(minute.precipitation_rate_in, inches, "{body}");
    }
    let history: WeatherHistory = serde_json::from_str(r#"{"precipitation":12.3,"precipitation_in":0.484}"#).unwrap();
    assert_eq!(history.precipitation, Some(12.3));
    assert_eq!(history.precipitation_in, Some(0.484));
}

#[test]
fn weather_unavailable_collections_differ_from_known_empty_results() {
    let absent: Weather = serde_json::from_str(r#"{}"#).unwrap();
    assert!(absent.deep.is_none());
    for body in [r#"{"deep":{}}"#, r#"{"deep":{"forecast":null,"alerts":null,"minutes":null,"hours":null,"days":null}}"#] {
        let weather: Weather = serde_json::from_str(body).unwrap();
        let deep = weather.deep.unwrap();
        assert!(deep.forecast.is_none() && deep.alerts.is_none() && deep.minutes.is_none() && deep.hours.is_none() && deep.days.is_none());
    }
    let empty: Weather = serde_json::from_str(r#"{"deep":{"forecast":[],"alerts":[],"minutes":[],"hours":[],"days":[],"future":true}}"#).unwrap();
    let deep = empty.deep.unwrap();
    assert!(deep.forecast.unwrap().is_empty() && deep.alerts.unwrap().is_empty() && deep.minutes.unwrap().is_empty() && deep.hours.unwrap().is_empty() && deep.days.unwrap().is_empty());
    let values: Weather = serde_json::from_str(r#"{"deep":{"forecast":[{"temperature":0}],"alerts":[{"event":"Example"}],"minutes":[{"precipitation_rate":0}],"hours":[{"temperature":0}],"days":[{"low":-1.5}]}}"#).unwrap();
    let deep = values.deep.unwrap();
    assert_eq!(deep.forecast.unwrap()[0].temperature, Some(0.0));
    assert_eq!(deep.alerts.unwrap()[0].event, "Example");
    assert_eq!(deep.minutes.unwrap()[0].precipitation_rate, Some(0.0));
    assert_eq!(deep.hours.unwrap()[0].temperature, Some(0.0));
    assert_eq!(deep.days.unwrap()[0].low, Some(-1.5));
}
