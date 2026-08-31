//! Official parseAPI client for Rust. One key, minimal JSON, fast.
//!
//! ```no_run
//! # async fn run() -> Result<(), parseapi::Error> {
//! let parse = parseapi::Client::new("your-api-key")?;
//! let country = parse.country("US").await?;
//! # Ok(())
//! # }
//! ```
//!
//! <https://parseapi.com>

mod types;

pub use types::*;

use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};
use serde::de::DeserializeOwned;
use std::fmt;
use std::time::Duration;

const DEFAULT_BASE_URL: &str = "https://api.parseapi.com";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);
const DEFAULT_RETRIES: u32 = 2;
const RETRY_STATUS: [u16; 5] = [429, 500, 502, 503, 504];
const RETRY_AFTER_CAP_MS: f64 = 5000.0;
const USER_AGENT: &str = concat!("parseapi-rust/", env!("CARGO_PKG_VERSION"));

/// RFC 3986 unreserved characters pass through, everything else is encoded.
const SEGMENT: &AsciiSet = &NON_ALPHANUMERIC
	.remove(b'-')
	.remove(b'_')
	.remove(b'.')
	.remove(b'~');

/// Every failure from the client. API errors carry the response body,
/// transport failures wrap the underlying [`reqwest::Error`].
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
	/// Every non-2xx response from the API. Branch on `code`, never on `message`.
	Api {
		status: u16,
		code: String,
		message: String,
		docs: Option<String>,
		request_id: Option<String>,
	},
	/// Network failure after retries (DNS, timeout, connect).
	Transport(reqwest::Error),
	/// Client construction failure (missing key).
	Config(String),
}

impl Error {
	/// The API error code (`not_found`, `invalid_api_key`, ...) when this is an API error.
	pub fn code(&self) -> Option<&str> {
		match self {
			Error::Api { code, .. } => Some(code),
			_ => None,
		}
	}

	/// The HTTP status when this is an API error.
	pub fn status(&self) -> Option<u16> {
		match self {
			Error::Api { status, .. } => Some(*status),
			_ => None,
		}
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Error::Api { message, code, .. } => write!(f, "parseapi: {message} ({code})"),
			Error::Transport(err) => write!(f, "parseapi: {err}"),
			Error::Config(message) => write!(f, "parseapi: {message}"),
		}
	}
}

impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			Error::Transport(err) => Some(err),
			_ => None,
		}
	}
}

pub type Result<T> = std::result::Result<T, Error>;

/// Requests the nested deep object. Paid on most endpoints.
#[derive(Debug, Clone, Copy, Default)]
pub struct DeepOptions {
	pub deep: bool,
}

/// Narrows a district lookup.
#[derive(Debug, Clone, Default)]
pub struct DistrictOptions {
	pub country: Option<String>,
	pub state: Option<String>,
}

/// Narrows a city lookup.
#[derive(Debug, Clone, Default)]
pub struct CityOptions {
	pub country: Option<String>,
	pub state: Option<String>,
}

/// Narrows a city search.
#[derive(Debug, Clone, Default)]
pub struct CitySearchOptions {
	pub country: Option<String>,
	pub state: Option<String>,
	pub limit: Option<u32>,
}

/// Tunes a nearby search. Radius in the unit ("km" default, "mi").
#[derive(Debug, Clone, Default)]
pub struct PostalNearbyOptions {
	pub radius: Option<f64>,
	pub unit: Option<String>,
}

/// Tunes cities around a named anchor.
#[derive(Debug, Clone, Default)]
pub struct CityNearbyOptions {
	pub country: Option<String>,
	pub state: Option<String>,
	pub radius: Option<f64>,
	pub unit: Option<String>,
	pub limit: Option<u32>,
}

/// Narrows a phone lookup. Country is the default region for national
/// formats without a leading plus.
#[derive(Debug, Clone, Default)]
pub struct PhoneOptions {
	pub country: Option<String>,
	pub deep: bool,
}

/// Narrows a VAT lookup. Country fills a missing prefix. From is the
/// caller's own VAT number for a consultation identifier.
#[derive(Debug, Clone, Default)]
pub struct VatOptions {
	pub country: Option<String>,
	pub from: Option<String>,
	pub deep: bool,
}

/// Fills a missing country prefix on an IBAN.
#[derive(Debug, Clone, Default)]
pub struct IbanOptions {
	pub country: Option<String>,
}

/// Narrows a phone-family lookup. Country is the default region for
/// national formats without a leading plus.
#[derive(Debug, Clone, Default)]
pub struct CountryOptions {
	pub country: Option<String>,
}

/// Selects a past bulletin day and/or converts an amount on a currency pair.
#[derive(Debug, Clone, Default)]
pub struct CurrencyRateOptions {
	pub date: Option<String>,
	pub amount: Option<f64>,
}

/// Evaluates the zone at an optional ISO-8601 instant.
#[derive(Debug, Clone, Default)]
pub struct TimezoneOptions {
	pub at: Option<String>,
}

/// Selects a year. `None` means the current UTC year.
#[derive(Debug, Clone, Copy, Default)]
pub struct HolidayOptions {
	pub year: Option<i32>,
}

/// Caps the result count.
#[derive(Debug, Clone, Copy, Default)]
pub struct EmojiSearchOptions {
	pub limit: Option<u32>,
}

/// Configures a [`Client`].
#[derive(Debug, Default)]
pub struct Builder {
	api_key: Option<String>,
	base_url: Option<String>,
	timeout: Option<Duration>,
	retries: Option<u32>,
}

impl Builder {
	pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
		self.api_key = Some(api_key.into());
		self
	}

	/// Overrides `https://api.parseapi.com` (tests, canaries).
	pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
		self.base_url = Some(base_url.into());
		self
	}

	/// Per-attempt timeout. Default 10s.
	pub fn timeout(mut self, timeout: Duration) -> Self {
		self.timeout = Some(timeout);
		self
	}

	/// Retries after the first attempt on network errors / 429 / 5xx.
	/// Default 2, 0 disables.
	pub fn retries(mut self, retries: u32) -> Self {
		self.retries = Some(retries);
		self
	}

	pub fn build(self) -> Result<Client> {
		let api_key = self
			.api_key
			.or_else(|| std::env::var("PARSEAPI_KEY").ok())
			.filter(|key| !key.is_empty())
			.ok_or_else(|| Error::Config("missing API key, pass one or set PARSEAPI_KEY".into()))?;
		let base_url = self
			.base_url
			.or_else(|| std::env::var("PARSEAPI_BASE_URL").ok())
			.filter(|url| !url.is_empty())
			.unwrap_or_else(|| DEFAULT_BASE_URL.to_string());
		let http = reqwest::Client::builder()
			.timeout(self.timeout.unwrap_or(DEFAULT_TIMEOUT))
			.build()
			.map_err(Error::Transport)?;
		Ok(Client {
			api_key,
			base_url: base_url.trim_end_matches('/').to_string(),
			retries: self.retries.unwrap_or(DEFAULT_RETRIES),
			http,
		})
	}
}

/// A parseAPI client. Create one and share it, the connection stays warm.
#[derive(Debug, Clone)]
pub struct Client {
	api_key: String,
	base_url: String,
	retries: u32,
	http: reqwest::Client,
}

fn seg(value: &str) -> String {
	utf8_percent_encode(value, SEGMENT).to_string()
}

fn jitter() -> f64 {
	let nanos = std::time::SystemTime::now()
		.duration_since(std::time::UNIX_EPOCH)
		.unwrap_or_default()
		.subsec_nanos();
	f64::from(nanos % 1000) / 1000.0
}

fn retry_delay(attempt: u32, retry_after: Option<&str>) -> Duration {
	if let Some(seconds) = retry_after.and_then(|value| value.parse::<f64>().ok()) {
		if seconds >= 0.0 {
			return Duration::from_millis((seconds * 1000.0).min(RETRY_AFTER_CAP_MS) as u64);
		}
	}
	Duration::from_millis((jitter() * 250.0 * 2_f64.powi(attempt as i32)) as u64)
}

fn build_error(status: u16, body: &str) -> Error {
	let parsed: serde_json::Value = serde_json::from_str(body).unwrap_or_default();
	let field = |name: &str| parsed.get(name).and_then(|v| v.as_str()).map(str::to_owned);
	Error::Api {
		status,
		code: field("code").unwrap_or_else(|| "unknown_error".to_string()),
		message: field("message").unwrap_or_else(|| format!("Request failed with status {status}")),
		docs: field("docs"),
		request_id: field("request_id"),
	}
}

type Query = Vec<(&'static str, String)>;

fn push(query: &mut Query, name: &'static str, value: Option<String>) {
	if let Some(value) = value.filter(|v| !v.is_empty()) {
		query.push((name, value));
	}
}

fn push_deep(query: &mut Query, deep: bool) {
	if deep {
		query.push(("deep", "true".to_string()));
	}
}

impl Client {
	/// Creates a client with an explicit key.
	pub fn new(api_key: impl Into<String>) -> Result<Client> {
		Client::builder().api_key(api_key).build()
	}

	/// Creates a client from the `PARSEAPI_KEY` env var.
	pub fn from_env() -> Result<Client> {
		Client::builder().build()
	}

	pub fn builder() -> Builder {
		Builder::default()
	}

	async fn get<T: DeserializeOwned>(&self, path: &str, query: Query, ua: Option<&str>) -> Result<T> {
		let url = format!("{}{}", self.base_url, path);
		let mut attempt: u32 = 0;
		loop {
			let mut request = self
				.http
				.get(&url)
				.header("X-API-Key", &self.api_key)
				.header(reqwest::header::USER_AGENT, ua.unwrap_or(USER_AGENT));
			if !query.is_empty() {
				request = request.query(&query);
			}

			let response = match request.send().await {
				Ok(response) => response,
				Err(err) => {
					if attempt < self.retries {
						tokio::time::sleep(retry_delay(attempt, None)).await;
						attempt += 1;
						continue;
					}
					return Err(Error::Transport(err));
				}
			};

			let status = response.status();
			if status.is_success() {
				return response.json::<T>().await.map_err(Error::Transport);
			}

			if RETRY_STATUS.contains(&status.as_u16()) && attempt < self.retries {
				let retry_after = response
					.headers()
					.get("retry-after")
					.and_then(|value| value.to_str().ok())
					.map(str::to_owned);
				tokio::time::sleep(retry_delay(attempt, retry_after.as_deref())).await;
				attempt += 1;
				continue;
			}

			let body = response.text().await.unwrap_or_default();
			return Err(build_error(status.as_u16(), &body));
		}
	}

	/// Looks up an IP address.
	pub async fn ip(&self, ip: &str, opts: impl Into<Option<DeepOptions>>) -> Result<Ip> {
		let mut query = Query::new();
		push_deep(&mut query, opts.into().is_some_and(|o| o.deep));
		self.get(&format!("/ip/{}", seg(ip)), query, None).await
	}

	/// Looks up the caller's IP.
	pub async fn ip_self(&self, opts: impl Into<Option<DeepOptions>>) -> Result<Ip> {
		let mut query = Query::new();
		push_deep(&mut query, opts.into().is_some_and(|o| o.deep));
		self.get("/ip", query, None).await
	}

	/// Looks up a continent by code (NA, EU, ...).
	pub async fn continent(&self, code: &str) -> Result<Continent> {
		self.get(&format!("/continent/{}", seg(code)), Query::new(), None).await
	}

	/// Lists countries in a continent.
	pub async fn continent_countries(&self, code: &str) -> Result<ContinentCountries> {
		self.get(&format!("/continent/{}/countries", seg(code)), Query::new(), None).await
	}

	/// Looks up a country group by code (EU, SCHENGEN, NATO, ...).
	pub async fn bloc(&self, code: &str) -> Result<Bloc> {
		self.get(&format!("/bloc/{}", seg(code)), Query::new(), None).await
	}

	/// Lists the current members of a bloc.
	pub async fn bloc_countries(&self, code: &str) -> Result<BlocCountries> {
		self.get(&format!("/bloc/{}/countries", seg(code)), Query::new(), None).await
	}

	/// Looks up a country by ISO code.
	pub async fn country(&self, code: &str) -> Result<Country> {
		self.get(&format!("/country/{}", seg(code)), Query::new(), None).await
	}

	/// Lists states in a country.
	pub async fn country_states(&self, code: &str) -> Result<CountryStates> {
		self.get(&format!("/country/{}/states", seg(code)), Query::new(), None).await
	}

	/// Looks up a state or province by code or name. Country is optional when
	/// the code or name is unique. Pass "" to omit it.
	pub async fn state(&self, code: &str, country: &str) -> Result<State> {
		let mut query = Query::new();
		push(&mut query, "country", Some(country.to_string()));
		self.get(&format!("/state/{}", seg(code)), query, None).await
	}

	/// Lists districts under a state.
	pub async fn state_districts(&self, code: &str, country: &str) -> Result<StateDistricts> {
		let mut query = Query::new();
		push(&mut query, "country", Some(country.to_string()));
		self.get(&format!("/state/{}/districts", seg(code)), query, None).await
	}

	/// Looks up a district (ADM2) by code or name.
	pub async fn district(&self, code: &str, opts: impl Into<Option<DistrictOptions>>) -> Result<District> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "country", opts.country);
		push(&mut query, "state", opts.state);
		self.get(&format!("/district/{}", seg(code)), query, None).await
	}

	/// Looks up a city by name.
	pub async fn city(&self, name: &str, opts: impl Into<Option<CityOptions>>) -> Result<City> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "country", opts.country);
		push(&mut query, "state", opts.state);
		self.get(&format!("/city/{}", seg(name)), query, None).await
	}

	/// Fetches a city by its minted parse id (`city_…`).
	pub async fn city_id(&self, id: &str) -> Result<City> {
		self.get(&format!("/city/id/{}", seg(id)), Query::new(), None).await
	}

	/// Searches cities by name prefix.
	pub async fn city_search(&self, q: &str, opts: impl Into<Option<CitySearchOptions>>) -> Result<CitySearch> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "q", Some(q.to_string()));
		push(&mut query, "country", opts.country);
		push(&mut query, "state", opts.state);
		push(&mut query, "limit", opts.limit.map(|l| l.to_string()));
		self.get("/city", query, None).await
	}

	/// Finds the nearest city to a point.
	pub async fn city_nearest(&self, lat: f64, lon: f64) -> Result<CityNearest> {
		let mut query = Query::new();
		push(&mut query, "lat", Some(lat.to_string()));
		push(&mut query, "lon", Some(lon.to_string()));
		self.get("/city", query, None).await
	}

	/// Lists cities around a named city, nearest first.
	pub async fn city_nearby(&self, name: &str, opts: impl Into<Option<CityNearbyOptions>>) -> Result<CityNearby> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "country", opts.country);
		push(&mut query, "state", opts.state);
		push(&mut query, "radius", opts.radius.map(|r| r.to_string()));
		push(&mut query, "unit", opts.unit);
		push(&mut query, "limit", opts.limit.map(|l| l.to_string()));
		self.get(&format!("/city/{}/nearby", seg(name)), query, None).await
	}

	/// Looks up a postal or ZIP code. Country is optional when the code is unique.
	/// Pass "" to omit it.
	pub async fn postal(&self, code: &str, country: &str) -> Result<Postal> {
		let mut query = Query::new();
		push(&mut query, "country", Some(country.to_string()));
		self.get(&format!("/postal/{}", seg(code)), query, None).await
	}

	/// Lists postal codes near one.
	pub async fn postal_nearby(
		&self,
		code: &str,
		country: &str,
		opts: impl Into<Option<PostalNearbyOptions>>,
	) -> Result<PostalNearby> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "country", if country.is_empty() { None } else { Some(country.to_string()) });
		push(&mut query, "radius", opts.radius.map(|r| r.to_string()));
		push(&mut query, "unit", opts.unit);
		self.get(&format!("/postal/{}/nearby", seg(code)), query, None).await
	}

	/// Measures the distance between two postal codes.
	pub async fn postal_distance(&self, from: &str, to: &str, country: &str) -> Result<PostalDistance> {
		let mut query = Query::new();
		push(&mut query, "country", Some(country.to_string()));
		self.get(&format!("/postal/{}/distance/{}", seg(from), seg(to)), query, None).await
	}

	/// Validates an email address.
	pub async fn email(&self, email: &str, opts: impl Into<Option<DeepOptions>>) -> Result<Email> {
		let mut query = Query::new();
		push_deep(&mut query, opts.into().is_some_and(|o| o.deep));
		self.get(&format!("/email/{}", seg(email)), query, None).await
	}

	/// Checksums a VAT number. Deep asks the live EU registry.
	pub async fn vat(&self, number: &str, opts: impl Into<Option<VatOptions>>) -> Result<Vat> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "country", opts.country);
		push(&mut query, "from", opts.from);
		push_deep(&mut query, opts.deep);
		self.get(&format!("/vat/{}", seg(number)), query, None).await
	}

	/// Checksums an IBAN and returns the bank, branch, and account identifiers sitting inside it.
	pub async fn iban(&self, iban: &str, opts: impl Into<Option<IbanOptions>>) -> Result<Iban> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "country", opts.country);
		self.get(&format!("/iban/{}", seg(iban)), query, None).await
	}

	/// Validates and formats a phone number.
	pub async fn phone(&self, number: &str, opts: impl Into<Option<PhoneOptions>>) -> Result<Phone> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "country", opts.country);
		push_deep(&mut query, opts.deep);
		self.get(&format!("/phone/{}", seg(number)), query, None).await
	}

	/// Looks up the current carrier serving a phone number. Metered.
	pub async fn carrier(&self, number: &str, opts: impl Into<Option<CountryOptions>>) -> Result<Carrier> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "country", opts.country);
		self.get(&format!("/carrier/{}", seg(number)), query, None).await
	}

	/// Looks up the caller ID name (CNAM) for a NANP phone number. Metered.
	pub async fn caller(&self, number: &str, opts: impl Into<Option<CountryOptions>>) -> Result<Caller> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "country", opts.country);
		self.get(&format!("/caller/{}", seg(number)), query, None).await
	}

	/// Checks live network status for a phone number worldwide. Metered.
	pub async fn hlr(&self, number: &str, opts: impl Into<Option<CountryOptions>>) -> Result<Hlr> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "country", opts.country);
		self.get(&format!("/hlr/{}", seg(number)), query, None).await
	}

	/// Checks if a domain is available to register.
	pub async fn domain(&self, domain: &str, opts: impl Into<Option<DeepOptions>>) -> Result<Domain> {
		let mut query = Query::new();
		push_deep(&mut query, opts.into().is_some_and(|o| o.deep));
		self.get(&format!("/domain/{}", seg(domain)), query, None).await
	}

	/// Returns MX records for a domain.
	pub async fn mx(&self, domain: &str) -> Result<Mx> {
		self.get(&format!("/mx/{}", seg(domain)), Query::new(), None).await
	}

	/// Parses a User-Agent string.
	pub async fn useragent(&self, ua: &str, opts: impl Into<Option<DeepOptions>>) -> Result<Useragent> {
		let mut query = Query::new();
		push_deep(&mut query, opts.into().is_some_and(|o| o.deep));
		self.get("/useragent", query, Some(ua)).await
	}

	/// Looks up a currency by ISO 4217 code.
	pub async fn currency(&self, code: &str) -> Result<Currency> {
		self.get(&format!("/currency/{}", seg(code)), Query::new(), None).await
	}

	/// Looks up a language by BCP 47 shortest code or ISO 639-3.
	pub async fn language(&self, code: &str) -> Result<Language> {
		self.get(&format!("/language/{}", seg(code)), Query::new(), None).await
	}

	/// Parses a person's name into its parts.
	pub async fn name(&self, name: &str) -> Result<Name> {
		self.get(&format!("/name/{}", seg(name)), Query::new(), None).await
	}

	/// Returns the daily official reference rate for a currency pair.
	pub async fn currency_rate(
		&self,
		base: &str,
		quote: &str,
		opts: impl Into<Option<CurrencyRateOptions>>,
	) -> Result<CurrencyRate> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "date", opts.date);
		push(&mut query, "amount", opts.amount.map(|a| a.to_string()));
		self.get(&format!("/currency/{}/{}", seg(base), seg(quote)), query, None).await
	}

	/// Looks up an IANA timezone.
	pub async fn timezone(&self, id: &str, opts: impl Into<Option<TimezoneOptions>>) -> Result<Timezone> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "at", opts.at);
		self.get(&format!("/timezone/{}", seg(id)), query, None).await
	}

	/// Lists public holidays for a country and year.
	pub async fn holiday(&self, country: &str, opts: impl Into<Option<HolidayOptions>>) -> Result<HolidayYear> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "year", opts.year.map(|y| y.to_string()));
		self.get(&format!("/holiday/{}", seg(country)), query, None).await
	}

	/// Checks one date (YYYY-MM-DD). `holiday` is `None` when the date is
	/// not a holiday.
	pub async fn holiday_date(&self, country: &str, date: &str) -> Result<HolidayDate> {
		self.get(&format!("/holiday/{}/{}", seg(country), seg(date)), Query::new(), None).await
	}

	/// Returns the elevation at a point.
	pub async fn elevation(&self, lat: f64, lon: f64) -> Result<Elevation> {
		let mut query = Query::new();
		push(&mut query, "lat", Some(lat.to_string()));
		push(&mut query, "lon", Some(lon.to_string()));
		self.get("/elevation", query, None).await
	}

	/// Returns everything at a point: elevation plus the admin place.
	pub async fn point(&self, lat: f64, lon: f64, opts: impl Into<Option<DeepOptions>>) -> Result<Point> {
		let mut query = Query::new();
		push(&mut query, "lat", Some(lat.to_string()));
		push(&mut query, "lon", Some(lon.to_string()));
		push_deep(&mut query, opts.into().is_some_and(|o| o.deep));
		self.get("/point", query, None).await
	}

	/// Returns current conditions at a point from the nearest official
	/// station. Every measurement ships metric and imperial side by side.
	pub async fn weather(&self, lat: f64, lon: f64, opts: impl Into<Option<DeepOptions>>) -> Result<Weather> {
		let mut query = Query::new();
		push(&mut query, "lat", Some(lat.to_string()));
		push(&mut query, "lon", Some(lon.to_string()));
		push_deep(&mut query, opts.into().is_some_and(|o| o.deep));
		self.get("/weather", query, None).await
	}

	/// Resolves an emoji by character, shortcode, or name.
	pub async fn emoji(&self, emoji: &str) -> Result<Emoji> {
		self.get(&format!("/emoji/{}", seg(emoji)), Query::new(), None).await
	}

	/// Searches emoji by keyword.
	pub async fn emoji_search(&self, q: &str, opts: impl Into<Option<EmojiSearchOptions>>) -> Result<EmojiSearch> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "q", Some(q.to_string()));
		push(&mut query, "limit", opts.limit.map(|l| l.to_string()));
		self.get("/emoji", query, None).await
	}
}
