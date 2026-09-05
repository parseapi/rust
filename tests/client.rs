use parseapi::*;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex, OnceLock};

// Process env is shared across test threads, serialize env-touching tests.
fn env_lock() -> &'static Mutex<()> {
	static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
	LOCK.get_or_init(|| Mutex::new(()))
}

#[derive(Debug, Clone)]
struct Recorded {
	target: String,
	headers: HashMap<String, String>,
}

struct TestServer {
	base_url: String,
	requests: Arc<Mutex<Vec<Recorded>>>,
}

impl TestServer {
	/// Serves the canned (status, body) responses in order, one connection each.
	fn start(responses: Vec<(u16, &'static str)>) -> TestServer {
		Self::start_with_headers(
			responses
				.into_iter()
				.map(|(status, body)| (status, body, String::new()))
				.collect(),
		)
	}

	fn start_with_headers(responses: Vec<(u16, &'static str, String)>) -> TestServer {
		let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
		let base_url = format!("http://{}", listener.local_addr().expect("addr"));
		let requests: Arc<Mutex<Vec<Recorded>>> = Arc::new(Mutex::new(Vec::new()));
		let recorded = Arc::clone(&requests);

		std::thread::spawn(move || {
			for (status, body, headers) in responses {
				let (mut stream, _) = match listener.accept() {
					Ok(conn) => conn,
					Err(_) => return,
				};
				let mut raw = Vec::new();
				let mut buf = [0_u8; 1024];
				while !raw.windows(4).any(|w| w == b"\r\n\r\n") {
					match stream.read(&mut buf) {
						Ok(0) => break,
						Ok(n) => raw.extend_from_slice(&buf[..n]),
						Err(_) => break,
					}
				}
				let text = String::from_utf8_lossy(&raw);
				let mut lines = text.split("\r\n");
				let request_line = lines.next().unwrap_or_default();
				let target = request_line
					.split(' ')
					.nth(1)
					.unwrap_or_default()
					.to_string();
				let mut request_headers = HashMap::new();
				for line in lines {
					if let Some((name, value)) = line.split_once(':') {
						request_headers
							.insert(name.trim().to_lowercase(), value.trim().to_string());
					}
				}
				recorded.lock().unwrap().push(Recorded {
					target,
					headers: request_headers,
				});

				let response = format!(
					"HTTP/1.1 {status} X\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n{headers}\r\n{body}",
					body.len(),
				);
				let _ = stream.write_all(response.as_bytes());
			}
		});

		TestServer { base_url, requests }
	}

	fn client(&self) -> Client {
		Client::builder()
			.api_key("test_key_123")
			.base_url(&self.base_url)
			.retries(0)
			.build()
			.expect("client")
	}

	fn requests(&self) -> Vec<Recorded> {
		self.requests.lock().unwrap().clone()
	}
}

macro_rules! url_test {
	($name:ident, $client:ident => $call:expr, $expected:expr) => {
		#[tokio::test]
		async fn $name() {
			let server = TestServer::start(vec![(200, "{}")]);
			let $client = server.client();
			let _ = $call.await;
			assert_eq!(server.requests()[0].target, $expected);
		}
	};
}

url_test!(url_ip, c => c.ip("8.8.8.8", None), "/ip/8.8.8.8");
url_test!(url_ip_self, c => c.ip_self(None), "/ip");
url_test!(
	url_ip_deep,
	c => c.ip("8.8.8.8", parseapi::IpOptions::default().deep(true)),
	"/ip/8.8.8.8?deep=true"
);
url_test!(url_continent, c => c.continent("NA"), "/continent/NA");
url_test!(
	url_continent_countries,
	c => c.continent_countries("NA"),
	"/continent/NA/countries"
);
url_test!(url_bloc, c => c.bloc("EU"), "/bloc/EU");
url_test!(
	url_bloc_countries,
	c => c.bloc_countries("SCHENGEN"),
	"/bloc/SCHENGEN/countries"
);
url_test!(url_country, c => c.country("US"), "/country/US");
url_test!(url_country_states, c => c.country_states("US"), "/country/US/states");
url_test!(url_state, c => c.state("NC", parseapi::StateOptions::default().country("US")), "/state/NC?country=US");
url_test!(url_state_name, c => c.state("colorado", None), "/state/colorado");
url_test!(
	url_state_districts,
	c => c.state_districts("NC", parseapi::StateDistrictsOptions::default().country("US")),
	"/state/NC/districts?country=US"
);
url_test!(url_district, c => c.district("37081", None), "/district/37081");
url_test!(
	url_city,
	c => c.city("charlotte", parseapi::CityOptions::default().state("NC")),
	"/city/charlotte?state=NC"
);
url_test!(url_city_id, c => c.city_id("city_mb8mbqrkz8zb"), "/city/id/city_mb8mbqrkz8zb");
url_test!(
	url_city_search,
	c => c.city_search("char", parseapi::CitySearchOptions::default().country("US").limit(10)),
	"/city?q=char&country=US&limit=10"
);
url_test!(
	url_city_nearest,
	c => c.city_nearest(35.2271, -80.8431),
	"/city?lat=35.2271&lon=-80.8431"
);
url_test!(
	url_city_nearby,
	c => c.city_nearby("denver", parseapi::CityNearbyOptions::default().radius(8.0).unit("mi").limit(3)),
	"/city/denver/nearby?radius=8&unit=mi&limit=3"
);
url_test!(url_postal, c => c.postal("28202", parseapi::PostalOptions::default().country("US")), "/postal/28202?country=US");
url_test!(url_postal_bare, c => c.postal("SW1A 1AA", None), "/postal/SW1A%201AA");
url_test!(
	url_postal_nearby,
	c => c.postal_nearby("28202", parseapi::PostalNearbyOptions::default().radius(40.0).unit("km").country("US")),
	"/postal/28202/nearby?country=US&radius=40&unit=km"
);
url_test!(
	url_postal_distance,
	c => c.postal_distance("28202", "10001", parseapi::PostalDistanceOptions::default().country("US")),
	"/postal/28202/distance/10001?country=US"
);
url_test!(url_email, c => c.email("a@b.com", None), "/email/a%40b.com");
url_test!(url_vat, c => c.vat("DE136695976", None), "/vat/DE136695976");
url_test!(
	url_iban,
	c => c.iban("DE89370400440532013000", None),
	"/iban/DE89370400440532013000"
);
url_test!(
	url_iban_country,
	c => c.iban("89370400440532013000", parseapi::IbanOptions::default().country("DE")),
	"/iban/89370400440532013000?country=DE"
);
url_test!(url_npi, c => c.npi("1881018208", None), "/npi/1881018208");
url_test!(
	url_npi_deep,
	c => c.npi("1881018208", parseapi::NpiOptions::default().deep(true)),
	"/npi/1881018208?deep=true"
);
url_test!(
	url_vat_from_deep,
	c => c.vat("DE136695976", parseapi::VatOptions::default().from("IE6388047V").deep(true)),
	"/vat/DE136695976?from=IE6388047V&deep=true"
);
url_test!(
	url_phone_encodes_plus,
	c => c.phone("+14155552671", parseapi::PhoneOptions::default().deep(true)),
	"/phone/%2B14155552671?deep=true"
);
url_test!(
	url_carrier_encodes_plus,
	c => c.carrier("+14155552671", None),
	"/carrier/%2B14155552671"
);
url_test!(
	url_caller_with_country,
	c => c.caller("4155552671", parseapi::CallerOptions::default().country("US")),
	"/caller/4155552671?country=US"
);
url_test!(url_hlr, c => c.hlr("+447712345678", None), "/hlr/%2B447712345678");
url_test!(url_domain, c => c.domain("example.com", None), "/domain/example.com");
url_test!(url_asn, c => c.asn("AS13335"), "/asn/AS13335");
url_test!(url_mac, c => c.mac("00:1B:63:84:45:E6"), "/mac/00%3A1B%3A63%3A84%3A45%3AE6");
url_test!(url_mx, c => c.mx("example.com"), "/mx/example.com");
url_test!(url_useragent, c => c.useragent("TestUA/1.0", None), "/useragent");
url_test!(url_vin, c => c.vin("1HGCM82633A004352", None), "/vin/1HGCM82633A004352");
url_test!(
	url_vin_deep,
	c => c.vin("1HGCM82633A004352", parseapi::VinOptions::default().deep(true)),
	"/vin/1HGCM82633A004352?deep=true"
);
url_test!(url_currency, c => c.currency("USD"), "/currency/USD");
url_test!(
	url_currency_rate,
	c => c.currency_rate("USD", "EUR", None),
	"/currency/USD/EUR"
);
url_test!(
	url_currency_rate_date_amount,
	c => c.currency_rate("USD", "JPY", parseapi::CurrencyRateOptions::default().date("2026-08-28").amount(100.0)),
	"/currency/USD/JPY?date=2026-08-28&amount=100"
);
url_test!(url_language, c => c.language("en"), "/language/en");
url_test!(url_name_encodes_spaces, c => c.name("Smith, John"), "/name/Smith%2C%20John");
url_test!(
	url_timezone_encodes_slash,
	c => c.timezone("America/New_York", None),
	"/timezone/America%2FNew_York"
);
url_test!(
	url_holiday,
	c => c.holiday("US", parseapi::HolidayOptions::default().year(1955)),
	"/holiday/US?year=1955"
);
url_test!(
	url_holiday_date,
	c => c.holiday_date("US", "2026-12-25"),
	"/holiday/US/2026-12-25"
);
url_test!(
	url_elevation,
	c => c.elevation(35.2, -80.8),
	"/elevation?lat=35.2&lon=-80.8"
);
url_test!(
	url_point_deep,
	c => c.point(36.0726, -79.792, parseapi::PointOptions::default().deep(true)),
	"/point?lat=36.0726&lon=-79.792&deep=true"
);
url_test!(
	url_weather,
	c => c.weather(40.7128, -74.006, parseapi::WeatherOptions::default().deep(true)),
	"/weather?lat=40.7128&lon=-74.006&deep=true"
);
url_test!(url_emoji, c => c.emoji("rocket"), "/emoji/rocket");
url_test!(
	url_emoji_search,
	c => c.emoji_search("fire", parseapi::EmojiSearchOptions::default().limit(20)),
	"/emoji?q=fire&limit=20"
);

#[tokio::test]
async fn sends_key_and_user_agent() {
	let server = TestServer::start(vec![(200, "{}")]);
	let client = server.client();
	let _ = client.country("US").await;
	let recorded = server.requests();
	assert_eq!(recorded[0].headers["x-api-key"], "test_key_123");
	let ua = &recorded[0].headers["user-agent"];
	assert!(ua.starts_with("parseapi-rust/0."), "unexpected UA {ua}");
}

#[tokio::test]
async fn useragent_overrides_ua_header() {
	let server = TestServer::start(vec![(200, "{}")]);
	let client = server.client();
	let _ = client.useragent("Mozilla/5.0 (Test)", None).await;
	assert_eq!(
		server.requests()[0].headers["user-agent"],
		"Mozilla/5.0 (Test)"
	);
}

#[test]
fn missing_key_is_a_config_error() {
	let _guard = env_lock().lock().unwrap();
	let saved = std::env::var("PARSEAPI_KEY").ok();
	std::env::remove_var("PARSEAPI_KEY");
	let result = Client::from_env();
	if let Some(saved) = saved {
		std::env::set_var("PARSEAPI_KEY", saved);
	}
	assert!(matches!(result, Err(Error::Config(_))));
}

#[tokio::test]
async fn env_key_fallback() {
	let server = TestServer::start(vec![(200, "{}")]);
	let client = {
		let _guard = env_lock().lock().unwrap();
		let saved = std::env::var("PARSEAPI_KEY").ok();
		std::env::set_var("PARSEAPI_KEY", "env_key_456");
		let client = Client::builder()
			.base_url(&server.base_url)
			.retries(0)
			.build();
		match saved {
			Some(saved) => std::env::set_var("PARSEAPI_KEY", saved),
			None => std::env::remove_var("PARSEAPI_KEY"),
		}
		client.expect("client")
	};
	let _ = client.country("US").await;
	assert_eq!(server.requests()[0].headers["x-api-key"], "env_key_456");
}

#[tokio::test]
async fn error_carries_the_api_shape() {
	let body = r#"{"code":"not_found","message":"City not found","docs":"https://parseapi.com/docs#not_found","request_id":"req_abc"}"#;
	let server = TestServer::start(vec![(404, body)]);
	let client = server.client();
	let err = client.city("notarealcityxyz", None).await.unwrap_err();
	match err {
		Error::Api {
			status,
			code,
			message,
			docs,
			request_id,
			..
		} => {
			assert_eq!(status, 404);
			assert_eq!(code, "not_found");
			assert_eq!(message, "City not found");
			assert_eq!(docs.as_deref(), Some("https://parseapi.com/docs#not_found"));
			assert_eq!(request_id.as_deref(), Some("req_abc"));
		}
		other => panic!("expected Api error, got {other:?}"),
	}
}

#[tokio::test]
async fn non_json_error_body() {
	let server = TestServer::start(vec![(400, "gateway timeout")]);
	let client = server.client();
	let err = client.country("US").await.unwrap_err();
	assert_eq!(err.code(), Some("unknown_error"));
	assert_eq!(err.status(), Some(400));
}

#[tokio::test]
async fn retries_500_then_succeeds() {
	let server = TestServer::start(vec![
		(500, r#"{"code":"server_error","message":"boom"}"#),
		(200, r#"{"country":"us","iso3":"USA"}"#),
	]);
	let client = Client::builder()
		.api_key("test_key_123")
		.base_url(&server.base_url)
		.retries(2)
		.build()
		.expect("client");
	let country = client.country("US").await.expect("retried response");
	assert_eq!(country.iso3, "USA");
	assert_eq!(server.requests().len(), 2);
}

#[tokio::test]
async fn does_not_retry_404() {
	let server = TestServer::start(vec![(404, r#"{"code":"not_found","message":"nope"}"#)]);
	let client = Client::builder()
		.api_key("test_key_123")
		.base_url(&server.base_url)
		.retries(2)
		.build()
		.expect("client");
	let err = client.country("XX").await.unwrap_err();
	assert_eq!(err.code(), Some("not_found"));
	assert_eq!(server.requests().len(), 1);
}

#[tokio::test]
async fn gives_up_after_retries() {
	let rate_limited = r#"{"code":"rate_limited","message":"slow down"}"#;
	let server = TestServer::start(vec![
		(429, rate_limited),
		(429, rate_limited),
		(429, rate_limited),
	]);
	let client = Client::builder()
		.api_key("test_key_123")
		.base_url(&server.base_url)
		.retries(2)
		.build()
		.expect("client");
	let err = client.country("US").await.unwrap_err();
	assert_eq!(err.code(), Some("rate_limited"));
	assert_eq!(server.requests().len(), 3);
}

#[test]
fn debug_output_redacts_key() {
	let builder = Client::builder().api_key("secret_key_for_debug_test");
	let output = format!("{builder:?}");
	assert!(!output.contains("secret_key_for_debug_test"));
	assert!(output.contains("[REDACTED]"));
	let client = builder.build().expect("client");
	let output = format!("{client:#?}");
	assert!(!output.contains("secret_key_for_debug_test"));
	assert!(output.contains("[REDACTED]"));
}

#[tokio::test]
async fn empty_explicit_key_uses_environment() {
	let server = TestServer::start(vec![(200, "{}")]);
	let client = {
		let _guard = env_lock().lock().unwrap();
		let saved = std::env::var("PARSEAPI_KEY").ok();
		std::env::set_var("PARSEAPI_KEY", "env_key_456");
		let client = Client::builder()
			.api_key("")
			.base_url(&server.base_url)
			.retries(0)
			.build();
		match saved {
			Some(saved) => std::env::set_var("PARSEAPI_KEY", saved),
			None => std::env::remove_var("PARSEAPI_KEY"),
		}
		client.expect("client")
	};
	client.country("US").await.expect("country");
	assert_eq!(server.requests()[0].headers["x-api-key"], "env_key_456");
}

url_test!(
	url_date,
	c => c.date("03/04/2026", parseapi::DateOptions::default().format("mdy").to("2026-12-25")),
	"/date/03%2F04%2F2026?format=mdy&to=2026-12-25"
);
url_test!(
	url_date_today,
	c => c.date_today(parseapi::DateTodayOptions::default().to("2026-12-25")),
	"/date?to=2026-12-25"
);
url_test!(
	url_timezone_at,
	c => c.timezone_at(0.0, 180.0, parseapi::TimezoneAtOptions::default().at("2026-09-05T12:00:00Z")),
	"/timezone?lat=0&lon=180&at=2026-09-05T12%3A00%3A00Z"
);

#[tokio::test]
async fn date_and_timezone_preserve_nulls_and_accept_new_fields() {
	let server = TestServer::start(vec![
		(
			200,
			r#"{"date":"03/04/2026","valid":false,"year":null,"leap":null,"future_field":{"value":true}}"#,
		),
		(
			200,
			r#"{"latitude":0,"longitude":180,"timezone":null,"name":null,"abbreviation":null,"offset":null,"offset_minutes":null,"dst":null,"next_dst":null,"future_field":true}"#,
		),
	]);
	let client = server.client();
	let date = client.date("03/04/2026", None).await.expect("date");
	assert!(!date.valid);
	assert_eq!(date.year, None);
	assert_eq!(date.leap, None);
	let zone = client.timezone_at(0.0, 180.0, None).await.expect("zone");
	assert_eq!(zone.timezone, None);
	assert_eq!(zone.dst, None);
	assert_eq!(zone.offset_minutes, None);
}

#[tokio::test]
async fn does_not_forward_key_on_redirect() {
	let destination = TestServer::start(vec![(200, "{}")]);
	let server = TestServer::start_with_headers(vec![(
		302,
		"",
		format!("Location: {}/country/US\r\n", destination.base_url),
	)]);
	let err = server.client().country("US").await.unwrap_err();
	assert_eq!(err.status(), Some(302));
	assert!(
		destination.requests().is_empty(),
		"client followed redirect to another origin"
	);
}

url_test!(url_timezone_convert, c => c.timezone("America/New_York", TimezoneOptions::default().at("2026-09-05T15:00:00").to("Asia/Tokyo")), "/timezone/America%2FNew_York?at=2026-09-05T15%3A00%3A00&to=Asia%2FTokyo");
url_test!(url_weather_history, c => c.weather(1.0,2.0,WeatherOptions::default().deep(true).date("2026-09-01")), "/weather?lat=1&lon=2&deep=true&date=2026-09-01");
url_test!(url_address, c => c.address("123 Main St",AddressOptions::default().country("US").deep(true)), "/address/123%20Main%20St?country=US&deep=true");
url_test!(url_address_search, c => c.address_search("123 Main",AddressSearchOptions::default().country("US").postal("28202").city("Charlotte").state("NC").ip("8.8.8.8")), "/address?q=123+Main&country=US&postal=28202&city=Charlotte&state=NC&ip=8.8.8.8");
url_test!(url_company, c => c.company("123456789",CompanyOptions::default().country("FR").deep(true)), "/company/123456789?country=FR&deep=true");

#[tokio::test]
async fn metered_retry_defaults_and_explicit_override() {
	for product in [
		"carrier",
		"caller",
		"hlr",
		"email",
		"email_deep",
		"vat",
		"vat_deep",
		"address",
		"address_deep",
		"country",
	] {
		for explicit in [false, true] {
			let default_attempts = if [
				"carrier",
				"caller",
				"hlr",
				"email_deep",
				"vat_deep",
				"address_deep",
			]
			.contains(&product)
			{
				1
			} else {
				3
			};
			let attempts = if explicit { 2 } else { default_attempts };
			let server = TestServer::start_with_headers(
				(0..attempts)
					.map(|_| (503, "", "Retry-After: 0\r\n".into()))
					.collect(),
			);
			let mut builder = Client::builder()
				.api_key("test_key")
				.base_url(&server.base_url);
			if explicit {
				builder = builder.retries(1);
			}
			let c = builder.build().expect("client");
			let err = match product {
				"carrier" => c.carrier("555-0100", None).await.unwrap_err(),
				"caller" => c.caller("555-0100", None).await.unwrap_err(),
				"hlr" => c.hlr("555-0100", None).await.unwrap_err(),
				"email" => c.email("a@b.com", None).await.unwrap_err(),
				"email_deep" => c
					.email("a@b.com", EmailOptions::default().deep(true))
					.await
					.unwrap_err(),
				"vat" => c.vat("DE136695976", None).await.unwrap_err(),
				"address" => c.address("123 Main St", None).await.unwrap_err(),
				"address_deep" => c
					.address(
						"123 Main St",
						parseapi::AddressOptions::default().deep(true),
					)
					.await
					.unwrap_err(),
				"vat_deep" => c
					.vat("DE136695976", VatOptions::default().deep(true))
					.await
					.unwrap_err(),
				_ => c.country("US").await.unwrap_err(),
			};
			assert_eq!(err.status(), Some(503));
			assert_eq!(
				server.requests().len(),
				attempts,
				"{product}, explicit={explicit}"
			);
		}
	}
}

#[test]
fn complete_response_fields_preserve_unknown_values() {
	let company:Company=serde_json::from_str(r#"{"company":"x","registered":null,"active":null,"gst":null,"siege":null,"type":"company","kind":"LIMITED","deep":{},"future":true}"#).unwrap();
	assert_eq!(company.registered, None);
	assert_eq!(company.active, None);
	assert_eq!(company.gst, None);
	assert_eq!(company.siege, None);
	assert_eq!(company.r#type.as_deref(), Some("company"));
	assert_eq!(company.kind.as_deref(), Some("LIMITED"));
	assert!(company.deep.is_some());
	let weather:Weather=serde_json::from_str(r#"{"deep":{"hours":[{"feels_like":22,"wind_gust":4}],"days":[{"sunrise":"06:00"}],"air":{"aqi":10},"history":{"date":"2026-09-01","high":24}}}"#).unwrap();
	let deep = weather.deep.unwrap();
	assert_eq!(deep.hours[0].feels_like, Some(22.0));
	assert_eq!(deep.air.unwrap().aqi, Some(10.0));
	assert_eq!(deep.history.unwrap().high, Some(24.0));
	let zone:Timezone=serde_json::from_str(r#"{"timezone":"America/New_York","to":{"timezone":"Asia/Tokyo","at":"2026-09-06T04:00:00+09:00"}}"#).unwrap();
	assert_eq!(zone.to.unwrap().at, "2026-09-06T04:00:00+09:00");
}

#[test]
fn unresolved_company_and_address_echoes_stay_null() {
	let company: Company = serde_json::from_str(r#"{"company":null,"valid":false}"#).unwrap();
	assert_eq!(company.company, None);
	assert!(!company.valid);
	let address: Address = serde_json::from_str(r#"{"address":null,"valid":false}"#).unwrap();
	assert_eq!(address.address, None);
	assert!(!address.valid);
}

#[tokio::test]
async fn network_records_preserve_nulls_and_tolerate_future_fields() {
	let server = TestServer::start(vec![
		(
			200,
			r#"{"asn":4294967295,"name":null,"country":null,"country_name":null,"future":true}"#,
		),
		(
			200,
			r#"{"mac":"junk","valid":false,"vendor":null,"local":null,"multicast":null,"future":true}"#,
		),
		(
			200,
			r#"{"mac":"02:00:00:00:00:01","valid":true,"vendor":null,"local":true,"multicast":false}"#,
		),
	]);
	let client = server.client();
	let asn = client.asn("4294967295").await.unwrap();
	assert_eq!(asn.asn, u32::MAX);
	assert!(asn.name.is_none() && asn.country.is_none() && asn.country_name.is_none());
	let mac = client.mac("junk").await.unwrap();
	assert_eq!(mac.mac, "junk");
	assert!(!mac.valid);
	assert!(mac.vendor.is_none() && mac.local.is_none() && mac.multicast.is_none());
	let local = client.mac("02:00:00:00:00:01").await.unwrap();
	assert!(local.valid);
	assert_eq!(local.local, Some(true));
	assert_eq!(local.multicast, Some(false));
}
