//! Response types for the ParseAPI public API. Nullable fields are `Option`.
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
	/// Present only when supported. District is qualified by the response country.
	pub district: Option<String>,
	pub district_name: Option<String>,
	pub census_division: Option<String>,
	pub census_region: Option<String>,
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

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ContinentCountryItem {
 pub country: String,
 pub name: String,
 pub flag: Option<String>,
 /// Legacy source adapter for the flag emoji.
 pub emoji: Option<String>,
 pub calling_code: Option<String>,
}
impl<'de> Deserialize<'de> for ContinentCountryItem {
 fn deserialize<D: serde::Deserializer<'de>>(decoder: D) -> Result<Self,D::Error> {
  #[derive(Default,Deserialize)] #[serde(default)]
  struct Wire { country:String, name:String, flag:Option<String>, calling_code:Option<String> }
  let mut value=serde_json::Value::deserialize(decoder)?;
  country_shape_alias(&mut value,"flag","emoji");
  let wire:Wire=serde_json::from_value(value).map_err(serde::de::Error::custom)?;
  Ok(Self{country:wire.country,name:wire.name,emoji:wire.flag.clone(),flag:wire.flag,calling_code:wire.calling_code})
 }
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
pub struct CountryCapital { pub name:Option<String>, pub lat:Option<f64>, pub lon:Option<f64> }

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CountryTax {
 pub name:Option<String>,
 /// Standard national reference percentage, not calculated tax due.
 pub rate:Option<f64>,
}

fn country_shape_alias(value:&mut serde_json::Value,current:&str,legacy:&str) {
 if let Some(row)=value.as_object_mut() {
  if !row.contains_key(current) { if let Some(old)=row.get(legacy).cloned(){row.insert(current.to_owned(),old);} }
 }
}
fn country_capital_shape(value:&mut serde_json::Value) {
 if let Some(row)=value.as_object_mut() {
  if row.get("capital").is_some_and(serde_json::Value::is_object){return;}
  if row.get("capital").is_some_and(serde_json::Value::is_string)||row.contains_key("capital_lat")||row.contains_key("capital_lon") {
   let name=row.get("capital").cloned().unwrap_or(serde_json::Value::Null);
   let lat=row.get("capital_lat").cloned().unwrap_or(serde_json::Value::Null);
   let lon=row.get("capital_lon").cloned().unwrap_or(serde_json::Value::Null);
   row.insert("capital".into(),serde_json::json!({"name":name,"lat":lat,"lon":lon}));
  }
 }
}
fn country_tax_shape(value:&mut serde_json::Value) {
 if let Some(row)=value.as_object_mut() {
  if row.get("tax").is_some_and(serde_json::Value::is_object){return;}
  if row.get("tax").is_some_and(serde_json::Value::is_string)||["tax_rate","tax_id_format","tax_id_regex"].iter().any(|key|row.contains_key(*key)) {
   let name=row.get("tax").cloned().unwrap_or(serde_json::Value::Null);
   let rate=row.get("tax_rate").cloned().unwrap_or(serde_json::Value::Null);
   row.insert("tax".into(),serde_json::json!({"name":name,"rate":rate}));
  }
 }
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct Country {
 pub country: String,
 pub name: String,
 pub name_local: Option<String>,
 pub continent: String,
 pub currency: Option<String>,
 pub currency_name: Option<String>,
 pub currency_symbol: Option<String>,
 pub flag: Option<String>,
 pub calling_code: Option<String>,
 pub languages: Vec<String>,
 pub deep: Option<CountryDeep>,
 pub timezones: Option<Vec<String>>,
 pub iso3: Option<String>,
 pub numeric: Option<i32>,
 pub name_full: Option<String>,
 pub demonym: Option<String>,
 pub region: Option<String>,
 pub subregion: Option<String>,
 pub capital: Option<CountryCapital>,
 pub borders: Option<Vec<String>>,
 pub blocs: Option<Vec<String>>,
 /// Legacy source adapters; canonical properties are flag, name_full and capital.
 pub emoji:Option<String>, pub full_name:Option<String>,
 pub capital_lat:Option<f64>, pub capital_lon:Option<f64>,
}
#[derive(Default,Deserialize)]
#[serde(default)]
struct CountryWire {
 country: String,
 name: String,
 name_local: Option<String>,
 continent: String,
 currency: Option<String>,
 currency_name: Option<String>,
 currency_symbol: Option<String>,
 flag: Option<String>,
 calling_code: Option<String>,
 #[serde(default, deserialize_with = "null_default")]
 languages: Vec<String>,
 deep: Option<CountryDeep>,
 timezones: Option<Vec<String>>,
 iso3: Option<String>,
 numeric: Option<i32>,
 name_full: Option<String>,
 demonym: Option<String>,
 region: Option<String>,
 subregion: Option<String>,
 capital: Option<CountryCapital>,
 borders: Option<Vec<String>>,
 blocs: Option<Vec<String>>,
}
impl<'de> Deserialize<'de> for Country {
 fn deserialize<D:serde::Deserializer<'de>>(decoder:D)->Result<Self,D::Error>{
  let mut value=serde_json::Value::deserialize(decoder)?;
  country_shape_alias(&mut value,"name_full","full_name");
  country_shape_alias(&mut value,"flag","emoji");
  country_capital_shape(&mut value);
  let wire:CountryWire=serde_json::from_value(value).map_err(serde::de::Error::custom)?;
  let emoji=wire.flag.clone();let full_name=wire.name_full.clone();
  let capital_lat=wire.capital.as_ref().and_then(|c|c.lat);let capital_lon=wire.capital.as_ref().and_then(|c|c.lon);
  Ok(Self{
   country: wire.country,
   name: wire.name,
   name_local: wire.name_local,
   continent: wire.continent,
   currency: wire.currency,
   currency_name: wire.currency_name,
   currency_symbol: wire.currency_symbol,
   flag: wire.flag,
   calling_code: wire.calling_code,
   languages: wire.languages,
   deep: wire.deep,
   timezones: wire.timezones,
   iso3: wire.iso3,
   numeric: wire.numeric,
   name_full: wire.name_full,
   demonym: wire.demonym,
   region: wire.region,
   subregion: wire.subregion,
   capital: wire.capital,
   borders: wire.borders,
   blocs: wire.blocs,
   emoji,full_name,capital_lat,capital_lon,
  })
 }
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
	pub name_local: Option<String>,
	#[serde(rename = "type")]
	pub kind: Option<String>,
	pub country: String,
	pub country_name: Option<String>,
	pub latitude: Option<f64>,
	pub longitude: Option<f64>,
	#[serde(default, deserialize_with = "null_default")]
	pub timezones: Vec<String>,
	pub iso_3166_2: Option<String>,
	pub deep: Option<StateDeep>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct StateDistrictItem {
	pub district: String,
	pub name: String,
	#[serde(rename = "type")]
	pub kind: Option<String>,
	pub deep: Option<StateDistrictItemDeep>,
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
	#[serde(default, deserialize_with = "null_default")]
	pub timezones: Vec<String>,
	pub deep: Option<DistrictDeep>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct City {
	pub name_romanized: Option<String>,
	pub romanization_language: Option<String>,
	pub r#match: Option<CityMatch>,
	pub name: String,
	pub name_local: Option<String>,
	#[serde(rename = "type")]
	pub kind: Option<String>,
	pub state: Option<String>,
	pub state_name: Option<String>,
	pub district: Option<String>,
	pub district_name: Option<String>,
	pub country: String,
	pub country_name: Option<String>,
	pub latitude: Option<f64>,
	pub longitude: Option<f64>,
	pub timezone: Option<String>,
	/// Minted parse id (`city_` + 12 chars). Stable pin via `/city/id/{id}`.
	pub id: String,
	pub deep: Option<CityDeep>,
	/// Present when hierarchy is requested; missing and null remain unknown.
	pub grain: Option<String>,
	pub parent: Option<CityParent>,
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
	pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CityNearby {
	pub city: String,
	pub state: Option<String>,
	pub country: String,
	pub radius: f64,
	pub unit: String,
	#[serde(default, deserialize_with = "null_default")]
	pub nearby: Vec<CityNearest>,
	pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct PostalMetro {
	pub code: String,
	pub name: String,
	pub r#type: String,
	/// Fraction of ZIP addresses; category shares are measured independently.
	pub share: Option<f64>,
	pub residential_share: Option<f64>,
	pub business_share: Option<f64>,
	pub other_share: Option<f64>,
	/// Percentage of ZIP addresses (0-100). None is unknown or absent.
	pub share_percent: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct PostalLocality {
	pub city: String,
	pub state: String,
	pub state_name: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Postal {
	pub postal: String,
	pub city: Option<String>,
	pub city_local: Option<String>,
	/// None is unknown. An empty list has no eligible choices. One choice does not imply city.
	pub localities: Option<Vec<PostalLocality>>,
	pub district: Option<String>,
	pub district_name: Option<String>,
	pub district_name_local: Option<String>,
	pub state: Option<String>,
	pub state_name: Option<String>,
	pub state_name_local: Option<String>,
	pub country: String,
	pub country_name: Option<String>,
	pub latitude: Option<f64>,
	pub longitude: Option<f64>,
	pub timezone: Option<String>,
	pub deep: Option<PostalDeep>,
	/// Open ZIP type, including future values. None is unknown or absent.
	pub r#type: Option<String>,
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
	pub deep: Option<PostalMetroDeep>,
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
	pub deep: Option<PostalMetroDeep>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct PostalDistanceEnd {
	pub postal: String,
	pub city: Option<String>,
	pub deep: Option<PostalMetroDeep>,
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
	/// Suggested first name. Not a verified identity.
	pub first_name: Option<String>,
	pub no_reply: Option<bool>,
	pub tag: Option<String>,
	pub mail_provider: Option<String>,
	pub deliverable: Option<bool>,
	pub catchall: Option<bool>,
	/// Mailbox status: deliverable, undeliverable or risky. Null when unavailable.
	pub status: Option<String>,
	/// Why the address received this result, such as mailbox_full or mailbox_not_found. Null when unavailable.
	pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Email {
	pub email: String,
	pub valid: bool,
	pub free: bool,
	pub domain: Option<String>,
	pub domain_type: Option<String>,
	pub domain_valid: Option<bool>,
	pub role: bool,
	pub disposable: bool,
	pub deep: Option<EmailDeep>,
	pub didyoumean: Option<String>,
}























































#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Phone {
	pub phone: Option<String>,
	pub valid: bool,
	pub country: Option<String>,
	/// What the numbering plan can see: mobile, landline, toll_free, unknown. Never voip.
	#[serde(rename = "type")]
	pub kind: Option<String>,
	pub national: Option<String>,
	pub international: Option<String>,
	pub deep: Option<PhoneDeep>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Carrier {
	pub phone: Option<String>,
	pub valid: bool,
	pub country: Option<String>,
	/// The network's word, including voip.
	#[serde(rename = "type")]
	pub kind: Option<String>,
	/// Current carrier display name. None when the probe had no answer.
	pub carrier: Option<String>,
	/// Carrier is a known burner number app. None when carrier is unknown.
	pub burner: Option<bool>,
	pub deep: Option<CarrierDeep>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Caller {
	pub phone: Option<String>,
	pub valid: bool,
	pub country: Option<String>,
	/// CNAM record verbatim (all-caps telco artifact). None when no record or outside NANP.
	pub caller: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Hlr {
	pub phone: Option<String>,
	pub valid: bool,
	pub country: Option<String>,
	/// Assigned to a subscriber at the last check.
	pub live: Option<bool>,
	/// Handset reachable at the last check. None means unconfirmed, never no.
	pub connected: Option<bool>,
	pub deep: Option<HlrDeep>,
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
pub struct StackTechnology {
	pub technology: String,
	pub name: String,
	pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Stack {
	pub domain: String,
	pub url: String,
	pub checked_at: Option<String>,
	pub cms: Option<Vec<StackTechnology>>,
	pub servers: Option<Vec<StackTechnology>>,
	pub frameworks: Option<Vec<StackTechnology>>,
	pub ecommerce: Option<Vec<StackTechnology>>,
	pub analytics: Option<Vec<StackTechnology>>,
	pub chat: Option<Vec<StackTechnology>>,
	pub payments: Option<Vec<StackTechnology>>,
	pub hosting: Option<Vec<StackTechnology>>,
	pub deep: Option<serde_json::Value>,
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
	/// None means signing evidence is unknown.
	pub dnssec: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct DomainDeep {
	pub registration: Option<DomainRegistration>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Domain {
	pub domain: String,
	/// Some(true) means unregistered, Some(false) registered, None unknown.
	pub available: Option<bool>,
	pub deep: Option<DomainDeep>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Asn {
	pub asn: u32,
	pub name: Option<String>,
	pub country: Option<String>,
	pub country_name: Option<String>,
}


/// Network identity. None means unknown or ambiguous.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Card {
	pub bin: String,
	pub brand: Option<String>,
	pub brand_name: Option<String>,
	pub logo: String,
	pub deep: Option<CardDeep>,
}

/// Optional recorded issuer data. None fields mean unknown.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CardDeep {
	pub prefix: Option<String>,
	pub issuer: Option<String>,
	pub country: Option<String>,
	pub r#type: Option<String>,
	pub prepaid: Option<bool>,
}

/// A published DNS record. Value retains DNS presentation syntax, including TXT quoting.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct DnsRecord {
	pub name: String,
	pub r#type: String,
	pub ttl: u32,
	pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Dns {
	pub domain: String,
	#[serde(default, deserialize_with = "null_default")]
	pub records: Vec<DnsRecord>,
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
	pub bot: Option<serde_json::Value>,
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


/// One language by BCP 47 shortest code or ISO 639-3. Codes are lowercase.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Language {
	pub language: String,
	pub name: String,
	pub name_local: Option<String>,
	pub script: Option<String>,
	pub direction: String,
	pub deep: Option<LanguageDeep>,
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
	pub deep: Option<NameDeep>,
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

/// Local clock and timezone facts. Missing clock fields remain unknown.
pub type Time = Timezone;

/// Serving timezone IDs and their pinned rule edition.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct TimeZones {
	pub timezone_database_version: String,
	pub timezones: Vec<String>,
	pub at: Option<String>,
	pub zones: Option<Vec<TimeZoneEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Timezone {
	pub location: Option<TimeLocation>,
	pub timezone: Option<String>,
	pub abbreviation: Option<String>,
	pub offset: Option<String>,
	pub dst: Option<bool>,
	pub latitude: Option<f64>,
	pub longitude: Option<f64>,
	pub at: Option<String>,
	pub unix: Option<i64>,
	pub to: Option<TimezoneConversionTarget>,
	pub targets: Option<Vec<TimezoneConversionTarget>>,
	pub deep: Option<TimezoneDeep>,
}

/// Calendar facts for a date. Calendar fields are None when valid is false.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct DateInfo {
	pub date: String,
	pub valid: bool,
	pub unix: Option<i64>,
	pub to: Option<String>,
	pub days: Option<i32>,
	pub deep: Option<DateInfoDeep>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Holiday {
	pub date: String,
	pub name: String,
	pub name_local: Option<String>,
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
	pub city: Option<PointCity>,
	pub elevation_ft: Option<f64>,
	pub resolution: Option<f64>,
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
	pub deep: Option<PointDeep>,
	pub timezone: Option<String>,
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
pub struct WeatherHour {
	pub at: Option<String>,
	pub daytime: Option<bool>,
	pub temperature: Option<f64>,
	pub temperature_f: Option<f64>,
	pub humidity: Option<f64>,
	pub precipitation_chance: Option<f64>,
	pub wind_speed: Option<f64>,
	pub wind_speed_mph: Option<f64>,
	pub wind_direction: Option<f64>,
	pub condition: Option<String>,
	pub condition_name: Option<String>,
	pub condition_emoji: Option<String>,
	pub feels_like: Option<f64>,
	pub feels_like_f: Option<f64>,
	pub wind_gust: Option<f64>,
	pub wind_gust_mph: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct WeatherMinute {
	pub at: Option<String>,
	pub precipitation: Option<f64>,
	pub precipitation_in: Option<f64>,
	#[serde(rename = "type")]
	pub precipitation_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct WeatherDay {
	pub date: Option<String>,
	pub high: Option<f64>,
	pub high_f: Option<f64>,
	pub low: Option<f64>,
	pub low_f: Option<f64>,
	pub precipitation_chance: Option<f64>,
	pub condition: Option<String>,
	pub condition_name: Option<String>,
	pub condition_emoji: Option<String>,
	pub sunrise: Option<String>,
	pub sunset: Option<String>,
	pub moon_phase: Option<String>,
	pub moon_phase_name: Option<String>,
	pub moon_phase_emoji: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct WeatherDeep {
	#[serde(default, deserialize_with = "null_default")]
	pub forecast: Vec<WeatherForecastPeriod>,
	#[serde(default, deserialize_with = "null_default")]
	pub alerts: Vec<WeatherAlert>,
	#[serde(default, deserialize_with = "null_default")]
	pub minutes: Vec<WeatherMinute>,
	#[serde(default, deserialize_with = "null_default")]
	pub hours: Vec<WeatherHour>,
	#[serde(default, deserialize_with = "null_default")]
	pub days: Vec<WeatherDay>,
	pub air: Option<WeatherAir>,
	pub history: Option<WeatherHistory>,
	pub current: Option<WeatherCurrentDeep>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct WeatherCurrent {
	pub temperature: Option<f64>,
	pub temperature_f: Option<f64>,
	pub feels_like: Option<f64>,
	pub feels_like_f: Option<f64>,
	pub humidity: Option<f64>,
	pub wind_speed: Option<f64>,
	pub wind_speed_mph: Option<f64>,
	pub wind_direction: Option<f64>,
	pub condition: Option<String>,
	pub condition_name: Option<String>,
	pub condition_emoji: Option<String>,
	pub observed_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct WeatherStation {
	pub id: String,
	pub name: Option<String>,
	pub distance: Option<f64>,
	pub distance_mi: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Weather {
	pub latitude: f64,
	pub longitude: f64,
	pub current: WeatherCurrent,
	pub station: Option<WeatherStation>,
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
	pub category: Option<String>,
	pub deep: Option<EmojiDeep>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct EmojiSearch {
	pub total: Option<u32>,
	pub next_offset: Option<u32>,
	pub q: String,
	#[serde(default, deserialize_with = "null_default")]
	pub emojis: Vec<Emoji>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct TimezoneConversionTarget {
	pub timezone: String,
	pub abbreviation: Option<String>,
	pub offset: String,
	pub dst: bool,
	pub at: String,
	pub unix: Option<i64>,
	pub deep: Option<TimezoneConversionTargetDeep>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct WeatherAir {
	pub aqi: Option<f64>,
	pub aqi_name: Option<String>,
	pub pm2_5: Option<f64>,
	pub pm10: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct WeatherHistory {
	pub date: String,
	pub high: Option<f64>,
	pub high_f: Option<f64>,
	pub low: Option<f64>,
	pub low_f: Option<f64>,
	pub precipitation: Option<f64>,
	pub precipitation_in: Option<f64>,
	pub wind_max: Option<f64>,
	pub wind_max_mph: Option<f64>,
	pub sunrise: Option<String>,
	pub sunset: Option<String>,
	pub moon_phase: Option<String>,
	pub moon_phase_name: Option<String>,
	pub moon_phase_emoji: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Address {
	pub address: Option<String>,
	pub valid: bool,
	pub registered: Option<bool>,
	pub number: Option<String>,
	pub street: Option<String>,
	pub unit: Option<String>,
	pub city: Option<String>,
	pub district: Option<String>,
	pub district_name: Option<String>,
	pub state: Option<String>,
	pub state_name: Option<String>,
	pub postal: Option<String>,
	pub country: Option<String>,
	pub country_name: Option<String>,
	pub latitude: Option<f64>,
	pub longitude: Option<f64>,
	pub deep: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct AddressSuggestion {
	pub address: String,
	pub number: Option<String>,
	pub street: Option<String>,
	pub unit: Option<String>,
	pub city: Option<String>,
	pub state: Option<String>,
	pub postal: Option<String>,
	pub latitude: Option<f64>,
	pub longitude: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct AddressSearch {
	pub q: String,
	pub postal: Option<String>,
	pub city: Option<String>,
	pub state: Option<String>,
	pub country: Option<String>,
	#[serde(default, deserialize_with = "null_default")]
	pub addresses: Vec<AddressSuggestion>,
	/// Why suggestions are empty: more_input, missing_context or no_matches. Null with suggestions. Open to future values. Operational failures are errors.
	pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CompanyCountry {
	pub name: Option<String>,
	#[serde(default, deserialize_with = "null_default")]
	pub blocs: Vec<String>,
	/// Levy name, such as VAT, GST or sales tax. Null when unknown or not applicable.
	pub tax: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CompanyDeep {
	pub activity: Option<String>,
	pub state_name: Option<String>,
	pub country_name: Option<String>,
	pub vat: Option<String>,
	pub gst: Option<bool>,
	pub acn: Option<String>,
	pub siren: Option<String>,
	pub siege: Option<bool>,
	pub kind: Option<String>,
	pub invoice: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Company {
	pub company: Option<String>,
	pub valid: bool,
	pub registered: Option<bool>,
	pub country: Option<String>,
	pub r#type: Option<String>,
	pub name: Option<String>,
	pub active: Option<bool>,
	pub address: Option<String>,
	pub city: Option<String>,
	pub state: Option<String>,
	pub postal: Option<String>,
	pub deep: Option<CompanyDeep>,
}


#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct NaicsChild {
	pub naics: String,
	pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct NaicsExclusion {
	pub description: String,
	/// Generic exclusions can have no linked codes.
	#[serde(default, deserialize_with = "null_default")]
	pub codes: Vec<NaicsChild>,
}

/// A query token corrected only during typo fallback.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct NaicsCorrection {
	pub from: String,
	pub to: String,
}

/// The actual title, activity term or code that matched a search.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct NaicsMatch {
	/// Currently name, term or naics. Future fields remain decodable.
	pub field: String,
	pub text: String,
	/// Empty for exact, plural and prefix matches.
	#[serde(default, deserialize_with = "null_default")]
	pub corrections: Vec<NaicsCorrection>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Naics {
	pub naics: String,
	pub name: String,
	pub level: u32,
	pub parent: Option<String>,
	pub parent_name: Option<String>,
	/// Search evidence, absent on direct lookup and older responses.
	pub r#match: Option<NaicsMatch>,
	pub year: u32,
	pub country: String,
	pub deep: Option<NaicsDeep>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct NaicsSearch {
	pub q: String,
	pub year: u32,
	pub country: String,
	#[serde(default, deserialize_with = "null_default")]
	pub results: Vec<NaicsSearchResult>,
}


#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct CountryDeep {
	/// Latest available GDP in current US dollars.
	pub gdp: Option<f64>,
	/// Latest available GDP per person in current US dollars.
	pub gdp_per_capita: Option<f64>,
	/// Annual percent change; negative values indicate decline.
	pub population_growth: Option<f64>,
	/// Share of population, from 0 to 100.
	pub urban_population: Option<f64>,
	/// Share of population using the internet, from 0 to 100.
	pub internet_users: Option<f64>,
	pub iso3: Option<String>,
	pub numeric: Option<i32>,
	pub full_name: Option<String>,
	pub demonym: Option<String>,
	pub capital: Option<String>,
	pub capital_lat: Option<f64>,
	pub capital_lon: Option<f64>,
	pub region: Option<String>,
	pub subregion: Option<String>,
	pub population: Option<i64>,
	/// Retained for source compatibility. No longer returned or decoded.
	#[deprecated(note = "Population observation periods are no longer returned")]
	pub population_period: Option<String>,
	pub area: Option<f64>,
	/// Land area in km2.
	pub land_area: Option<f64>,
	/// Water area in km2.
	pub water_area: Option<f64>,
	/// Coastline length in km. Zero is a known landlocked coastline.
	pub coastline: Option<f64>,
	/// Mean elevation in metres above sea level.
	pub elevation: Option<f64>,
	pub lowest_point: Option<CountryElevationPoint>,
	pub highest_point: Option<CountryElevationPoint>,
	pub tld: Option<String>,
	pub borders: Option<Vec<String>>,
	pub blocs: Option<Vec<String>>,
	/// Country tax reference; individual unknown facts remain null.
	pub tax: Option<CountryTax>,
	/// Standard country reference rate in percent (19 means 19%). Null is unknown, zero is known zero.
	pub tax_rate: Option<f64>,
	/// Tax registration number mask (9 is a digit, A is a letter). Describes format only.
	pub tax_id_format: Option<String>,
	/// Anchored tax registration number format regex. A match does not establish registration.
	pub tax_id_regex: Option<String>,
	pub week_start: Option<String>,
	pub units: Option<String>,
	pub driving_side: Option<String>,
	pub plugs: Option<Vec<String>>,
	pub voltage: Option<i32>,
	pub frequency: Option<i32>,
	pub emergency: Option<CountryEmergency>,
	pub postal_format: Option<String>,
	pub postal_regex: Option<String>,
	pub ioc: Option<String>,
	pub fifa: Option<String>,
	pub plate: Option<String>,
	pub gdp_usd: Option<f64>,
	pub gdp_per_capita_usd: Option<f64>,
	pub population_growth_percent: Option<f64>,
	pub urban_population_percent: Option<f64>,
	pub internet_users_percent: Option<f64>,
	/// Average terrain elevation in meters above sea level.
	pub elevation_average: Option<f64>,
}

#[derive(Default,Deserialize)]
#[serde(default)]
struct CountryDeepWire {
 gdp: Option<f64>,
 gdp_per_capita: Option<f64>,
 population_growth: Option<f64>,
 urban_population: Option<f64>,
 internet_users: Option<f64>,
 iso3: Option<String>,
 numeric: Option<i32>,
 full_name: Option<String>,
 demonym: Option<String>,
 capital: Option<String>,
 capital_lat: Option<f64>,
 capital_lon: Option<f64>,
 region: Option<String>,
 subregion: Option<String>,
 population: Option<i64>,
 area: Option<f64>,
 land_area: Option<f64>,
 water_area: Option<f64>,
 coastline: Option<f64>,
 elevation: Option<f64>,
 lowest_point: Option<CountryElevationPoint>,
 highest_point: Option<CountryElevationPoint>,
 tld: Option<String>,
 borders: Option<Vec<String>>,
 blocs: Option<Vec<String>>,
 tax: Option<CountryTax>,
 tax_id_format: Option<String>,
 tax_id_regex: Option<String>,
 week_start: Option<String>,
 units: Option<String>,
 driving_side: Option<String>,
 plugs: Option<Vec<String>>,
 voltage: Option<i32>,
 frequency: Option<i32>,
 emergency: Option<CountryEmergency>,
 postal_format: Option<String>,
 postal_regex: Option<String>,
 ioc: Option<String>,
 fifa: Option<String>,
 plate: Option<String>,
 gdp_usd: Option<f64>,
 gdp_per_capita_usd: Option<f64>,
 population_growth_percent: Option<f64>,
 urban_population_percent: Option<f64>,
 internet_users_percent: Option<f64>,
 elevation_average: Option<f64>,
}
impl<'de> Deserialize<'de> for CountryDeep {
 fn deserialize<D:serde::Deserializer<'de>>(decoder:D)->Result<Self,D::Error>{
  let mut value=serde_json::Value::deserialize(decoder)?;
  let legacy_tax = !value.get("tax").is_some_and(serde_json::Value::is_object);
  if !legacy_tax {if let Some(row)=value.as_object_mut(){row.remove("tax_id_format");row.remove("tax_id_regex");}}
  country_tax_shape(&mut value);
  let wire:CountryDeepWire=serde_json::from_value(value).map_err(serde::de::Error::custom)?;
  let tax_rate=wire.tax.as_ref().and_then(|t|t.rate);
  let tax_id_format=if legacy_tax {wire.tax_id_format.clone()} else {None};
  let tax_id_regex=if legacy_tax {wire.tax_id_regex.clone()} else {None};
  #[allow(deprecated)]
  Ok(Self{
   gdp: wire.gdp,
   gdp_per_capita: wire.gdp_per_capita,
   population_growth: wire.population_growth,
   urban_population: wire.urban_population,
   internet_users: wire.internet_users,
   iso3: wire.iso3,
   numeric: wire.numeric,
   full_name: wire.full_name,
   demonym: wire.demonym,
   capital: wire.capital,
   capital_lat: wire.capital_lat,
   capital_lon: wire.capital_lon,
   region: wire.region,
   subregion: wire.subregion,
   population: wire.population,
   population_period: None,
   area: wire.area,
   land_area: wire.land_area,
   water_area: wire.water_area,
   coastline: wire.coastline,
   elevation: wire.elevation,
   lowest_point: wire.lowest_point,
   highest_point: wire.highest_point,
   tld: wire.tld,
   borders: wire.borders,
   blocs: wire.blocs,
   tax: wire.tax,
   tax_rate: tax_rate,
   tax_id_format: tax_id_format,
   tax_id_regex: tax_id_regex,
   week_start: wire.week_start,
   units: wire.units,
   driving_side: wire.driving_side,
   plugs: wire.plugs,
   voltage: wire.voltage,
   frequency: wire.frequency,
   emergency: wire.emergency,
   postal_format: wire.postal_format,
   postal_regex: wire.postal_regex,
   ioc: wire.ioc,
   fifa: wire.fifa,
   plate: wire.plate,
   gdp_usd: wire.gdp_usd,
   gdp_per_capita_usd: wire.gdp_per_capita_usd,
   population_growth_percent: wire.population_growth_percent,
   urban_population_percent: wire.urban_population_percent,
   internet_users_percent: wire.internet_users_percent,
   elevation_average: wire.elevation_average,
  })
 }
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct CountryElevationPoint {
	pub name: Option<String>,
	/// Elevation in metres above sea level. Values below sea level are negative.
	pub elevation: f64,
}


#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct StateDeep {
	pub population: Option<i64>,
	/// Retained for source compatibility. No longer returned or decoded.
	#[deprecated(note = "Population observation periods are no longer returned")]
	#[serde(skip)]
	pub population_period: Option<String>,
	pub area: Option<f64>,
	pub fips: Option<String>,
	pub capital: Option<String>,

	pub area_codes: Option<Vec<String>>,
	/// Levy name, such as VAT, GST or sales tax. Null when unknown or not applicable.
	pub tax: Option<String>,
	/// State or province reference rate in percent. Country, state and postal rates are alternative references, not additive.
	pub tax_rate: Option<f64>,
}


#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct DistrictDeep {
	pub population: Option<i64>,
	/// Retained for source compatibility. No longer returned or decoded.
	#[deprecated(note = "Population observation periods are no longer returned")]
	#[serde(skip)]
	pub population_period: Option<String>,
	/// Total area in km2 (land + water, or the official total).
	pub area: Option<f64>,
	/// Land area in km2. None when the source publishes total only.
	pub land_area: Option<f64>,
	/// Water area in km2. None when the source publishes total only.
	pub water_area: Option<f64>,
	pub seat: Option<String>,
	/// Median annual property tax payable on owner-occupied homes in this statistical area. Null when unsupported, missing or censored.
	pub property_tax: Option<PropertyTax>,
}


#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CityDeep {
	/// What this city is the capital of: country, state, or none.
	pub capital_of: Option<String>,
	pub elevation: Option<f64>,
	pub elevation_ft: Option<f64>,
	pub population: Option<i64>,
	/// Retained for source compatibility. No longer returned or decoded.
	#[deprecated(note = "Population observation periods are no longer returned")]
	#[serde(skip)]
	pub population_period: Option<String>,
	/// Official observation date and basis; absent or null means unknown.
	pub population_observed_on: Option<String>,
	pub population_basis: Option<String>,
	/// Total area in km2 (land + water, or the official total).
	pub area: Option<f64>,
	/// Land area in km2. None when the source publishes total only.
	pub land_area: Option<f64>,
	/// Water area in km2. None when the source publishes total only.
	pub water_area: Option<f64>,
}


#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct PostalDeep {
	pub elevation: Option<f64>,
	pub elevation_ft: Option<f64>,
	pub population: Option<i64>,
	/// Retained for source compatibility. No longer returned or decoded.
	#[deprecated(note = "Population observation periods are no longer returned")]
	#[serde(skip)]
	pub population_period: Option<String>,
	/// Total area in km2. None when the source has no water split.
	pub area: Option<f64>,
	/// Land area in km2, where the source has it.
	pub land_area: Option<f64>,
	/// Water area in km2, where the source has it.
	pub water_area: Option<f64>,
	pub currency: Option<String>,

	pub neighbors: Option<Vec<String>>,
	/// None is unknown; Some(empty) is observed outside all covered areas.
	pub metros: Option<Vec<PostalMetro>>,
	/// Levy name, such as VAT, GST or sales tax. Null when unknown or not applicable.
	pub tax: Option<String>,
	/// Combined US ZIP reference rate in percent (7.9 means 7.9%). An exact address can differ. Null is unknown, zero is known zero.
	pub tax_rate: Option<f64>,
	/// State component of the ZIP reference rate, in percent. Null when unknown.
	pub tax_rate_state: Option<f64>,
	/// County component of the ZIP reference rate, in percent. Null when unknown.
	pub tax_rate_county: Option<f64>,
	/// City component of the ZIP reference rate, in percent. Null when unknown.
	pub tax_rate_city: Option<f64>,
	/// Special component of the ZIP reference rate, in percent. Null when unknown.
	pub tax_rate_special: Option<f64>,
	/// Median annual property tax payable on owner-occupied homes in this statistical area. Null when unsupported, missing or censored.
	pub property_tax: Option<PropertyTax>,
}





#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct PhoneDeep {
	/// NPA-derived state code (US/CA).
	pub state: Option<String>,
	pub state_name: Option<String>,
	/// Numbering-plan IANA zone. None when the prefix covers more than one zone.
	pub timezone: Option<String>,
}


#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CarrierDeep {
	/// Issuing rate-center city.
	pub city: Option<String>,
	pub state: Option<String>,
	pub state_name: Option<String>,
}


#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct HlrDeep {
	/// Network diagnostics available from the last check. None when unconfirmed.
	pub roaming: Option<bool>,
	pub roaming_network: Option<String>,
	/// ISO2, uppercase.
	pub roaming_country: Option<String>,
	/// Serving network name at the last check.
	pub network: Option<String>,
	pub original_network: Option<String>,
	pub mcc: Option<String>,
	pub mnc: Option<String>,
}


#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct NaicsDeep {
	pub description: Option<String>,

	pub children: Option<Vec<NaicsChild>>,
	/// Classification exclusions. None for omitted/null older responses.
	pub exclusions: Option<Vec<NaicsExclusion>>,
}


#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct LanguageDeep {
	pub iso3: Option<String>,

	pub countries: Option<Vec<String>>,
}


#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct NameDeep {
	pub gender: Option<String>,
	pub salutation: Option<String>,
	pub short: Option<String>,
	pub directory: Option<String>,
	pub initials: Option<String>,
}


#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct TimezoneDeep {
	pub standard_offset: Option<String>,
	pub standard_offset_seconds: Option<i32>,
	pub dst_offset_seconds: Option<i32>,
	pub season: Option<TimeSeason>,
	pub timezone_database_version: Option<String>,
	pub resolution: Option<TimeResolution>,
	pub name: Option<String>,
	pub offset_seconds: Option<i32>,
	pub offset_minutes: Option<i32>,
	pub next_dst: Option<TimezoneNextDst>,
}


#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct TimezoneConversionTargetDeep {
	pub name: Option<String>,
	pub offset_seconds: Option<i32>,
	pub offset_minutes: Option<i32>,
}


#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct DateInfoDeep {
	pub year: Option<i32>,
	pub month: Option<i32>,
	pub month_name: Option<String>,
	pub day: Option<i32>,
	pub weekday: Option<i32>,
	pub weekday_name: Option<String>,
	pub week: Option<i32>,
	pub week_year: Option<i32>,
	pub day_of_year: Option<i32>,
	pub quarter: Option<i32>,
	pub leap: Option<bool>,
	pub days_in_month: Option<i32>,
}


#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct EmojiDeep {
	pub artwork: Option<std::collections::HashMap<String, EmojiArtwork>>,
	pub subcategory: Option<String>,
	pub base: Option<String>,
	pub variants: Option<Vec<EmojiVariant>>,

	pub codepoints: Option<Vec<String>>,
	pub hex: Option<String>,
	pub status: Option<String>,
	pub version: Option<String>,

	pub keywords: Option<Vec<String>>,

	pub skins: Option<Vec<EmojiSkin>>,
}


#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct WeatherCurrentDeep {
	pub dewpoint: Option<f64>,
	pub dewpoint_f: Option<f64>,
	pub wind_gust: Option<f64>,
	pub wind_gust_mph: Option<f64>,
	pub pressure: Option<f64>,
	pub pressure_inhg: Option<f64>,
	pub visibility: Option<f64>,
	pub visibility_mi: Option<f64>,
}


#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct PointCity {
	#[serde(rename = "type")]
	pub city_type: Option<String>,
	pub name: Option<String>,
	pub name_local: Option<String>,
	pub state: Option<String>,
	pub state_name: Option<String>,
	pub country: Option<String>,
	pub country_name: Option<String>,
	pub latitude: Option<f64>,
	pub longitude: Option<f64>,
	/// Minted parse id (`city_` + 12 chars). Stable pin via `/city/id/{id}`.
	pub id: Option<String>,
	pub distance: Option<f64>,
	pub distance_mi: Option<f64>,
}


#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct PostalMetroDeep {
	pub metros: Option<Vec<PostalMetro>>,
}


#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct StateDistrictItemDeep {
	pub population: Option<i64>,
	/// Retained for source compatibility. No longer returned or decoded.
	#[deprecated(note = "Population observation periods are no longer returned")]
	#[serde(skip)]
	pub population_period: Option<String>,

}


#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CountryEmergency {
	pub police: Option<String>,
	pub ambulance: Option<String>,
	pub fire: Option<String>,
}


#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct NaicsSearchResult {
	pub naics: String,
	pub name: String,
	pub level: u32,
	pub parent: Option<String>,
	pub parent_name: Option<String>,
	/// Search evidence, absent on direct lookup and older responses.
	pub r#match: Option<NaicsMatch>,
	pub deep: Option<NaicsDeep>,
}

/// Property-tax estimate for an area, not a specific property.
#[derive(Debug, Clone, Default, Deserialize)]
#[non_exhaustive]
pub struct PropertyTax {
	/// Median annual tax payable, in currency units adjusted to the final year of period. Not a tax rate or an individual property bill.
	pub annual_median: f64,
	/// ISO 4217 currency code, currently USD.
	pub currency: String,
	/// Reporting period, YYYY-YYYY. Monetary amounts use the final year of this period.
	pub period: String,
}

// Company directory responses. National Company and CompanyDeep remain separate.

/// An address with its recorded role; role does not imply mailing validity or headquarters.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CompanyProfileAddress {
	pub r#type: String,
	pub street: Option<String>,
	pub city: Option<String>,
	pub state: Option<String>,
	pub postal: Option<String>,
	pub country: Option<String>,
}

/// A reported exchange/symbol pair. No listings does not establish private ownership.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CompanyProfileListing {
	pub exchange: String,
	pub symbol: String,
}

/// Recorded registration jurisdiction, separate from address or operating location.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CompanyProfileJurisdiction {
	pub country: String,
	pub state: Option<String>,
}

/// Another associated hostname and its recorded URL, when known.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CompanyProfileWebsite {
	pub domain: String,
	pub url: Option<String>,
}

/// An authority-scoped identifier; values preserve leading zeros.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CompanyProfileIdentifier {
	pub r#type: String,
	pub authority: String,
	pub value: String,
}

/// A reported classification; type is an open scheme string.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CompanyProfileIndustry {
	pub r#type: String,
	pub code: String,
	pub name: Option<String>,
}

/// Reported founding value and precision (year, month or day), distinct from incorporation.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CompanyProfileFounding {
	pub value: String,
	/// Open string; currently year, month or day. Preserve the source value without padding.
	pub precision: String,
}

/// Reported workforce count at its explicit measurement date.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CompanyProfileEmployees {
	pub count: u64,
	pub as_of: String,
	/// Open string: headcount, full_time or full_time_equivalent. None on older responses.
	pub basis: Option<String>,
	/// Open string, currently legal_entity or consolidated_group.
	pub scope: String,
	/// Open string, currently reported.
	pub method: String,
	pub approximate: bool,
}

/// Legal form recorded by a register; codes remain open strings.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CompanyProfileRegistrationLegalForm {
	pub code: String,
	pub name: String,
}

/// Recorded principal-address components, not inferred ISO codes or headquarters.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CompanyProfileRegistrationAddress {
	pub kind: String,
	pub line1: Option<String>,
	pub line2: Option<String>,
	pub city: Option<String>,
	pub state: Option<String>,
	pub postal: Option<String>,
	pub country_raw: Option<String>,
}

/// Registry-scoped legal facts, not an operation or tax-exemption verdict.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CompanyProfileRegistration {
	pub authority: String,
	pub number: String,
	pub jurisdiction: CompanyProfileJurisdiction,
	pub role: String,
	pub legal_form: CompanyProfileRegistrationLegalForm,
	pub status: String,
	/// This register's reported entity-form date, not universal incorporation or founding.
	pub formation_date: Option<String>,
	pub address: Option<CompanyProfileRegistrationAddress>,
}

/// Attribution only for the named selected enrichment fields; observation is not a source update.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CompanyProfileSource {
	pub r#type: String,
	pub url: String,
	#[serde(default, deserialize_with = "null_default")]
	pub fields: Vec<String>,
	/// Artifact observation timestamp.
	pub observed_at: String,
	/// Explicit source update timestamp, or null. Measurement dates belong to employees.as_of.
	pub updated_at: Option<String>,
}

/// Selected public business number, not a reachability verdict.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CompanyProfilePhone {
	pub number: String,
	pub kind: Option<String>,
}

/// Selected public business email, not a delivery verdict.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CompanyProfileEmail {
	pub address: String,
	pub kind: Option<String>,
}

/// An associated domain and its recorded URL.
pub type CompanyProfileDomain = CompanyProfileWebsite;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CompanySocialProfile {
 pub platform: Option<String>,
 pub url: String,
 pub handle: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CompanyPhoneNumber {
 pub number: String,
 pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CompanyEmailAddress {
 pub address: String,
 pub r#type: Option<String>,
}

/// Optional directory detail. Every member may be missing or null; existing releases may omit enrichment fields.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CompanyProfileDeep {
 pub domains: Option<Vec<CompanyProfileDomain>>,
 pub social_profiles: Option<Vec<CompanySocialProfile>>,
 pub phone_numbers: Option<Vec<CompanyPhoneNumber>>,
 pub email_addresses: Option<Vec<CompanyEmailAddress>>,
	pub legal_name: Option<String>,
	pub aliases: Option<Vec<String>>,
	pub jurisdiction: Option<CompanyProfileJurisdiction>,
	/// Recorded legal status; not an operating or compliance verdict.
	pub status: Option<String>,
	pub websites: Option<Vec<CompanyProfileWebsite>>,
	pub identifiers: Option<Vec<CompanyProfileIdentifier>>,
	pub incorporated: Option<String>,
	pub addresses: Option<Vec<CompanyProfileAddress>>,
	pub industries: Option<Vec<CompanyProfileIndustry>>,
	pub parent: Option<String>,
	pub description: Option<String>,
	/// Reported asset URL; the client does not fetch or license the asset.
	pub logo: Option<String>,
	/// Selected company account URLs; an empty array does not prove no accounts exist.
	pub socials: Option<Vec<String>>,
	pub phones: Option<Vec<CompanyProfilePhone>>,
	pub emails: Option<Vec<CompanyProfileEmail>>,
	pub founded: Option<CompanyProfileFounding>,
	pub employees: Option<CompanyProfileEmployees>,
	pub registrations: Option<Vec<CompanyProfileRegistration>>,
	/// Attribution for projected enrichment fields only, not the entire legal profile.
	pub sources: Option<Vec<CompanyProfileSource>>,
}

/// Search match evidence. Open strings permit future fields and identifier/listing namespaces.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CompanyMatch {
	pub field: Option<String>,
	pub value: Option<String>,
	pub r#type: Option<String>,
	pub authority: Option<String>,
	pub exchange: Option<String>,
}

/// A directory profile, distinct from national company-number validation.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CompanyProfile {
	pub id: String,
	pub name: String,
	pub country: Option<String>,
	pub website: Option<String>,
	#[serde(default, deserialize_with = "null_default")]
	pub listings: Vec<CompanyProfileListing>,
	pub address: Option<CompanyProfileAddress>,
	pub deep: Option<CompanyProfileDeep>,
}

/// A directory search profile with match evidence; a match is not proof of legal identity.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CompanyCandidate {
	pub id: String,
	pub name: String,
	pub country: Option<String>,
	pub website: Option<String>,
	#[serde(default, deserialize_with = "null_default")]
	pub listings: Vec<CompanyProfileListing>,
	pub address: Option<CompanyProfileAddress>,
	pub deep: Option<CompanyProfileDeep>,
	pub r#match: CompanyMatch,
}

/// One page of company candidates. Reuse next with the same selector, filters and limit.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CompanySearch {
	#[serde(default, deserialize_with = "null_default")]
	pub companies: Vec<CompanyCandidate>,
	pub next: Option<String>,
}

/// Counts for this directory edition, not complete country or worldwide coverage.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CompanyCoverage {
	pub scope: String,
	pub label: String,
	pub description: String,
	pub snapshot_at: String,
	pub companies: u64,
	#[serde(default, deserialize_with = "null_default")]
	pub countries: Vec<String>,
	pub with_website: u64,
	pub with_listings: u64,
	pub with_address: u64,
}

/// Source wall-time resolution. Missing observations remain unknown.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct TimeResolution {
	pub kind: Option<String>,
	pub policy: Option<String>,
	pub adjustment_seconds: Option<i32>,
	pub alternatives: Option<Vec<TimeResolutionAlternative>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct TimeResolutionAlternative {
	pub at: Option<String>,
	pub unix: Option<i64>,
	pub offset: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct TimeZoneEntry {
	pub timezone: String,
	#[serde(deserialize_with = "null_default")]
	pub countries: Vec<String>,
	pub area: Option<String>,
	pub abbreviation: String,
	pub offset: String,
	pub offset_seconds: i32,
	pub dst: bool,
	pub observes_dst: bool,
}
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct TimeTransitionState {
	pub at: Option<String>,
	pub offset: Option<String>,
	pub offset_seconds: Option<i32>,
	pub abbreviation: Option<String>,
	pub dst: Option<bool>,
}
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct TimeTransition {
	pub at: Option<String>,
	pub before: Option<TimeTransitionState>,
	pub after: Option<TimeTransitionState>,
	pub change_seconds: Option<i32>,
}
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct TimeSeason {
	pub start: Option<TimeTransition>,
	pub end: Option<TimeTransition>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct TimeLocationInput {
	pub r#type: String,
	pub value: String,
}
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct TimeLocationCandidate {
	pub id: Option<String>,
	pub name: Option<String>,
	pub country: Option<String>,
	pub state: Option<String>,
	pub timezone: Option<String>,
	pub latitude: Option<f64>,
	pub longitude: Option<f64>,
}
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct TimeLocation {
	pub input: TimeLocationInput,
	pub status: String,
	#[serde(deserialize_with = "null_default")]
	pub candidates: Vec<TimeLocationCandidate>,
	pub truncated: bool,
	pub source: String,
}

// Industry names for the existing US NAICS response contract.
pub type Industry = Naics;
pub type IndustryChild = NaicsChild;
pub type IndustryCorrection = NaicsCorrection;
pub type IndustryDeep = NaicsDeep;
pub type IndustryExclusion = NaicsExclusion;
pub type IndustryMatch = NaicsMatch;
pub type IndustrySearch = NaicsSearch;
pub type IndustrySearchResult = NaicsSearchResult;














/// Card-prefix reference data. None means unknown.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Bin {
	pub bin: String,
	/// Actual longest matched prefix, which may be shorter than the input.
	pub prefix: Option<String>,
	pub country: Option<String>,
	pub issuer: Option<String>,
	pub brand: Option<String>,
	pub brand_name: Option<String>,
	pub r#type: Option<String>,
	pub prepaid: Option<bool>,
	pub deep: Option<serde_json::Value>,
}





#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct EmojiBrowse {
	pub total: u32,
	pub next_offset: Option<u32>,
	#[serde(default, deserialize_with = "null_default")]
	pub emojis: Vec<Emoji>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct EmojiVariant {
	pub emoji: String,
	#[serde(default, deserialize_with = "null_default")]
	pub tones: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct EmojiCategory {
	pub category: String,
	#[serde(default, deserialize_with = "null_default")]
	pub subcategories: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct EmojiCategories {
	#[serde(default, deserialize_with = "null_default")]
	pub categories: Vec<EmojiCategory>,
}

/// UTF-16 code-unit positions. End is exclusive.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct EmojiPosition { pub start: u32, pub end: u32 }

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct EmojiParseItem {
	pub emoji: String,
	pub name: String,
	pub category: Option<String>,
	pub deep: Option<EmojiDeep>,
	#[serde(default, deserialize_with = "null_default")]
	pub shortcodes: Vec<String>,
	#[serde(default, deserialize_with = "null_default")]
	pub positions: Vec<EmojiPosition>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct EmojiParse {
	#[serde(default, deserialize_with = "null_default")]
	pub emojis: Vec<EmojiParseItem>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct EmojiArtwork {
	pub license: Option<String>,
	pub license_url: Option<String>,
	pub attribution: Option<String>,
	pub svg: Option<String>,
	#[serde(default, deserialize_with = "null_default")]
	pub png: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct EmojiCompleteness {
	pub missing_count: u32,
	pub unexpected_count: u32,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct EmojiSource {
	pub id: String,
	pub version: Option<String>,
	pub source_url: String,
	pub license: String,
	pub license_url: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct EmojiNotice {
	pub id: String,
	pub license: String,
	pub notice: String,
}



#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct EmojiArtworkCoverage {
	pub svg: u32,
	pub png: u32,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct EmojiFlagLicense {
	pub license: String,
	pub license_url: String,
	pub description: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct EmojiArtworkSource {
	pub style: String,
	pub version: Option<String>,
	pub unicode_version: Option<String>,
	pub source_url: String,
	pub license: String,
	pub license_url: String,
	pub attribution: String,
	pub loaded_coverage: Option<EmojiArtworkCoverage>,
	pub flag_license: Option<EmojiFlagLicense>,
	pub notices: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct EmojiMetadata {
 pub shortcode_sources: Option<std::collections::HashMap<String,EmojiShortcodeSourceMetadata>>,
	pub edition: String,
	pub max_version: Option<String>,
	pub source_version: String,
	pub total: u32,
	pub completeness: EmojiCompleteness,
	#[serde(default, deserialize_with = "null_default")]
	pub sources: Vec<EmojiSource>,
	#[serde(default, deserialize_with = "null_default")]
	pub artwork_sources: Vec<EmojiArtworkSource>,
	#[serde(default, deserialize_with = "null_default")]
	pub notices: Vec<EmojiNotice>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct EmojiCatalog {
 pub localization: Option<EmojiCatalogLocalization>,
	pub profile: Option<String>,
	pub scope: Option<EmojiCatalogScope>,
	pub search: Option<EmojiCatalogSearchFacts>,
	pub metadata: EmojiMetadata,
	pub total: u32,
	#[serde(default, deserialize_with = "null_default")]
	pub emojis: Vec<Emoji>,
	pub forms: std::collections::HashMap<String, String>,
	pub shortcodes: std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>>,
	pub legacy: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct EmojiCatalogScope {
	pub filters: std::collections::HashMap<String,String>,
	pub complete_inventory: bool,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct EmojiNativeSearchFacts {
 pub order: Option<u32>,
	pub name: Option<String>,
	#[serde(default, deserialize_with = "null_default")]
	pub keywords: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct EmojiCatalogSearchFacts {
 pub localized_keywords: Option<Vec<String>>,
	pub native: std::collections::HashMap<String, EmojiNativeSearchFacts>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct EmojiUnresolved {
	pub emoji: Option<String>,
	pub shortcode: Option<String>,
	pub start: u32,
	pub end: u32,
	pub reason: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct EmojiConversion {
	pub text: String,
	pub unresolved: Vec<EmojiUnresolved>,
}

























































#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct EmojiCatalogLocalization {
 pub requested_locale: Option<String>, pub resolved_locale: Option<String>,
 pub translated_names: u32, pub translated_keywords: u32, pub fallback_names: u32, pub fallback_keywords: u32,
}
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct EmojiShortcodeSourceMetadata {
 pub records: u32, pub missing_records: u32, pub ambiguous_aliases: u32,
}


/// A reviewed immediate parent with nullable display and native names.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CityParent {
	pub id: String,
	pub name: Option<String>,
	pub name_local: Option<String>,
	pub country: String,
	pub state: Option<String>,
	#[serde(rename = "type")]
	pub kind: Option<String>,
	pub grain: Option<String>,
}

/// A page from the curated City collection.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CityList {
	pub object: String,
	pub has_more:Option<bool>,
	pub url:Option<String>,
	pub q: Option<String>,
	pub country: Option<String>,
	pub state: Option<String>,
	#[serde(rename = "type")]
	pub kind: Option<String>,
	pub grain: Option<String>,
	#[serde(default, deserialize_with = "null_default")]
	pub data: Vec<CityResource>,
	pub next_cursor: Option<String>,
}

/// A page of cities around the supplied coordinate.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CityNearbyCoords {
	pub object: String,
	pub has_more:Option<bool>,
	pub url:Option<String>,
	pub latitude: f64,
	pub longitude: f64,
	pub radius: f64,
	pub unit: String,
	#[serde(default, deserialize_with = "null_default")]
	pub data: Vec<CityResource>,
	pub next_cursor: Option<String>,
}

/// Complete City Core facts. Unknown facts remain null; requested detail is optional.
#[derive(Debug,Clone,Deserialize)]
#[non_exhaustive]
pub struct CityResource {
	pub name_romanized: Option<String>,
	pub romanization_language: Option<String>,
 pub r#match: Option<CityMatch>,
 pub id:String, pub object:String, pub name:Option<String>, pub name_local:Option<String>, pub country:Option<String>, pub country_name:Option<String>, pub state:Option<String>, pub state_name:Option<String>, pub district:Option<String>, pub district_name:Option<String>,
 #[serde(rename="type")] pub kind:Option<String>,
 pub grain:Option<String>, pub latitude:Option<f64>, pub longitude:Option<f64>, pub timezone:Option<String>, pub distance:Option<f64>, pub distance_mi:Option<f64>, pub parent:Option<CityParent>, pub deep:Option<CityDeep>,
}

/// Bounded safe identities on HTTP409 ambiguous_city. No Deep fields.
#[derive(Debug,Clone,Deserialize)]
#[non_exhaustive]
pub struct CityAmbiguityCandidate {
 pub id:String, pub object:String, pub name:Option<String>, pub name_local:Option<String>, pub country:Option<String>, pub country_name:Option<String>, pub state:Option<String>, pub state_name:Option<String>,
 #[serde(rename="type")] pub kind:Option<String>,
}

#[derive(Debug,Clone,Deserialize)]
#[non_exhaustive]
pub struct CityRelations {
 pub object:String, pub data:Vec<CityResource>, pub has_more:Option<bool>, pub url:Option<String>, pub next_cursor:Option<String>, pub city_id:String, pub relationship:String, pub coverage:String, pub parent_id:Option<String>,
}

#[derive(Debug,Clone,Deserialize)]
#[non_exhaustive]
pub struct CityDistance {
 pub object:String, pub from:String, pub to:String, pub distance:Option<f64>, pub distance_mi:Option<f64>, pub method:String,
}

#[derive(Debug,Clone,Deserialize)]
#[non_exhaustive]
pub struct CityMatch { pub r#type:String, pub source:String, pub value:String, pub language:Option<String>, pub edit_distance:Option<u8> }
