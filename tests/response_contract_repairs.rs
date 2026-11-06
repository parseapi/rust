use parseapi::*;

#[test]
fn postal_compact_property_tax_does_not_require_district_fields() {
    for body in [r#"{}"#, r#"{}"#, r#"{"property_tax":null}"#] {
        let postal: Postal = serde_json::from_str(body).unwrap();
        assert!(postal.property_tax.as_ref().is_none());
    }
    for amount in [0.0, 1234.5] {
        let body = serde_json::json!({"currency":"USD","property_tax":{"annual_median":amount},"future":true});
        let postal: Postal = serde_json::from_value(body).unwrap();
        let deep = postal.clone();
        assert_eq!(deep.currency.as_deref(), Some("USD"));
        let tax = deep.property_tax.unwrap();
        assert_eq!(tax.annual_median, amount);
        assert!(tax.currency.is_none() && tax.period.is_none());
        assert_eq!(serde_json::to_value(&tax).unwrap(), serde_json::json!({"annual_median":amount}));
    }
    let legacy: PostalDeep = serde_json::from_str(r#"{"property_tax":{"annual_median":1234.5,"currency":"USD","period":"2020-2024"}}"#).unwrap();
    let tax = legacy.property_tax.unwrap();
    assert_eq!(tax.currency.as_deref(), Some("USD"));
    assert_eq!(tax.period.as_deref(), Some("2020-2024"));
    assert_eq!(serde_json::to_value(tax).unwrap(), serde_json::json!({"annual_median":1234.5,"currency":"USD","period":"2020-2024"}));
    let district: DistrictDeep = serde_json::from_str(r#"{"property_tax":{"annual_median":0,"currency":"USD","period":"2020-2024"}}"#).unwrap();
    let tax = district.property_tax.unwrap();
    assert_eq!(tax.annual_median, 0.0);
    assert_eq!(tax.currency, "USD");
    assert_eq!(tax.period, "2020-2024");
    assert!(serde_json::from_str::<PropertyTax>(r#"{"annual_median":0}"#).is_err());
}

#[test]
fn point_deep_exposes_metric_elevation_and_electoral_values() {
    for body in [r#"{}"#, r#"{}"#, r#"{"elevation":null,"electoral":null}"#] {
        let point: Point = serde_json::from_str(body).unwrap();
        assert!(point.elevation.is_none());
        assert!(point.electoral.as_ref().is_none());
    }
    let zero: Point = serde_json::from_str(r#"{"elevation":0,"elevation_ft":0,"electoral":{"code":"0801","name":null,"congress":119,"future":true}}"#).unwrap();
    let deep = zero.clone();
    assert_eq!(deep.elevation, Some(0.0));
    assert_eq!(deep.elevation_ft, Some(0.0));
    let electoral = deep.electoral.unwrap();
    assert_eq!(electoral.code, "0801");
    assert_eq!(electoral.name, None);
    assert_eq!(electoral.congress, Some(119));
    let old: Point = serde_json::from_str(r#"{"elevation":-1.5,"resolution":30,"electoral":{"code":"0200","name":"At-large","congress":null}}"#).unwrap();
    let deep = old.clone();
    assert_eq!(deep.elevation, Some(-1.5));
    assert_eq!(deep.resolution, Some(30.0));
    assert_eq!(deep.electoral.unwrap().congress, None);
}

#[test]
fn weather_deep_preserves_unavailable_and_known_empty_collections() {
    let omitted: Weather = serde_json::from_str(r#"{}"#).unwrap();
    assert!(omitted.minutes.is_none());
    for body in [r#"{}"#, r#"{"forecast":null,"alerts":null,"minutes":null,"hours":null,"days":null}"#] {
        let weather: Weather = serde_json::from_str(body).unwrap();
        let deep = weather.clone();
        assert!(deep.forecast.is_none() && deep.alerts.is_none() && deep.minutes.is_none() && deep.hours.is_none() && deep.days.is_none());
    }
    let quiet: Weather = serde_json::from_str(r#"{"forecast":[],"alerts":[],"minutes":[],"hours":[],"days":[],"future":true}"#).unwrap();
    let deep = quiet.clone();
    assert!(deep.forecast.unwrap().is_empty());
    assert!(deep.alerts.unwrap().is_empty());
    assert!(deep.minutes.unwrap().is_empty());
    assert!(deep.hours.unwrap().is_empty());
    assert!(deep.days.unwrap().is_empty());
    let values: Weather = serde_json::from_str(r#"{"alerts":[{"event":"Example alert"}],"hours":[{"temperature":0}],"minutes":[{"precipitation_rate":0}],"days":[{"low":-1.5}],"forecast":[{"temperature":0}]}"#).unwrap();
    let deep = values.clone();
    assert_eq!(deep.alerts.unwrap()[0].event, "Example alert");
    assert_eq!(deep.hours.unwrap()[0].temperature, Some(0.0));
    assert_eq!(deep.minutes.unwrap()[0].precipitation_rate, Some(0.0));
    assert_eq!(deep.days.unwrap()[0].low, Some(-1.5));
    assert_eq!(deep.forecast.unwrap()[0].temperature, Some(0.0));
}
