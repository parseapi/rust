use parseapi::{
	CityNearbyOptions, CityOptions, CitySearchOptions, Client, CountryOptions, CurrencyRateOptions,
	DeepOptions, Error, HolidayOptions, PhoneOptions, PostalNearbyOptions, VatOptions,
};
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
		let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
		let base_url = format!("http://{}", listener.local_addr().expect("addr"));
		let requests: Arc<Mutex<Vec<Recorded>>> = Arc::new(Mutex::new(Vec::new()));
		let recorded = Arc::clone(&requests);

		std::thread::spawn(move || {
			for (status, body) in responses {
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
				let target = request_line.split(' ').nth(1).unwrap_or_default().to_string();
				let mut headers = HashMap::new();
				for line in lines {
					if let Some((name, value)) = line.split_once(':') {
						headers.insert(name.trim().to_lowercase(), value.trim().to_string());
					}
				}
				recorded.lock().unwrap().push(Recorded { target, headers });

				let response = format!(
					"HTTP/1.1 {status} X\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
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
	c => c.ip("8.8.8.8", DeepOptions { deep: true }),
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
url_test!(url_state, c => c.state("NC", "US"), "/state/NC?country=US");
url_test!(url_state_name, c => c.state("colorado", ""), "/state/colorado");
url_test!(
	url_state_districts,
	c => c.state_districts("NC", "US"),
	"/state/NC/districts?country=US"
);
url_test!(url_district, c => c.district("37081", None), "/district/37081");
url_test!(
	url_city,
	c => c.city(
		"charlotte",
		CityOptions { state: Some("NC".into()), ..Default::default() }
	),
	"/city/charlotte?state=NC"
);
url_test!(url_city_id, c => c.city_id("city_mb8mbqrkz8zb"), "/city/id/city_mb8mbqrkz8zb");
url_test!(
	url_city_search,
	c => c.city_search(
		"char",
		CitySearchOptions { country: Some("US".into()), limit: Some(10), ..Default::default() }
	),
	"/city?q=char&country=US&limit=10"
);
url_test!(
	url_city_nearest,
	c => c.city_nearest(35.2271, -80.8431),
	"/city?lat=35.2271&lon=-80.8431"
);
url_test!(
	url_city_nearby,
	c => c.city_nearby(
		"denver",
		CityNearbyOptions { radius: Some(8.0), unit: Some("mi".into()), limit: Some(3), ..Default::default() }
	),
	"/city/denver/nearby?radius=8&unit=mi&limit=3"
);
url_test!(url_postal, c => c.postal("28202", "US"), "/postal/28202?country=US");
url_test!(url_postal_bare, c => c.postal("SW1A 1AA", ""), "/postal/SW1A%201AA");
url_test!(
	url_postal_nearby,
	c => c.postal_nearby(
		"28202",
		"US",
		PostalNearbyOptions { radius: Some(40.0), unit: Some("km".into()) }
	),
	"/postal/28202/nearby?country=US&radius=40&unit=km"
);
url_test!(
	url_postal_distance,
	c => c.postal_distance("28202", "10001", "US"),
	"/postal/28202/distance/10001?country=US"
);
url_test!(url_email, c => c.email("a@b.com", None), "/email/a%40b.com");
url_test!(url_vat, c => c.vat("DE136695976", None), "/vat/DE136695976");
url_test!(
	url_vat_from_deep,
	c => c.vat(
		"DE136695976",
		VatOptions { from: Some("IE6388047V".into()), deep: true, ..Default::default() }
	),
	"/vat/DE136695976?from=IE6388047V&deep=true"
);
url_test!(
	url_phone_encodes_plus,
	c => c.phone("+14155552671", PhoneOptions { deep: true, ..Default::default() }),
	"/phone/%2B14155552671?deep=true"
);
url_test!(
	url_carrier_encodes_plus,
	c => c.carrier("+14155552671", None),
	"/carrier/%2B14155552671"
);
url_test!(
	url_caller_with_country,
	c => c.caller("4155552671", CountryOptions { country: Some("US".into()) }),
	"/caller/4155552671?country=US"
);
url_test!(url_hlr, c => c.hlr("+447712345678", None), "/hlr/%2B447712345678");
url_test!(url_domain, c => c.domain("example.com", None), "/domain/example.com");
url_test!(url_mx, c => c.mx("example.com"), "/mx/example.com");
url_test!(url_useragent, c => c.useragent("TestUA/1.0", None), "/useragent");
url_test!(url_currency, c => c.currency("USD"), "/currency/USD");
url_test!(
	url_currency_rate,
	c => c.currency_rate("USD", "EUR", None),
	"/currency/USD/EUR"
);
url_test!(
	url_currency_rate_date_amount,
	c => c.currency_rate(
		"USD",
		"JPY",
		CurrencyRateOptions {
			date: Some("2026-08-28".into()),
			amount: Some(100.0),
		}
	),
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
	c => c.holiday("US", HolidayOptions { year: Some(1955) }),
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
	c => c.point(36.0726, -79.792, DeepOptions { deep: true }),
	"/point?lat=36.0726&lon=-79.792&deep=true"
);
url_test!(
	url_weather,
	c => c.weather(40.7128, -74.006, DeepOptions { deep: true }),
	"/weather?lat=40.7128&lon=-74.006&deep=true"
);
url_test!(url_emoji, c => c.emoji("rocket"), "/emoji/rocket");
url_test!(
	url_emoji_search,
	c => c.emoji_search("fire", parseapi::EmojiSearchOptions { limit: Some(20) }),
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
	assert_eq!(server.requests()[0].headers["user-agent"], "Mozilla/5.0 (Test)");
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
		let client = Client::builder().base_url(&server.base_url).retries(0).build();
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
		Error::Api { status, code, message, docs, request_id } => {
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
	let server = TestServer::start(vec![(429, rate_limited), (429, rate_limited), (429, rate_limited)]);
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
