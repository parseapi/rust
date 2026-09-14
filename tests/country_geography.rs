use parseapi::Country;

#[test]
fn country_geography_preserves_decimals_zero_and_negative_elevations() {
	let country: Country = serde_json::from_str(r#"{"country":"XX","deep":{"land_area":10010.5,"water_area":0,"coastline":0,"elevation":0,"lowest_point":{"name":null,"elevation":-430.5},"highest_point":{"name":"Summit","elevation":8848.86}}}"#).unwrap();
	let c = country.deep.unwrap();
	assert_eq!(c.land_area, Some(10010.5));
	assert_eq!(c.water_area, Some(0.0));
	assert_eq!(c.coastline, Some(0.0));
	assert_eq!(c.elevation, Some(0.0));
	let low = c.lowest_point.unwrap();
	assert_eq!(low.name, None);
	assert_eq!(low.elevation, -430.5);
	let high = c.highest_point.unwrap();
	assert_eq!(high.name.as_deref(), Some("Summit"));
	assert_eq!(high.elevation, 8848.86);
}

#[test]
fn older_locked_and_null_country_geography_remain_unknown() {
	for body in ["{}", r#"{"deep":{}}"#, r#"{"deep":{"land_area":null,"water_area":null,"coastline":null,"elevation":null,"lowest_point":null,"highest_point":null}}"#] {
		let country: Country = serde_json::from_str(body).unwrap();
		assert_eq!(country.deep.is_none(), body == "{}");
		if let Some(c) = country.deep {
			assert!(c.land_area.is_none() && c.water_area.is_none() && c.coastline.is_none() && c.elevation.is_none());
			assert!(c.lowest_point.is_none() && c.highest_point.is_none());
		}
	}
}
