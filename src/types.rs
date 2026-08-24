//! Response types for the parseAPI public API. Shapes are append-only
//! upstream, so these only ever grow. Nullable fields are `Option`.
//! Deep objects follow the triad: `None` when not requested, empty when
//! requested but locked, populated when unlocked.

use serde::Deserialize;

/// The API sends explicit nulls for some array fields. Treat null like missing.
fn null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
	D: serde::Deserializer<'de>,
	T: Default + serde::Deserialize<'de>,
{
	Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct IpDeep {
	pub state: Option<String>,
	pub city: Option<String>,
	pub registry: Option<String>,
	pub datacenter: Option<bool>,
	pub relay: Option<bool>,
	pub tor: Option<bool>,
	pub vpn: Option<bool>,
	pub provider: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Ip {
	pub ip: String,
	pub country: Option<String>,
	pub country_name: Option<String>,
	pub continent: Option<String>,
	pub asn: Option<String>,
	pub asn_name: Option<String>,
	pub deep: Option<IpDeep>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Continent {
	pub continent: String,
	pub name: String,
	pub region: String,
	pub subregion: String,
	pub population: Option<i64>,
	pub area: Option<f64>,
	pub emoji: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct ContinentCountryItem {
	pub country: String,
	pub name: String,
	pub emoji: Option<String>,
	pub calling_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct ContinentCountries {
	pub continent: String,
	#[serde(default, deserialize_with = "null_default")]
	pub countries: Vec<ContinentCountryItem>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Country {
	pub country: String,
	pub iso3: String,
	pub numeric: i32,
	pub name: String,
	pub full_name: Option<String>,
	pub local_name: Option<String>,
	pub demonym: Option<String>,
	pub capital: Option<String>,
	pub capital_lat: Option<f64>,
	pub capital_lon: Option<f64>,
	pub continent: String,
	pub region: Option<String>,
	pub subregion: Option<String>,
	pub population: Option<i64>,
	pub area: Option<f64>,
	pub currency: Option<String>,
	pub currency_name: Option<String>,
	pub currency_symbol: Option<String>,
	pub tld: Option<String>,
	pub calling_code: Option<String>,
	pub emoji: Option<String>,
	#[serde(default, deserialize_with = "null_default")]
	pub languages: Vec<String>,
	#[serde(default, deserialize_with = "null_default")]
	pub borders: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CountryStateItem {
	pub state: String,
	pub name: String,
	#[serde(rename = "type")]
	pub kind: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CountryStates {
	pub country: String,
	#[serde(default, deserialize_with = "null_default")]
	pub states: Vec<CountryStateItem>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct State {
	pub state: String,
	pub name: String,
	pub local_name: Option<String>,
	#[serde(rename = "type")]
	pub kind: Option<String>,
	pub country: String,
	pub country_name: Option<String>,
	pub latitude: Option<f64>,
	pub longitude: Option<f64>,
	pub population: Option<i64>,
	pub area: Option<f64>,
	pub timezone: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct StateDistrictItem {
	pub district: String,
	pub name: String,
	#[serde(rename = "type")]
	pub kind: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct StateDistricts {
	pub state: String,
	pub state_name: Option<String>,
	pub country: String,
	pub country_name: Option<String>,
	#[serde(default, deserialize_with = "null_default")]
	pub districts: Vec<StateDistrictItem>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct District {
	pub district: String,
	pub name: String,
	#[serde(rename = "type")]
	pub kind: Option<String>,
	pub state: Option<String>,
	pub state_name: Option<String>,
	pub country: String,
	pub country_name: Option<String>,
	pub latitude: Option<f64>,
	pub longitude: Option<f64>,
	pub population: Option<i64>,
	pub land_area: Option<f64>,
	pub water_area: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct City {
	pub name: String,
	pub local_name: Option<String>,
	pub state: Option<String>,
	pub state_name: Option<String>,
	pub country: String,
	pub latitude: Option<f64>,
	pub longitude: Option<f64>,
	pub population: Option<i64>,
	pub timezone: Option<String>,
	/// Minted parse id (`city_` + 12 chars). Stable pin via `/city/id/{id}`.
	pub id: String,
}

/// A [`City`] plus the distance from the query point.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CityNearest {
	#[serde(flatten)]
	pub city: City,
	pub distance: f64,
	pub distance_mi: f64,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CitySearch {
	pub q: String,
	pub country: Option<String>,
	pub state: Option<String>,
	#[serde(default, deserialize_with = "null_default")]
	pub cities: Vec<City>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Postal {
	pub postal: String,
	pub city: Option<String>,
	pub city_local: Option<String>,
	pub district: Option<String>,
	pub district_name: Option<String>,
	pub district_name_local: Option<String>,
	pub state: Option<String>,
	pub state_name: Option<String>,
	pub state_name_local: Option<String>,
	pub country: String,
	pub latitude: Option<f64>,
	pub longitude: Option<f64>,
	pub elevation: Option<f64>,
	pub elevation_ft: Option<f64>,
	pub population: Option<i64>,
	pub timezone: Option<String>,
	pub currency: Option<String>,
	#[serde(default, deserialize_with = "null_default")]
	pub neighbors: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct PostalNearbyItem {
	pub postal: String,
	pub city: Option<String>,
	pub state: Option<String>,
	pub country: String,
	pub distance: f64,
	pub distance_mi: f64,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct PostalNearby {
	pub postal: String,
	pub country: String,
	pub radius: f64,
	pub unit: String,
	#[serde(default, deserialize_with = "null_default")]
	pub nearby: Vec<PostalNearbyItem>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct PostalDistanceEnd {
	pub postal: String,
	pub city: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct PostalDistance {
	pub country: String,
	pub from: PostalDistanceEnd,
	pub to: PostalDistanceEnd,
	pub distance: f64,
	pub distance_mi: f64,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct EmailDeep {
	pub deliverable: Option<bool>,
	pub catchall: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Email {
	pub email: String,
	pub valid: bool,
	pub domain: Option<String>,
	pub domain_valid: Option<bool>,
	pub role: bool,
	pub disposable: bool,
	pub deep: Option<EmailDeep>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct PhoneDeep {
	#[serde(rename = "type")]
	pub kind: Option<String>,
	pub carrier: Option<String>,
	/// Carrier is a known burner number app. None when carrier is unknown.
	pub burner: Option<bool>,
	pub city: Option<String>,
	pub state: Option<String>,
	pub state_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Phone {
	pub phone: Option<String>,
	pub valid: bool,
	pub country: Option<String>,
	pub national: Option<String>,
	pub international: Option<String>,
	pub deep: Option<PhoneDeep>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct MxRecord {
	pub priority: i32,
	pub host: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct DomainRegistration {
	pub registered: bool,
	pub created: Option<String>,
	pub updated: Option<String>,
	pub expires: Option<String>,
	pub registrar: Option<String>,
	#[serde(default, deserialize_with = "null_default")]
	pub status: Vec<String>,
	pub dnssec: bool,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct DomainDeep {
	#[serde(default, deserialize_with = "null_default")]
	pub a: Vec<String>,
	#[serde(default, deserialize_with = "null_default")]
	pub aaaa: Vec<String>,
	#[serde(default, deserialize_with = "null_default")]
	pub ns: Vec<String>,
	#[serde(default, deserialize_with = "null_default")]
	pub mx: Vec<MxRecord>,
	#[serde(default, deserialize_with = "null_default")]
	pub txt: Vec<String>,
	pub mailhost: Option<String>,
	pub registration: Option<DomainRegistration>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Domain {
	pub domain: String,
	pub available: bool,
	pub deep: Option<DomainDeep>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Mx {
	pub domain: String,
	#[serde(default, deserialize_with = "null_default")]
	pub mx: Vec<MxRecord>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct UseragentDeviceDeep {
	#[serde(rename = "type")]
	pub kind: Option<String>,
	pub brand: Option<String>,
	pub model: Option<String>,
	pub cpu: Option<String>,
	pub touchscreen: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct UseragentOsDeep {
	pub name: Option<String>,
	pub version: Option<String>,
	pub platform: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct UseragentBrowserBrand {
	pub brand: String,
	pub version: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct UseragentBrowserDeep {
	pub name: Option<String>,
	pub version: Option<String>,
	#[serde(rename = "type")]
	pub kind: Option<String>,
	#[serde(default, deserialize_with = "null_default")]
	pub brands: Vec<UseragentBrowserBrand>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct UseragentEngineDeep {
	pub name: Option<String>,
	pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct UseragentDeep {
	pub device: Option<UseragentDeviceDeep>,
	pub os: Option<UseragentOsDeep>,
	pub browser: Option<UseragentBrowserDeep>,
	pub engine: Option<UseragentEngineDeep>,
	pub headless: Option<bool>,
	pub ai: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Useragent {
	pub useragent: String,
	pub device: Option<String>,
	pub os: Option<String>,
	pub browser: Option<String>,
	pub bot: bool,
	pub mobile: bool,
	pub deep: Option<UseragentDeep>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Currency {
	pub currency: String,
	pub numeric: Option<i32>,
	pub name: String,
	pub name_plural: Option<String>,
	pub symbol: Option<String>,
	pub symbol_native: Option<String>,
	pub digits: Option<i32>,
	#[serde(default, deserialize_with = "null_default")]
	pub countries: Vec<String>,
}

/// One language by BCP 47 shortest code or ISO 639-3. Codes are lowercase.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Language {
	pub language: String,
	pub iso3: Option<String>,
	pub name: String,
	pub local_name: Option<String>,
	pub script: Option<String>,
	pub direction: String,
	#[serde(default, deserialize_with = "null_default")]
	pub countries: Vec<String>,
}

/// A parsed person name. Junk input returns valid false, never an error.
/// Gender comes from dictionary data and is None when the data does not decide.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Name {
	pub name: String,
	pub valid: bool,
	pub prefix: Option<String>,
	pub first: Option<String>,
	pub middle: Option<String>,
	pub last: Option<String>,
	pub suffix: Option<String>,
	pub gender: Option<String>,
	pub salutation: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CurrencyRate {
	pub base: String,
	pub quote: String,
	pub rate: f64,
	pub date: String,
	pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct TimezoneNextDst {
	pub at: String,
	pub dst: bool,
	pub offset: String,
	pub abbreviation: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Timezone {
	pub timezone: String,
	pub name: Option<String>,
	pub abbreviation: String,
	pub offset: String,
	pub offset_minutes: i32,
	pub dst: bool,
	pub next_dst: Option<TimezoneNextDst>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Holiday {
	pub date: String,
	pub name: String,
	pub local_name: Option<String>,
	/// "public" for an official day off, "observance" for cultural days.
	#[serde(rename = "type")]
	pub kind: String,
	#[serde(default, deserialize_with = "null_default")]
	pub regions: Vec<String>,
	pub substitute: bool,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct HolidayYear {
	pub country: String,
	pub year: i32,
	#[serde(default, deserialize_with = "null_default")]
	pub holidays: Vec<Holiday>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct HolidayDate {
	pub country: String,
	pub date: String,
	pub holiday: Option<Holiday>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Elevation {
	pub latitude: f64,
	pub longitude: f64,
	pub elevation: Option<f64>,
	pub elevation_ft: Option<f64>,
	pub resolution: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct PointDeep {
	pub city: Option<CityNearest>,
	pub timezone: Option<Timezone>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Point {
	pub latitude: f64,
	pub longitude: f64,
	pub country: Option<String>,
	pub country_name: Option<String>,
	pub state: Option<String>,
	pub state_name: Option<String>,
	pub district: Option<String>,
	pub district_name: Option<String>,
	pub elevation: Option<f64>,
	pub elevation_ft: Option<f64>,
	pub resolution: Option<f64>,
	pub deep: Option<PointDeep>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct WeatherForecastPeriod {
	pub name: String,
	pub start: Option<String>,
	pub end: Option<String>,
	pub daytime: Option<bool>,
	pub temperature: Option<f64>,
	pub temperature_f: Option<f64>,
	pub precipitation_chance: Option<f64>,
	pub wind_speed: Option<f64>,
	pub wind_speed_mph: Option<f64>,
	pub wind_direction: Option<f64>,
	pub condition: Option<String>,
	pub condition_name: Option<String>,
	pub condition_emoji: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct WeatherAlert {
	pub event: String,
	pub severity: Option<String>,
	pub urgency: Option<String>,
	pub headline: Option<String>,
	pub onset: Option<String>,
	pub expires: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct WeatherDeep {
	#[serde(default, deserialize_with = "null_default")]
	pub forecast: Vec<WeatherForecastPeriod>,
	#[serde(default, deserialize_with = "null_default")]
	pub alerts: Vec<WeatherAlert>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Weather {
	pub latitude: f64,
	pub longitude: f64,
	pub temperature: Option<f64>,
	pub temperature_f: Option<f64>,
	pub feels_like: Option<f64>,
	pub feels_like_f: Option<f64>,
	pub dewpoint: Option<f64>,
	pub dewpoint_f: Option<f64>,
	pub humidity: Option<f64>,
	pub wind_speed: Option<f64>,
	pub wind_speed_mph: Option<f64>,
	pub wind_gust: Option<f64>,
	pub wind_gust_mph: Option<f64>,
	pub wind_direction: Option<f64>,
	pub pressure: Option<f64>,
	pub pressure_inhg: Option<f64>,
	pub visibility: Option<f64>,
	pub visibility_mi: Option<f64>,
	pub condition: Option<String>,
	pub condition_name: Option<String>,
	pub condition_emoji: Option<String>,
	pub observed_at: Option<String>,
	pub station: String,
	pub station_name: Option<String>,
	pub station_distance: f64,
	pub station_distance_mi: f64,
	pub source: String,
	pub source_name: Option<String>,
	pub deep: Option<WeatherDeep>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct EmojiSkin {
	pub emoji: String,
	pub tone: String,
	pub unicode: Option<String>,
	pub hex: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Emoji {
	pub emoji: String,
	pub name: String,
	#[serde(default, deserialize_with = "null_default")]
	pub shortcodes: Vec<String>,
	#[serde(default, deserialize_with = "null_default")]
	pub codepoints: Vec<String>,
	pub hex: String,
	pub category: Option<String>,
	pub status: Option<String>,
	pub version: Option<String>,
	#[serde(default, deserialize_with = "null_default")]
	pub keywords: Vec<String>,
	#[serde(default, deserialize_with = "null_default")]
	pub skins: Vec<EmojiSkin>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct EmojiSearch {
	pub q: String,
	#[serde(default, deserialize_with = "null_default")]
	pub emojis: Vec<Emoji>,
}
