use parseapi::Country;

#[test]
fn country_geography_preserves_decimals_zero_and_negative_elevations() {
	let country: Country = serde_json::from_str(r#"{"country":"XX","gdp_usd":30769700000000,"gdp_per_capita_usd":90026.5163,"population_growth_percent":-0.25,"urban_population_percent":0,"internet_users_percent":95.5,"land_area":10010.5,"water_area":0,"coastline":0,"elevation":0,"lowest_point":{"name":null,"elevation":-430.5},"highest_point":{"name":"Summit","elevation":8848.86}}"#).unwrap();
	let c = country.clone();
	assert_eq!(c.gdp_usd, Some(30769700000000.0));
	assert_eq!(c.gdp_per_capita_usd, Some(90026.5163));
	assert_eq!(c.population_growth_percent, Some(-0.25));
	assert_eq!(c.urban_population_percent, Some(0.0));
	assert_eq!(c.internet_users_percent, Some(95.5));
	assert_eq!(c.land_area, Some(10010.5));
	assert_eq!(c.water_area, Some(0.0));
	assert_eq!(c.coastline, Some(0.0));
	assert_eq!(c.elevation, Some(0.0));
	let low = c.lowest_point.as_ref().unwrap();
	assert_eq!(low.name, None);
	assert_eq!(low.elevation, -430.5);
	let high = c.highest_point.as_ref().unwrap();
	assert_eq!(high.name.as_deref(), Some("Summit"));
	assert_eq!(high.elevation, 8848.86);
}

#[test]
fn older_locked_and_null_country_geography_remain_unknown() {
	for body in ["{}", r#"{}"#, r#"{"land_area":null,"water_area":null,"coastline":null,"elevation":null,"lowest_point":null,"highest_point":null}"#] {
		let country: Country = serde_json::from_str(body).unwrap();
		{ let c = country;
			assert!(c.land_area.is_none() && c.water_area.is_none() && c.coastline.is_none() && c.elevation.is_none());
			assert!(c.lowest_point.is_none() && c.highest_point.is_none());
		}
	}
}


#[test]
fn minute_rates_and_historical_amounts_keep_their_values() {
 let weather: parseapi::Weather = serde_json::from_str(r#"{"minutes":[{"at":"2026-10-27T12:00:00Z","precipitation_rate":1.8,"precipitation_rate_in":0.071,"type":"rain"},{"at":"2026-10-27T12:01:00Z","precipitation_rate":0,"precipitation_rate_in":0},{"at":"2026-10-27T12:02:00Z","precipitation_rate":null,"precipitation_rate_in":null}],"history":{"precipitation":12.3,"precipitation_in":0.484}}"#).unwrap();
 let deep = weather;
 let minutes = deep.minutes.as_ref().unwrap();
 assert_eq!(minutes[0].precipitation_rate,Some(1.8));
 assert_eq!(minutes[0].precipitation_rate_in,Some(0.071));
 assert_eq!(minutes[1].precipitation_rate,Some(0.0));
 assert_eq!(minutes[2].precipitation_rate,None);
 let history = deep.history.unwrap();
 assert_eq!(history.precipitation,Some(12.3));
 assert_eq!(history.precipitation_in,Some(0.484));
}
