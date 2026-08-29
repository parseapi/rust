//! Live smoke against the edge. Canary-ready: env-driven, clean exit codes.
//!   PARSEAPI_KEY       required
//!   PARSEAPI_BASE_URL  optional override
//! Run: cargo run --example smoke

use parseapi::{Client, DeepOptions, Error};

const UA: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

struct Smoke {
	total: u32,
	failures: u32,
}

impl Smoke {
	fn check(&mut self, name: &str, ok: bool, detail: String) {
		self.total += 1;
		if !ok {
			self.failures += 1;
		}
		let mark = if ok { "ok  " } else { "FAIL" };
		if detail.is_empty() {
			println!("{mark} {name}");
		} else {
			println!("{mark} {name} ({detail})");
		}
	}

	fn ok<T>(&mut self, name: &str, result: Result<T, Error>, assert: impl FnOnce(&T) -> Option<String>) {
		match result {
			Ok(value) => match assert(&value) {
				None => self.check(name, true, String::new()),
				Some(problem) => self.check(name, false, problem),
			},
			Err(err) => self.check(name, false, err.to_string()),
		}
	}

	fn err<T>(&mut self, name: &str, result: Result<T, Error>, code: &str) {
		match result {
			Ok(_) => self.check(name, false, "expected error, got 200".to_string()),
			Err(err) => {
				let got = err.code().unwrap_or("transport");
				self.check(name, got == code, format!("got {got}"));
			}
		}
	}
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
	let parse = match Client::from_env() {
		Ok(client) => client,
		Err(err) => {
			eprintln!("{err}");
			std::process::exit(1);
		}
	};
	let deep = DeepOptions { deep: true };
	let mut s = Smoke { total: 0, failures: 0 };

	s.ok("ip", parse.ip("8.8.8.8", None).await, |r| {
		(r.ip == "8.8.8.8").then_some(None).unwrap_or(Some("wrong ip".into()))
	});
	s.ok("ip_self", parse.ip_self(None).await, |r| {
		(!r.ip.is_empty()).then_some(None).unwrap_or(Some("no ip".into()))
	});
	s.ok("continent", parse.continent("NA").await, |r| {
		(r.name == "North America").then_some(None).unwrap_or(Some("wrong name".into()))
	});
	s.ok("continent_countries", parse.continent_countries("NA").await, |r| {
		(!r.countries.is_empty()).then_some(None).unwrap_or(Some("empty".into()))
	});
	s.ok("country", parse.country("US").await, |r| {
		(r.iso3 == "USA").then_some(None).unwrap_or(Some("wrong iso3".into()))
	});
	s.ok("country_states", parse.country_states("US").await, |r| {
		(r.states.len() >= 50).then_some(None).unwrap_or(Some("too few".into()))
	});
	s.ok("state", parse.state("NC", "US").await, |r| {
		(r.name == "North Carolina").then_some(None).unwrap_or(Some("wrong".into()))
	});
	s.ok("state_districts", parse.state_districts("NC", "US").await, |r| {
		(!r.districts.is_empty()).then_some(None).unwrap_or(Some("empty".into()))
	});
	s.ok("district", parse.district("37081", None).await, |r| {
		r.name.contains("Guilford").then_some(None).unwrap_or(Some("wrong district".into()))
	});
	s.ok(
		"city",
		parse.city("charlotte", parseapi::CityOptions { country: Some("US".into()), ..Default::default() }).await,
		|r| {
			if r.name != "Charlotte" {
				return Some("wrong city".into());
			}
			if !r.id.starts_with("city_") {
				return Some("missing id".into());
			}
			None
		},
	);
	let city_id = match parse
		.city("charlotte", parseapi::CityOptions { country: Some("US".into()), ..Default::default() })
		.await
	{
		Ok(c) => c.id,
		Err(_) => String::new(),
	};
	if !city_id.is_empty() {
		s.ok("city_id", parse.city_id(&city_id).await, |r| {
			(r.id == city_id && r.name == "Charlotte")
				.then_some(None)
				.unwrap_or(Some("id mismatch".into()))
		});
	}
	s.ok(
		"city_search",
		parse
			.city_search(
				"char",
				parseapi::CitySearchOptions { country: Some("US".into()), limit: Some(5), ..Default::default() },
			)
			.await,
		|r| (!r.cities.is_empty()).then_some(None).unwrap_or(Some("empty".into())),
	);
	s.ok("city_nearest", parse.city_nearest(35.2271, -80.8431).await, |r| {
		(r.distance >= 0.0).then_some(None).unwrap_or(Some("no distance".into()))
	});
	s.ok("postal", parse.postal("28202", "US").await, |r| {
		(r.city.as_deref() == Some("Charlotte")).then_some(None).unwrap_or(Some("wrong city".into()))
	});
	s.ok(
		"postal_nearby",
		parse
			.postal_nearby("28202", "US", parseapi::PostalNearbyOptions { radius: Some(40.0), unit: None })
			.await,
		|r| (!r.nearby.is_empty()).then_some(None).unwrap_or(Some("empty".into())),
	);
	s.ok("postal_distance", parse.postal_distance("28202", "10001", "US").await, |r| {
		(r.distance > 800.0 && r.distance < 1000.0)
			.then_some(None)
			.unwrap_or(Some(format!("distance {}", r.distance)))
	});
	s.ok("email", parse.email("hello@gmail.com", None).await, |r| {
		r.valid.then_some(None).unwrap_or(Some("not valid".into()))
	});
	s.ok("phone", parse.phone("+14155552671", None).await, |r| {
		(r.phone.as_deref() == Some("+14155552671")).then_some(None).unwrap_or(Some("wrong phone".into()))
	});
	s.ok("domain", parse.domain("gmail.com", None).await, |r| {
		(!r.available).then_some(None).unwrap_or(Some("gmail available?".into()))
	});
	s.ok("mx", parse.mx("gmail.com").await, |r| {
		(!r.mx.is_empty()).then_some(None).unwrap_or(Some("no mx".into()))
	});
	s.ok("useragent", parse.useragent(UA, None).await, |r| {
		(r.browser.as_deref() == Some("Chrome"))
			.then_some(None)
			.unwrap_or(Some(format!("browser {:?}", r.browser)))
	});
	s.ok("currency", parse.currency("USD").await, |r| {
		(r.symbol.as_deref() == Some("$")).then_some(None).unwrap_or(Some("wrong symbol".into()))
	});
	s.ok("currency_rate", parse.currency_rate("USD", "EUR").await, |r| {
		(r.rate > 0.0 && r.rate < 10.0).then_some(None).unwrap_or(Some("bad rate".into()))
	});
	s.ok("language", parse.language("en").await, |r| {
		(r.language == "en" && r.name == "English")
			.then_some(None)
			.unwrap_or(Some("wrong language".into()))
	});
	s.ok("name", parse.name("BILLY O'SHALL").await, |r| {
		(r.name == "Billy O'Shall" && r.valid && r.gender.as_deref() == Some("male"))
			.then_some(None)
			.unwrap_or(Some("wrong name".into()))
	});
	s.ok("timezone", parse.timezone("America/New_York", None).await, |r| {
		(r.offset_minutes == -240 || r.offset_minutes == -300)
			.then_some(None)
			.unwrap_or(Some(format!("offset {}", r.offset_minutes)))
	});
	s.ok("holiday", parse.holiday("US", None).await, |r| {
		(r.holidays.len() > 5).then_some(None).unwrap_or(Some("too few".into()))
	});
	s.ok("holiday_date", parse.holiday_date("US", "2026-12-25").await, |r| {
		(r.holiday.as_ref().map(|h| h.name.as_str()) == Some("Christmas Day"))
			.then_some(None)
			.unwrap_or(Some("not christmas".into()))
	});
	s.ok("holiday null", parse.holiday_date("US", "2026-08-12").await, |r| {
		r.holiday.is_none().then_some(None).unwrap_or(Some("expected null".into()))
	});
	s.ok("elevation", parse.elevation(35.2271, -80.8431).await, |r| {
		r.elevation.is_some().then_some(None).unwrap_or(Some("no elevation".into()))
	});
	s.ok("point", parse.point(36.0726, -79.792, None).await, |r| {
		(r.country.as_deref() == Some("US"))
			.then_some(None)
			.unwrap_or(Some(format!("country {:?}", r.country)))
	});
	s.ok("weather", parse.weather(40.7128, -74.006, None).await, |r| {
		r.station.as_ref().is_some_and(|s| !s.id.is_empty()).then_some(None).unwrap_or(Some("no station".into()))
	});
	s.ok("emoji", parse.emoji("rocket").await, |r| {
		(r.emoji == "\u{1F680}").then_some(None).unwrap_or(Some("wrong emoji".into()))
	});
	s.ok(
		"emoji_search",
		parse.emoji_search("fire", parseapi::EmojiSearchOptions { limit: Some(5) }).await,
		|r| (!r.emojis.is_empty()).then_some(None).unwrap_or(Some("empty".into())),
	);
	s.ok("point deep triad", parse.point(36.0726, -79.792, deep).await, |r| {
		r.deep.is_some().then_some(None).unwrap_or(Some("deep missing".into()))
	});

	s.err("honest 404", parse.city("notarealcityxyz", None).await, "not_found");
	let bogus = Client::builder().api_key("bogus_key_123").retries(0).build().expect("client");
	s.err("bogus key 401", bogus.country("US").await, "invalid_api_key");

	println!("\n{}/{} passed", s.total - s.failures, s.total);
	std::process::exit(if s.failures == 0 { 0 } else { 1 });
}
