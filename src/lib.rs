//! Official ParseAPI client for Rust. One key, minimal JSON, fast.
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
/// transport failures expose their underlying error through `source()`.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
	/// Every non-2xx response from the API. Branch on `code`, never on `message`.
	#[non_exhaustive]
	Api {
		status: u16,
		code: String,
		message: String,
		docs: Option<String>,
		request_id: Option<String>,
	},
	/// Network failure after retries (DNS, timeout, connect).
	Transport(Box<dyn std::error::Error + Send + Sync>),
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
			Error::Transport(err) => Some(err.as_ref()),
			_ => None,
		}
	}
}

pub type Result<T> = std::result::Result<T, Error>;

/// Options for measurement conversion.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct MeasureOptions {
	pub to: Option<String>,
	pub locale: Option<String>,
	pub system: Option<String>,
}

impl MeasureOptions {
	pub fn to(mut self, value: impl Into<String>) -> Self {
		self.to = Some(value.into());
		self
	}
	pub fn locale(mut self, value: impl Into<String>) -> Self {
		self.locale = Some(value.into());
		self
	}
	pub fn system(mut self, value: impl Into<String>) -> Self {
		self.system = Some(value.into());
		self
	}
}

/// Options for reviewed unit discovery.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct MeasureUnitsOptions {
	pub query: Option<String>,
	pub r#type: Option<String>,
	pub unit: Option<String>,
}

impl MeasureUnitsOptions {
	pub fn query(mut self, value: impl Into<String>) -> Self {
		self.query = Some(value.into());
		self
	}
	pub fn r#type(mut self, value: impl Into<String>) -> Self {
		self.r#type = Some(value.into());
		self
	}
	pub fn unit(mut self, value: impl Into<String>) -> Self {
		self.unit = Some(value.into());
		self
	}
}

/// Configures `ip`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct IpOptions {
	pub deep: bool,
}

impl IpOptions {
	/// Sets the `deep` query option.
	pub fn deep(mut self, value: bool) -> Self {
		self.deep = value;
		self
	}
}

/// Configures `ip_self`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct IpSelfOptions {
	pub deep: bool,
}

impl IpSelfOptions {
	/// Sets the `deep` query option.
	pub fn deep(mut self, value: bool) -> Self {
		self.deep = value;
		self
	}
}

/// Configures `state`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct StateOptions {
	pub country: Option<String>,
	pub deep: bool,
}

impl StateOptions {
	/// Sets the `country` query option.
	pub fn country(mut self, value: impl Into<String>) -> Self {
		self.country = Some(value.into());
		self
	}
	/// Requests optional detail from the same lookup.
	pub fn deep(mut self, value: bool) -> Self { self.deep = value; self }
}

/// Configures `state_districts`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct StateDistrictsOptions {
	pub country: Option<String>,
	pub deep: bool,
}

impl StateDistrictsOptions {
	/// Sets the `country` query option.
	pub fn country(mut self, value: impl Into<String>) -> Self {
		self.country = Some(value.into());
		self
	}
	/// Requests optional detail from the same lookup.
	pub fn deep(mut self, value: bool) -> Self { self.deep = value; self }
}

/// Configures `district`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct DistrictOptions {
	pub country: Option<String>,
	pub state: Option<String>,
	pub deep: bool,
}

impl DistrictOptions {
	/// Sets the `country` query option.
	pub fn country(mut self, value: impl Into<String>) -> Self {
		self.country = Some(value.into());
		self
	}
	/// Sets the `state` query option.
	pub fn state(mut self, value: impl Into<String>) -> Self {
		self.state = Some(value.into());
		self
	}
	/// Requests optional detail from the same lookup.
	pub fn deep(mut self, value: bool) -> Self { self.deep = value; self }
}

/// Configures `city`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct CityOptions {
	pub country: Option<String>,
	pub state: Option<String>,
	pub deep: bool,
}

impl CityOptions {
	/// Sets the `country` query option.
	pub fn country(mut self, value: impl Into<String>) -> Self {
		self.country = Some(value.into());
		self
	}
	/// Sets the `state` query option.
	pub fn state(mut self, value: impl Into<String>) -> Self {
		self.state = Some(value.into());
		self
	}
	/// Requests optional detail from the same lookup.
	pub fn deep(mut self, value: bool) -> Self { self.deep = value; self }
}

/// Configures `city_search`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct CitySearchOptions {
	pub country: Option<String>,
	pub state: Option<String>,
	pub limit: Option<u32>,
	pub deep: bool,
}

impl CitySearchOptions {
	/// Sets the `country` query option.
	pub fn country(mut self, value: impl Into<String>) -> Self {
		self.country = Some(value.into());
		self
	}
	/// Sets the `state` query option.
	pub fn state(mut self, value: impl Into<String>) -> Self {
		self.state = Some(value.into());
		self
	}
	/// Sets the `limit` query option.
	pub fn limit(mut self, value: u32) -> Self {
		self.limit = Some(value);
		self
	}
	/// Requests optional detail from the same lookup.
	pub fn deep(mut self, value: bool) -> Self { self.deep = value; self }
}

/// Configures `city_nearby`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct CityNearbyOptions {
	pub country: Option<String>,
	pub state: Option<String>,
	pub radius: Option<f64>,
	pub unit: Option<String>,
	pub limit: Option<u32>,
	pub deep: bool,
}

impl CityNearbyOptions {
	/// Sets the `country` query option.
	pub fn country(mut self, value: impl Into<String>) -> Self {
		self.country = Some(value.into());
		self
	}
	/// Sets the `state` query option.
	pub fn state(mut self, value: impl Into<String>) -> Self {
		self.state = Some(value.into());
		self
	}
	/// Sets the `radius` query option.
	pub fn radius(mut self, value: f64) -> Self {
		self.radius = Some(value);
		self
	}
	/// Sets the `unit` query option.
	pub fn unit(mut self, value: impl Into<String>) -> Self {
		self.unit = Some(value.into());
		self
	}
	/// Sets the `limit` query option.
	pub fn limit(mut self, value: u32) -> Self {
		self.limit = Some(value);
		self
	}
	/// Requests optional detail from the same lookup.
	pub fn deep(mut self, value: bool) -> Self { self.deep = value; self }
}

/// Configures `postal`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct PostalOptions {
	pub country: Option<String>,
	pub deep: bool,
}

impl PostalOptions {
	/// Sets the `country` query option.
	pub fn country(mut self, value: impl Into<String>) -> Self {
		self.country = Some(value.into());
		self
	}
	/// Requests optional detail from the same lookup.
	pub fn deep(mut self, value: bool) -> Self { self.deep = value; self }
}

/// Configures `postal_nearby`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct PostalNearbyOptions {
	pub country: Option<String>,
	pub radius: Option<f64>,
	pub unit: Option<String>,
	pub deep: bool,
}

impl PostalNearbyOptions {
	/// Sets the `country` query option.
	pub fn country(mut self, value: impl Into<String>) -> Self {
		self.country = Some(value.into());
		self
	}
	/// Sets the `radius` query option.
	pub fn radius(mut self, value: f64) -> Self {
		self.radius = Some(value);
		self
	}
	/// Sets the `unit` query option.
	pub fn unit(mut self, value: impl Into<String>) -> Self {
		self.unit = Some(value.into());
		self
	}
	/// Requests optional detail from the same lookup.
	pub fn deep(mut self, value: bool) -> Self { self.deep = value; self }
}

/// Configures `postal_distance`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct PostalDistanceOptions {
	pub country: Option<String>,
	pub deep: bool,
}

impl PostalDistanceOptions {
	/// Sets the `country` query option.
	pub fn country(mut self, value: impl Into<String>) -> Self {
		self.country = Some(value.into());
		self
	}
	/// Requests optional detail from the same lookup.
	pub fn deep(mut self, value: bool) -> Self { self.deep = value; self }
}

/// Configures `email`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct EmailOptions {
	pub deep: bool,
}

impl EmailOptions {
	/// Sets the `deep` query option.
	pub fn deep(mut self, value: bool) -> Self {
		self.deep = value;
		self
	}
}

/// Configures `vat`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct VatOptions {
	pub country: Option<String>,
	pub from: Option<String>,
	pub deep: bool,
}

impl VatOptions {
	/// Sets the `country` query option.
	pub fn country(mut self, value: impl Into<String>) -> Self {
		self.country = Some(value.into());
		self
	}
	/// Sets the `from` query option.
	pub fn from(mut self, value: impl Into<String>) -> Self {
		self.from = Some(value.into());
		self
	}
	/// Sets the `deep` query option.
	pub fn deep(mut self, value: bool) -> Self {
		self.deep = value;
		self
	}
}

/// Configures `bin`. Deep requests an empty object on every plan.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct BinOptions {
	pub deep: bool,
}

impl BinOptions {
	pub fn deep(mut self, value: bool) -> Self {
		self.deep = value;
		self
	}
}

/// Configures `iban`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct IbanOptions {
	pub country: Option<String>,
	pub deep: bool,
}

impl IbanOptions {
	/// Sets the `country` query option.
	pub fn country(mut self, value: impl Into<String>) -> Self {
		self.country = Some(value.into());
		self
	}
	/// Requests optional detail from the same lookup.
	pub fn deep(mut self, value: bool) -> Self { self.deep = value; self }
}

/// Configures `name_with_options`. Country is an ISO2 gender context.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct NameOptions {
	pub country: Option<String>,
	pub deep: bool,
}

impl NameOptions {
	/// Sets the country context without asserting nationality.
	pub fn country(mut self, value: impl Into<String>) -> Self {
		self.country = Some(value.into());
		self
	}
	/// Requests optional detail from the same lookup.
	pub fn deep(mut self, value: bool) -> Self { self.deep = value; self }
}

/// Configures `npi`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct NpiOptions {
	pub deep: bool,
}

impl NpiOptions {
	/// Sets the `deep` query option.
	pub fn deep(mut self, value: bool) -> Self {
		self.deep = value;
		self
	}
}

/// Configures `phone`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct PhoneOptions {
	pub country: Option<String>,
	pub deep: bool,
}

impl PhoneOptions {
	/// Sets the `country` query option.
	pub fn country(mut self, value: impl Into<String>) -> Self {
		self.country = Some(value.into());
		self
	}
	/// Sets the `deep` query option.
	pub fn deep(mut self, value: bool) -> Self {
		self.deep = value;
		self
	}
}

/// Configures `carrier`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct CarrierOptions {
	pub country: Option<String>,
	pub deep: bool,
}

impl CarrierOptions {
	/// Sets the `country` query option.
	pub fn country(mut self, value: impl Into<String>) -> Self {
		self.country = Some(value.into());
		self
	}
	/// Requests optional detail from the same lookup.
	pub fn deep(mut self, value: bool) -> Self { self.deep = value; self }
}

/// Configures `caller`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct CallerOptions {
	pub country: Option<String>,
}

impl CallerOptions {
	/// Sets the `country` query option.
	pub fn country(mut self, value: impl Into<String>) -> Self {
		self.country = Some(value.into());
		self
	}
}

/// Configures `hlr`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct HlrOptions {
	pub country: Option<String>,
	pub deep: bool,
}

impl HlrOptions {
	/// Sets the `country` query option.
	pub fn country(mut self, value: impl Into<String>) -> Self {
		self.country = Some(value.into());
		self
	}
	/// Requests optional detail from the same lookup.
	pub fn deep(mut self, value: bool) -> Self { self.deep = value; self }
}

/// Configures `dns`. Omit type to check all ten supported record types.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct DnsOptions {
	pub r#type: Option<String>,
}

impl DnsOptions {
	/// Selects the DNS question. Responses may include its CNAME chain.
	pub fn r#type(mut self, value: impl Into<String>) -> Self {
		self.r#type = Some(value.into());
		self
	}
}

/// Configures `domain`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct DomainOptions {
	pub deep: bool,
}

impl DomainOptions {
	/// Sets the `deep` query option.
	pub fn deep(mut self, value: bool) -> Self {
		self.deep = value;
		self
	}
}

/// Configures `useragent`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct UseragentOptions {
	pub deep: bool,
}

impl UseragentOptions {
	/// Sets the `deep` query option.
	pub fn deep(mut self, value: bool) -> Self {
		self.deep = value;
		self
	}
}

/// Configures `vin`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct VinOptions {
	pub deep: bool,
}

impl VinOptions {
	/// Sets the `deep` query option.
	pub fn deep(mut self, value: bool) -> Self {
		self.deep = value;
		self
	}
}

/// Configures `tariff`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct TariffOptions {
	/// Add units and the special and other schedule columns on paid plans.
	pub deep: bool,
	/// ISO 3166-1 alpha-2 origin. With paid deep, resolves country-specific measures. Optional for schedule detail.
	pub origin: Option<String>,
}

impl TariffOptions {
	/// Add units and the special and other schedule columns on paid plans.
	pub fn deep(mut self, value: bool) -> Self {
		self.deep = value;
		self
	}
	/// ISO 3166-1 alpha-2 origin. With paid deep, resolves country-specific measures. Optional for schedule detail.
	pub fn origin(mut self, value: impl Into<String>) -> Self {
		self.origin = Some(value.into());
		self
	}
}

/// Configures `currency_rate`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct CurrencyRateOptions {
	pub date: Option<String>,
	pub amount: Option<f64>,
}

impl CurrencyRateOptions {
	/// Sets the `date` query option.
	pub fn date(mut self, value: impl Into<String>) -> Self {
		self.date = Some(value.into());
		self
	}
	/// Sets the `amount` query option.
	pub fn amount(mut self, value: f64) -> Self {
		self.amount = Some(value);
		self
	}
}

/// Configures `time`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct TimeOptions {
	pub at: Option<String>,
	pub to: Option<String>,
	pub deep: bool,
}

impl TimeOptions {
	/// Sets the `at` query option.
	pub fn at(mut self, value: impl Into<String>) -> Self {
		self.at = Some(value.into());
		self
	}
	/// Sets the `to` query option.
	pub fn to(mut self, value: impl Into<String>) -> Self {
		self.to = Some(value.into());
		self
	}
	/// Requests optional detail from the same lookup.
	pub fn deep(mut self, value: bool) -> Self { self.deep = value; self }
}

/// Configures `time_at`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct TimeAtOptions {
	pub at: Option<String>,
	pub to: Option<String>,
	pub deep: bool,
}

impl TimeAtOptions {
	/// Sets the destination IANA timezone.
	pub fn to(mut self, value: impl Into<String>) -> Self {
		self.to = Some(value.into());
		self
	}

	/// Sets the `at` query option.
	pub fn at(mut self, value: impl Into<String>) -> Self {
		self.at = Some(value.into());
		self
	}
	/// Requests optional detail from the same lookup.
	pub fn deep(mut self, value: bool) -> Self { self.deep = value; self }
}

/// Configures `timezone`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct TimezoneOptions {
	pub at: Option<String>,
	pub to: Option<String>,
	pub deep: bool,
}

impl TimezoneOptions {
	/// Sets the `at` query option.
	pub fn at(mut self, value: impl Into<String>) -> Self {
		self.at = Some(value.into());
		self
	}
	/// Sets the `to` query option.
	pub fn to(mut self, value: impl Into<String>) -> Self {
		self.to = Some(value.into());
		self
	}
	/// Requests optional detail from the same lookup.
	pub fn deep(mut self, value: bool) -> Self { self.deep = value; self }
}

/// Configures `timezone_at`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct TimezoneAtOptions {
	pub at: Option<String>,
	pub deep: bool,
}

impl TimezoneAtOptions {
	/// Sets the `at` query option.
	pub fn at(mut self, value: impl Into<String>) -> Self {
		self.at = Some(value.into());
		self
	}
	/// Requests optional detail from the same lookup.
	pub fn deep(mut self, value: bool) -> Self { self.deep = value; self }
}

/// Configures `date`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct DateOptions {
	pub format: Option<String>,
	pub to: Option<String>,
	pub deep: bool,
}

impl DateOptions {
	/// Sets the `format` query option.
	pub fn format(mut self, value: impl Into<String>) -> Self {
		self.format = Some(value.into());
		self
	}
	/// Sets the `to` query option.
	pub fn to(mut self, value: impl Into<String>) -> Self {
		self.to = Some(value.into());
		self
	}
	/// Requests optional detail from the same lookup.
	pub fn deep(mut self, value: bool) -> Self { self.deep = value; self }
}

/// Configures `date_today`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct DateTodayOptions {
	pub to: Option<String>,
	pub deep: bool,
}

impl DateTodayOptions {
	/// Sets the `to` query option.
	pub fn to(mut self, value: impl Into<String>) -> Self {
		self.to = Some(value.into());
		self
	}
	/// Requests optional detail from the same lookup.
	pub fn deep(mut self, value: bool) -> Self { self.deep = value; self }
}

/// Configures `holiday`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct HolidayOptions {
	pub year: Option<i32>,
}

impl HolidayOptions {
	/// Sets the `year` query option.
	pub fn year(mut self, value: i32) -> Self {
		self.year = Some(value);
		self
	}
}

/// Configures `point`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct PointOptions {
	/// Add terrain and compact nearest-city context on every plan. The timezone ID stays in core.
	pub deep: bool,
}

impl PointOptions {
	/// Add terrain and compact nearest-city context on every plan. The timezone ID stays in core.
	pub fn deep(mut self, value: bool) -> Self {
		self.deep = value;
		self
	}
}

/// Configures `weather`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct WeatherOptions {
	/// Add specialist current measurements, forecasts and related detail on paid plans.
	pub deep: bool,
	/// Past UTC day (YYYY-MM-DD). Requires paid deep and adds deep.history alongside current conditions.
	pub date: Option<String>,
}

impl WeatherOptions {
	/// Add specialist current measurements, forecasts and related detail on paid plans.
	pub fn deep(mut self, value: bool) -> Self {
		self.deep = value;
		self
	}
	/// Past UTC day (YYYY-MM-DD). Requires paid deep and adds deep.history alongside current conditions.
	pub fn date(mut self, value: impl Into<String>) -> Self {
		self.date = Some(value.into());
		self
	}
}

/// Configures `naics_search`. Limit defaults to 10 and accepts 1-50.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct NaicsSearchOptions {
	pub limit: Option<u32>,
	pub deep: bool,
}

impl NaicsSearchOptions {
	pub fn limit(mut self, value: u32) -> Self {
		self.limit = Some(value);
		self
	}
	/// Requests optional detail from the same lookup.
	pub fn deep(mut self, value: bool) -> Self { self.deep = value; self }
}

/// Configures `emoji_search`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct EmojiSearchOptions {
	pub limit: Option<u32>,
	pub deep: bool,
}

impl EmojiSearchOptions {
	/// Sets the `limit` query option.
	pub fn limit(mut self, value: u32) -> Self {
		self.limit = Some(value);
		self
	}
	/// Requests optional detail from the same lookup.
	pub fn deep(mut self, value: bool) -> Self { self.deep = value; self }
}

/// Configures `address`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct AddressOptions {
	pub country: Option<String>,
	pub deep: bool,
}

impl AddressOptions {
	/// Sets the `country` query option.
	pub fn country(mut self, value: impl Into<String>) -> Self {
		self.country = Some(value.into());
		self
	}
	/// Sets the `deep` query option.
	pub fn deep(mut self, value: bool) -> Self {
		self.deep = value;
		self
	}
}

/// Configures `address_search`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct AddressSearchOptions {
	pub country: Option<String>,
	pub postal: Option<String>,
	pub city: Option<String>,
	pub state: Option<String>,
	pub ip: Option<String>,
}

impl AddressSearchOptions {
	/// Sets the `country` query option.
	pub fn country(mut self, value: impl Into<String>) -> Self {
		self.country = Some(value.into());
		self
	}
	/// Sets the `postal` query option.
	pub fn postal(mut self, value: impl Into<String>) -> Self {
		self.postal = Some(value.into());
		self
	}
	/// Sets the `city` query option.
	pub fn city(mut self, value: impl Into<String>) -> Self {
		self.city = Some(value.into());
		self
	}
	/// Sets the `state` query option.
	pub fn state(mut self, value: impl Into<String>) -> Self {
		self.state = Some(value.into());
		self
	}
	/// Sets the `ip` query option.
	pub fn ip(mut self, value: impl Into<String>) -> Self {
		self.ip = Some(value.into());
		self
	}
}

/// Configures `company`. Omitted fields use API defaults.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct CompanyOptions {
	pub country: Option<String>,
	pub deep: bool,
}

impl CompanyOptions {
	/// Sets the `country` query option.
	pub fn country(mut self, value: impl Into<String>) -> Self {
		self.country = Some(value.into());
		self
	}
	/// Sets the `deep` query option.
	pub fn deep(mut self, value: bool) -> Self {
		self.deep = value;
		self
	}
}


/// Options for `country`. Deep reveals the same question in more detail.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct CountryOptions { pub deep: bool }
impl CountryOptions { pub fn deep(mut self, value: bool) -> Self { self.deep = value; self } }


/// Options for `city_id`. Deep reveals the same question in more detail.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct CityIdOptions { pub deep: bool }
impl CityIdOptions { pub fn deep(mut self, value: bool) -> Self { self.deep = value; self } }


/// Options for `city_nearest`. Deep reveals the same question in more detail.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct CityNearestOptions { pub deep: bool }
impl CityNearestOptions { pub fn deep(mut self, value: bool) -> Self { self.deep = value; self } }


/// Options for `naics`. Deep reveals the same question in more detail.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct NaicsOptions { pub deep: bool }
impl NaicsOptions { pub fn deep(mut self, value: bool) -> Self { self.deep = value; self } }


/// Options for `currency`. Deep reveals the same question in more detail.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct CurrencyOptions { pub deep: bool }
impl CurrencyOptions { pub fn deep(mut self, value: bool) -> Self { self.deep = value; self } }


/// Options for `language`. Deep reveals the same question in more detail.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct LanguageOptions { pub deep: bool }
impl LanguageOptions { pub fn deep(mut self, value: bool) -> Self { self.deep = value; self } }


/// Options for `emoji`. Deep reveals the same question in more detail.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct EmojiOptions { pub deep: bool }
impl EmojiOptions { pub fn deep(mut self, value: bool) -> Self { self.deep = value; self } }

/// Configures a [`Client`].
#[derive(Default)]
pub struct Builder {
	api_key: Option<String>,
	base_url: Option<String>,
	timeout: Option<Duration>,
	retries: Option<u32>,
}

impl fmt::Debug for Builder {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Builder")
			.field("api_key", &self.api_key.as_ref().map(|_| "[REDACTED]"))
			.field("timeout", &self.timeout)
			.field("retries", &self.retries)
			.finish_non_exhaustive()
	}
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

	/// Overrides retries for every operation. Ordinary lookups default to two
	/// retries and metered operations default to none. Additional attempts can
	/// be billed. Zero disables all automatic retries.
	pub fn retries(mut self, retries: u32) -> Self {
		self.retries = Some(retries);
		self
	}

	pub fn build(self) -> Result<Client> {
		let api_key = self
			.api_key
			.filter(|key| !key.is_empty())
			.or_else(|| std::env::var("PARSEAPI_KEY").ok())
			.filter(|key| !key.is_empty())
			.ok_or_else(|| Error::Config("missing API key, pass one or set PARSEAPI_KEY".into()))?;
		let base_url = self
			.base_url
			.or_else(|| std::env::var("PARSEAPI_BASE_URL").ok())
			.filter(|url| !url.is_empty())
			.unwrap_or_else(|| DEFAULT_BASE_URL.to_string());
		let http = reqwest::Client::builder()
			.redirect(reqwest::redirect::Policy::none())
			.timeout(self.timeout.unwrap_or(DEFAULT_TIMEOUT))
			.build()
			.map_err(|err| Error::Transport(Box::new(err)))?;
		Ok(Client {
			api_key,
			base_url: base_url.trim_end_matches('/').to_string(),
			retries: self.retries.unwrap_or(DEFAULT_RETRIES),
			retries_explicit: self.retries.is_some(),
			http,
		})
	}
}

/// A ParseAPI client. Create one and share it, the connection stays warm.
#[derive(Clone)]
pub struct Client {
	api_key: String,
	base_url: String,
	retries: u32,
	retries_explicit: bool,
	http: reqwest::Client,
}

impl fmt::Debug for Client {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Client")
			.field("api_key", &"[REDACTED]")
			.field("retries", &self.retries)
			.finish_non_exhaustive()
	}
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
	if let Some(at) = retry_after.and_then(|value| httpdate::parse_http_date(value).ok()) {
		return at
			.duration_since(std::time::SystemTime::now())
			.unwrap_or_default()
			.min(Duration::from_secs(5));
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

fn metered_request(path: &str, query: &Query) -> bool {
	if ["carrier", "caller", "hlr", "litigator", "reassigned"]
		.iter()
		.any(|product| path.starts_with(&format!("/{product}/")))
	{
		return true;
	}
	(path.starts_with("/email/") || path.starts_with("/vat/") || path.starts_with("/address/"))
		&& query
			.iter()
			.any(|(name, value)| *name == "deep" && value == "true")
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

	async fn get<T: DeserializeOwned>(
		&self,
		path: &str,
		query: Query,
		ua: Option<&str>,
	) -> Result<T> {
		let retries = if !self.retries_explicit && metered_request(path, &query) {
			0
		} else {
			self.retries
		};
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
					if attempt < retries {
						tokio::time::sleep(retry_delay(attempt, None)).await;
						attempt += 1;
						continue;
					}
					return Err(Error::Transport(Box::new(err)));
				}
			};

			let status = response.status();
			if status.is_success() {
				return response
					.json::<T>()
					.await
					.map_err(|err| Error::Transport(Box::new(err)));
			}

			if RETRY_STATUS.contains(&status.as_u16()) && attempt < retries {
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

	/// Look up an IP. Deep enrichment is included with a paid plan, without a separate check meter.
	pub async fn ip(&self, ip: &str, opts: impl Into<Option<IpOptions>>) -> Result<Ip> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push_deep(&mut query, opts.deep);
		self.get(&format!("/ip/{}", seg(ip)), query, None).await
	}

	/// Look up the public IP making this request. On a server, this is the server's IP.
	pub async fn ip_self(&self, opts: impl Into<Option<IpSelfOptions>>) -> Result<Ip> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push_deep(&mut query, opts.deep);
		self.get("/ip", query, None).await
	}

	/// Calls `/continent/{code}`.
	pub async fn continent(&self, code: &str) -> Result<Continent> {
		self.get(&format!("/continent/{}", seg(code)), Query::new(), None)
			.await
	}

	/// Calls `/continent/{code}/countries`.
	pub async fn continent_countries(&self, code: &str) -> Result<ContinentCountries> {
		self.get(
			&format!("/continent/{}/countries", seg(code)),
			Query::new(),
			None,
		)
		.await
	}

	/// Calls `/bloc/{code}`.
	pub async fn bloc(&self, code: &str) -> Result<Bloc> {
		self.get(&format!("/bloc/{}", seg(code)), Query::new(), None)
			.await
	}

	/// Calls `/bloc/{code}/countries`.
	pub async fn bloc_countries(&self, code: &str) -> Result<BlocCountries> {
		self.get(
			&format!("/bloc/{}/countries", seg(code)),
			Query::new(),
			None,
		)
		.await
	}

	/// Calls `/country/{code}`.
	pub async fn country(&self, code: &str) -> Result<Country> {
		self.country_with_options(code, None).await
	}

	/// Calls the same operation with optional detail.
	pub async fn country_with_options(&self, code: &str, opts: impl Into<Option<CountryOptions>>) -> Result<Country> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push_deep(&mut query, opts.deep);
		self.get(&format!("/country/{}", seg(code)), query, None)
			.await
	}

	/// Calls `/country/{code}/states`.
	pub async fn country_states(&self, code: &str) -> Result<CountryStates> {
		self.get(
			&format!("/country/{}/states", seg(code)),
			Query::new(),
			None,
		)
		.await
	}

	/// Calls `/state/{code}`.
	pub async fn state(&self, code: &str, opts: impl Into<Option<StateOptions>>) -> Result<State> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "country", opts.country);
		push_deep(&mut query, opts.deep);
		self.get(&format!("/state/{}", seg(code)), query, None)
			.await
	}

	/// Calls `/state/{code}/districts`.
	pub async fn state_districts(
		&self,
		code: &str,
		opts: impl Into<Option<StateDistrictsOptions>>,
	) -> Result<StateDistricts> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "country", opts.country);
		push_deep(&mut query, opts.deep);
		self.get(&format!("/state/{}/districts", seg(code)), query, None)
			.await
	}

	/// Calls `/district/{code}`.
	pub async fn district(
		&self,
		code: &str,
		opts: impl Into<Option<DistrictOptions>>,
	) -> Result<District> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "country", opts.country);
		push(&mut query, "state", opts.state);
		push_deep(&mut query, opts.deep);
		self.get(&format!("/district/{}", seg(code)), query, None)
			.await
	}

	/// Calls `/city/{name}`.
	pub async fn city(&self, name: &str, opts: impl Into<Option<CityOptions>>) -> Result<City> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "country", opts.country);
		push(&mut query, "state", opts.state);
		push_deep(&mut query, opts.deep);
		self.get(&format!("/city/{}", seg(name)), query, None).await
	}

	/// Calls `/city/id/{id}`.
	pub async fn city_id(&self, id: &str) -> Result<City> {
		self.city_id_with_options(id, None).await
	}

	/// Calls the same operation with optional detail.
	pub async fn city_id_with_options(&self, id: &str, opts: impl Into<Option<CityIdOptions>>) -> Result<City> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push_deep(&mut query, opts.deep);
		self.get(&format!("/city/id/{}", seg(id)), query, None)
			.await
	}

	/// Calls `/city`.
	pub async fn city_search(
		&self,
		query: &str,
		opts: impl Into<Option<CitySearchOptions>>,
	) -> Result<CitySearch> {
		let opts = opts.into().unwrap_or_default();
		let mut params = Query::new();
		push(&mut params, "q", Some(query.to_string()));
		push(&mut params, "country", opts.country);
		push(&mut params, "state", opts.state);
		push(
			&mut params,
			"limit",
			opts.limit.map(|value| value.to_string()),
		);
		push_deep(&mut params, opts.deep);
		self.get("/city", params, None).await
	}

	/// Calls `/city`.
	pub async fn city_nearest(&self, lat: f64, lon: f64) -> Result<CityNearest> {
		self.city_nearest_with_options(lat, lon, None).await
	}

	/// Calls the same operation with optional detail.
	pub async fn city_nearest_with_options(&self, lat: f64, lon: f64, opts: impl Into<Option<CityNearestOptions>>) -> Result<CityNearest> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "lat", Some(lat.to_string()));
		push(&mut query, "lon", Some(lon.to_string()));
		push_deep(&mut query, opts.deep);
		self.get("/city", query, None).await
	}

	/// Calls `/city/{name}/nearby`.
	pub async fn city_nearby(
		&self,
		name: &str,
		opts: impl Into<Option<CityNearbyOptions>>,
	) -> Result<CityNearby> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "country", opts.country);
		push(&mut query, "state", opts.state);
		push(
			&mut query,
			"radius",
			opts.radius.map(|value| value.to_string()),
		);
		push(&mut query, "unit", opts.unit);
		push(
			&mut query,
			"limit",
			opts.limit.map(|value| value.to_string()),
		);
		push_deep(&mut query, opts.deep);
		self.get(&format!("/city/{}/nearby", seg(name)), query, None)
			.await
	}

	/// Look up a postal area. Pass country when known. Check nullable coordinates before another
	/// location lookup.
	pub async fn postal(
		&self,
		code: &str,
		opts: impl Into<Option<PostalOptions>>,
	) -> Result<Postal> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "country", opts.country);
		push_deep(&mut query, opts.deep);
		self.get(&format!("/postal/{}", seg(code)), query, None)
			.await
	}

	/// Calls `/postal/{code}/nearby`.
	pub async fn postal_nearby(
		&self,
		code: &str,
		opts: impl Into<Option<PostalNearbyOptions>>,
	) -> Result<PostalNearby> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "country", opts.country);
		push(
			&mut query,
			"radius",
			opts.radius.map(|value| value.to_string()),
		);
		push(&mut query, "unit", opts.unit);
		push_deep(&mut query, opts.deep);
		self.get(&format!("/postal/{}/nearby", seg(code)), query, None)
			.await
	}

	/// Calls `/postal/{code}/distance/{other}`.
	pub async fn postal_distance(
		&self,
		code: &str,
		other: &str,
		opts: impl Into<Option<PostalDistanceOptions>>,
	) -> Result<PostalDistance> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "country", opts.country);
		push_deep(&mut query, opts.deep);
		self.get(
			&format!("/postal/{}/distance/{}", seg(code), seg(other)),
			query,
			None,
		)
		.await
	}

	/// Parse an email and check its format and domain. Deep explicitly requests a metered
	/// deliverability check. Deep checks use one attempt by default. An explicit retry count can
	/// repeat paid usage.
	pub async fn email(&self, email: &str, opts: impl Into<Option<EmailOptions>>) -> Result<Email> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push_deep(&mut query, opts.deep);
		self.get(&format!("/email/{}", seg(email)), query, None)
			.await
	}

	/// Check VAT format and checksum. Deep requests a metered registry check where supported. Deep
	/// checks use one attempt by default. Supply your own VAT number for a consultation reference
	/// when supported.
	pub async fn vat(&self, number: &str, opts: impl Into<Option<VatOptions>>) -> Result<Vat> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "country", opts.country);
		push(&mut query, "from", opts.from);
		push_deep(&mut query, opts.deep);
		self.get(&format!("/vat/{}", seg(number)), query, None)
			.await
	}

	/// Calls `/iban/{iban}`.
	pub async fn iban(&self, iban: &str, opts: impl Into<Option<IbanOptions>>) -> Result<Iban> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "country", opts.country);
		push_deep(&mut query, opts.deep);
		self.get(&format!("/iban/{}", seg(iban)), query, None).await
	}

	/// Calls `/npi/{npi}`.
	pub async fn npi(&self, npi: &str, opts: impl Into<Option<NpiOptions>>) -> Result<Npi> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push_deep(&mut query, opts.deep);
		self.get(&format!("/npi/{}", seg(npi)), query, None).await
	}

	/// Parse a phone number and its formats. Pass country for national numbers when needed. Deep
	/// reveals numbering-plan location and timezone on every plan. Carrier, caller, and HLR are separate metered lookups.
	pub async fn phone(
		&self,
		number: &str,
		opts: impl Into<Option<PhoneOptions>>,
	) -> Result<Phone> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "country", opts.country);
		push_deep(&mut query, opts.deep);
		self.get(&format!("/phone/{}", seg(number)), query, None)
			.await
	}

	/// Request a metered carrier lookup. No automatic retries by default.
	pub async fn carrier(
		&self,
		number: &str,
		opts: impl Into<Option<CarrierOptions>>,
	) -> Result<Carrier> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "country", opts.country);
		push_deep(&mut query, opts.deep);
		self.get(&format!("/carrier/{}", seg(number)), query, None)
			.await
	}

	/// Request a metered caller-name lookup for a NANP number. No automatic retries by default.
	pub async fn caller(
		&self,
		number: &str,
		opts: impl Into<Option<CallerOptions>>,
	) -> Result<Caller> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "country", opts.country);
		self.get(&format!("/caller/{}", seg(number)), query, None)
			.await
	}

	/// Look up phone status at the last check. Live means assigned and connected means reachable at
	/// that check. Cached results may be returned. Null means unconfirmed. Deep adds network
	/// diagnostics within the same metered lookup. No automatic retries by default.
	pub async fn hlr(&self, number: &str, opts: impl Into<Option<HlrOptions>>) -> Result<Hlr> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "country", opts.country);
		push_deep(&mut query, opts.deep);
		self.get(&format!("/hlr/{}", seg(number)), query, None)
			.await
	}

	/// Check whether a domain is registered. Deep adds registration dates, registrar, status and DNSSEC on paid plans.
	pub async fn domain(
		&self,
		domain: &str,
		opts: impl Into<Option<DomainOptions>>,
	) -> Result<Domain> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push_deep(&mut query, opts.deep);
		self.get(&format!("/domain/{}", seg(domain)), query, None)
			.await
	}

	/// Calls `/asn/{asn}`.
	pub async fn asn(&self, asn: &str) -> Result<Asn> {
		self.get(&format!("/asn/{}", seg(asn)), Query::new(), None)
			.await
	}

	/// Calls `/mac/{mac}`.
	pub async fn mac(&self, mac: &str) -> Result<Mac> {
		self.get(&format!("/mac/{}", seg(mac)), Query::new(), None)
			.await
	}

	/// Look up a 6-11 digit card prefix. Preserve leading zeros in the string.
	pub async fn bin(&self, bin: &str, opts: impl Into<Option<BinOptions>>) -> Result<Bin> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push_deep(&mut query, opts.deep);
		self.get(&format!("/bin/{}", seg(bin)), query, None).await
	}

	/// Calls `/swift/{code}`. Valid checks syntax, not payment reachability.
	pub async fn swift(&self, code: &str) -> Result<SwiftCode> {
		self.get(&format!("/swift/{}", seg(code)), Query::new(), None)
			.await
	}

	/// Parse or convert a measurement. Amount is a decimal string. Without to, use the
	/// type's canonical unit. Locale and system (us or imperial) resolve explicit ambiguity.
	pub async fn measure(&self, measure: &str, opts: impl Into<Option<MeasureOptions>>) -> Result<Measure> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "to", opts.to);
		push(&mut query, "locale", opts.locale);
		push(&mut query, "system", opts.system);
		self.get(&format!("/measure/{}", seg(measure)), query, None).await
	}

	/// Discover reviewed units. unit filters compatible conversion targets.
	pub async fn measure_units(&self, opts: impl Into<Option<MeasureUnitsOptions>>) -> Result<MeasureUnits> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "q", opts.query);
		push(&mut query, "type", opts.r#type);
		push(&mut query, "unit", opts.unit);
		self.get("/measure/units", query, None).await
	}

	/// Published DNS records with TTLs. Values retain DNS presentation syntax.
	/// Pooled on every plan. Omit type to check all ten supported record types.
	pub async fn dns(&self, domain: &str, opts: impl Into<Option<DnsOptions>>) -> Result<Dns> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "type", opts.r#type);
		self.get(&format!("/dns/{}", seg(domain)), query, None).await
	}

	/// Calls `/mx/{domain}`.
	pub async fn mx(&self, domain: &str) -> Result<Mx> {
		self.get(&format!("/mx/{}", seg(domain)), Query::new(), None)
			.await
	}

	/// Calls `/useragent`.
	pub async fn useragent(
		&self,
		ua: &str,
		opts: impl Into<Option<UseragentOptions>>,
	) -> Result<Useragent> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push_deep(&mut query, opts.deep);
		self.get("/useragent", query, Some(ua)).await
	}

	/// Calls `/vin/{vin}`.
	pub async fn vin(&self, vin: &str, opts: impl Into<Option<VinOptions>>) -> Result<Vin> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push_deep(&mut query, opts.deep);
		self.get(&format!("/vin/{}", seg(vin)), query, None).await
	}

	/// Looks up a US NAICS 2022 code and its hierarchy.
	pub async fn naics(&self, code: &str) -> Result<Naics> {
		self.naics_with_options(code, None).await
	}

	/// Calls the same operation with optional detail.
	pub async fn naics_with_options(&self, code: &str, opts: impl Into<Option<NaicsOptions>>) -> Result<Naics> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push_deep(&mut query, opts.deep);
		self.get(&format!("/naics/{}", seg(code)), query, None).await
	}

	/// Searches US NAICS 2022 industry names and activity terms.
	pub async fn naics_search(&self, query: &str, opts: impl Into<Option<NaicsSearchOptions>>) -> Result<NaicsSearch> {
		let opts = opts.into().unwrap_or_default();
		let mut params = Query::new();
		params.push(("q", query.into()));
		push(&mut params, "limit", opts.limit.map(|value| value.to_string()));
		push_deep(&mut params, opts.deep);
		self.get("/naics", params, None).await
	}

	/// Look up the general US duty schedule line. Paid deep adds units and the special and other
	/// schedule columns. Add origin with deep to resolve country-specific measures. Without origin,
	/// schedule detail remains available and origin-dependent fields are null. A null effective rate
	/// is not a zero rate.
	pub async fn tariff(
		&self,
		code: &str,
		opts: impl Into<Option<TariffOptions>>,
	) -> Result<Tariff> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push_deep(&mut query, opts.deep);
		push(&mut query, "origin", opts.origin);
		self.get(&format!("/tariff/{}", seg(code)), query, None)
			.await
	}

	/// Calls `/tariff`.
	pub async fn tariff_search(&self, query: &str) -> Result<TariffSearch> {
		let mut params = Query::new();
		push(&mut params, "q", Some(query.to_string()));
		self.get("/tariff", params, None).await
	}

	/// Calls `/currency/{code}`.
	pub async fn currency(&self, code: &str) -> Result<Currency> {
		self.currency_with_options(code, None).await
	}

	/// Calls the same operation with optional detail.
	pub async fn currency_with_options(&self, code: &str, opts: impl Into<Option<CurrencyOptions>>) -> Result<Currency> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push_deep(&mut query, opts.deep);
		self.get(&format!("/currency/{}", seg(code)), query, None)
			.await
	}

	/// Calls `/language/{code}`.
	pub async fn language(&self, code: &str) -> Result<Language> {
		self.language_with_options(code, None).await
	}

	/// Calls the same operation with optional detail.
	pub async fn language_with_options(&self, code: &str, opts: impl Into<Option<LanguageOptions>>) -> Result<Language> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push_deep(&mut query, opts.deep);
		self.get(&format!("/language/{}", seg(code)), query, None)
			.await
	}

	/// Calls `/name/{name}`.
	pub async fn name(&self, name: &str) -> Result<Name> {
		self.name_with_options(name, None).await
	}

	/// Parses a name with an optional ISO2 country context for gender.
	pub async fn name_with_options(&self, name: &str, opts: impl Into<Option<NameOptions>>) -> Result<Name> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "country", opts.country);
		push_deep(&mut query, opts.deep);
		self.get(&format!("/name/{}", seg(name)), query, None).await
	}

	/// Calls `/currency/{base}/{quote}`.
	pub async fn currency_rate(
		&self,
		base: &str,
		quote: &str,
		opts: impl Into<Option<CurrencyRateOptions>>,
	) -> Result<CurrencyRate> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "date", opts.date);
		push(
			&mut query,
			"amount",
			opts.amount.map(|value| value.to_string()),
		);
		self.get(
			&format!("/currency/{}/{}", seg(base), seg(quote)),
			query,
			None,
		)
		.await
	}

	/// Calls `/time/{timezone}`.
	pub async fn time(
		&self,
		timezone: &str,
		opts: impl Into<Option<TimeOptions>>,
	) -> Result<Time> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "at", opts.at);
		push(&mut query, "to", opts.to);
		let path = if timezone.is_empty() { "/time".to_string() } else { format!("/time/{}", seg(timezone)) };
		push_deep(&mut query, opts.deep);
		self.get(&path, query, None)
			.await
	}

	/// Calls `/time`.
	pub async fn time_at(
		&self,
		lat: f64,
		lon: f64,
		opts: impl Into<Option<TimeAtOptions>>,
	) -> Result<Time> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "lat", Some(lat.to_string()));
		push(&mut query, "lon", Some(lon.to_string()));
		push(&mut query, "at", opts.at);
		push(&mut query, "to", opts.to);
		push_deep(&mut query, opts.deep);
		self.get("/time", query, None).await
	}

	/// Calls `/timezone/{timezone}`.
	pub async fn timezone(
		&self,
		timezone: &str,
		opts: impl Into<Option<TimezoneOptions>>,
	) -> Result<Timezone> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "at", opts.at);
		push(&mut query, "to", opts.to);
		push_deep(&mut query, opts.deep);
		self.get(&format!("/timezone/{}", seg(timezone)), query, None)
			.await
	}

	/// Calls `/timezone`.
	pub async fn timezone_at(
		&self,
		lat: f64,
		lon: f64,
		opts: impl Into<Option<TimezoneAtOptions>>,
	) -> Result<Timezone> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "lat", Some(lat.to_string()));
		push(&mut query, "lon", Some(lon.to_string()));
		push(&mut query, "at", opts.at);
		push_deep(&mut query, opts.deep);
		self.get("/timezone", query, None).await
	}

	/// Calls `/date/{date}`.
	pub async fn date(&self, date: &str, opts: impl Into<Option<DateOptions>>) -> Result<DateInfo> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "format", opts.format);
		push(&mut query, "to", opts.to);
		push_deep(&mut query, opts.deep);
		self.get(&format!("/date/{}", seg(date)), query, None).await
	}

	/// Calls `/date`.
	pub async fn date_today(&self, opts: impl Into<Option<DateTodayOptions>>) -> Result<DateInfo> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "to", opts.to);
		push_deep(&mut query, opts.deep);
		self.get("/date", query, None).await
	}

	/// Calls `/holiday/{country}`.
	pub async fn holiday(
		&self,
		country: &str,
		opts: impl Into<Option<HolidayOptions>>,
	) -> Result<HolidayYear> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "year", opts.year.map(|value| value.to_string()));
		self.get(&format!("/holiday/{}", seg(country)), query, None)
			.await
	}

	/// Calls `/holiday/{country}/{date}`.
	pub async fn holiday_date(&self, country: &str, date: &str) -> Result<HolidayDate> {
		self.get(
			&format!("/holiday/{}/{}", seg(country), seg(date)),
			Query::new(),
			None,
		)
		.await
	}

	/// Calls `/elevation`.
	pub async fn elevation(&self, lat: f64, lon: f64) -> Result<Elevation> {
		let mut query = Query::new();
		push(&mut query, "lat", Some(lat.to_string()));
		push(&mut query, "lon", Some(lon.to_string()));
		self.get("/elevation", query, None).await
	}

	/// Resolve the country, state, district and timezone at coordinates. Deep adds terrain and compact
	/// nearest-city context on every plan. The timezone ID stays in core. The nearest city is null
	/// when none is within 200 km.
	pub async fn point(
		&self,
		lat: f64,
		lon: f64,
		opts: impl Into<Option<PointOptions>>,
	) -> Result<Point> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "lat", Some(lat.to_string()));
		push(&mut query, "lon", Some(lon.to_string()));
		push_deep(&mut query, opts.deep);
		self.get("/point", query, None).await
	}

	/// Get current conditions in metric and imperial units. Paid deep adds specialist current
	/// measurements, forecasts and related detail. With deep, date selects a past UTC day (YYYY-MM-DD)
	/// in deep.history alongside current conditions. Date alone does not request history.
	pub async fn weather(
		&self,
		lat: f64,
		lon: f64,
		opts: impl Into<Option<WeatherOptions>>,
	) -> Result<Weather> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "lat", Some(lat.to_string()));
		push(&mut query, "lon", Some(lon.to_string()));
		push_deep(&mut query, opts.deep);
		push(&mut query, "date", opts.date);
		self.get("/weather", query, None).await
	}

	/// Calls `/emoji/{emoji}`.
	pub async fn emoji(&self, emoji: &str) -> Result<Emoji> {
		self.emoji_with_options(emoji, None).await
	}

	/// Calls the same operation with optional detail.
	pub async fn emoji_with_options(&self, emoji: &str, opts: impl Into<Option<EmojiOptions>>) -> Result<Emoji> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push_deep(&mut query, opts.deep);
		self.get(&format!("/emoji/{}", seg(emoji)), query, None)
			.await
	}

	/// Calls `/emoji`.
	pub async fn emoji_search(
		&self,
		query: &str,
		opts: impl Into<Option<EmojiSearchOptions>>,
	) -> Result<EmojiSearch> {
		let opts = opts.into().unwrap_or_default();
		let mut params = Query::new();
		push(&mut params, "q", Some(query.to_string()));
		push(
			&mut params,
			"limit",
			opts.limit.map(|value| value.to_string()),
		);
		push_deep(&mut params, opts.deep);
		self.get("/emoji", params, None).await
	}

	/// Calls `/address/{address}`.
	pub async fn address(
		&self,
		address: &str,
		opts: impl Into<Option<AddressOptions>>,
	) -> Result<Address> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "country", opts.country);
		push_deep(&mut query, opts.deep);
		self.get(&format!("/address/{}", seg(address)), query, None)
			.await
	}

	/// Find address suggestions using the context supplied. Prefer postal, or city and state, from the
	/// form; ip is an optional end-user locality hint for server-side calls. An empty result has
	/// reason more_input, missing_context or no_matches. Suggestions have reason null. Operational
	/// failures are errors.
	pub async fn address_search(
		&self,
		query: &str,
		opts: impl Into<Option<AddressSearchOptions>>,
	) -> Result<AddressSearch> {
		let opts = opts.into().unwrap_or_default();
		let mut params = Query::new();
		push(&mut params, "q", Some(query.to_string()));
		push(&mut params, "country", opts.country);
		push(&mut params, "postal", opts.postal);
		push(&mut params, "city", opts.city);
		push(&mut params, "state", opts.state);
		push(&mut params, "ip", opts.ip);
		self.get("/address", params, None).await
	}

	/// Calls `/company/{number}`.
	pub async fn company(
		&self,
		number: &str,
		opts: impl Into<Option<CompanyOptions>>,
	) -> Result<Company> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push(&mut query, "country", opts.country);
		push_deep(&mut query, opts.deep);
		self.get(&format!("/company/{}", seg(number)), query, None)
			.await
	}
}

#[cfg(test)]
mod transport_tests {
	use super::*;
	#[test]
	fn retry_after_accepts_http_dates() {
		let future =
			httpdate::fmt_http_date(std::time::SystemTime::now() + Duration::from_secs(3600));
		let past =
			httpdate::fmt_http_date(std::time::SystemTime::now() - Duration::from_secs(3600));
		assert_eq!(retry_delay(0, Some(&future)), Duration::from_secs(5));
		assert_eq!(retry_delay(0, Some(&past)), Duration::ZERO);
		assert_eq!(retry_delay(0, Some("0")), Duration::ZERO);
	}
}
