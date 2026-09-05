//! Response types for the parseAPI public API. Fields are appended as the API grows. Nullable fields are `Option`.
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
pub struct Bloc {
	pub bloc: String,
	pub name: String,
	pub members: i32,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct BlocCountryItem {
	pub country: String,
	pub name: String,
	pub emoji: Option<String>,
	pub calling_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct BlocCountries {
	pub bloc: String,
	#[serde(default, deserialize_with = "null_default")]
	pub countries: Vec<BlocCountryItem>,
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
	#[serde(default, deserialize_with = "null_default")]
	pub blocs: Vec<String>,
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
	#[serde(default, deserialize_with = "null_default")]
	pub timezones: Vec<String>,
	pub iso_3166_2: Option<String>,
	pub fips: Option<String>,
	pub capital: Option<String>,
	#[serde(default, deserialize_with = "null_default")]
	pub area_codes: Vec<String>,
	pub tax: Option<String>,
	pub tax_rate: Option<f64>,
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
	/// Total area in km2 (land + water, or the official total).
	pub area: Option<f64>,
	/// Land area in km2. None when the source publishes total only.
	pub land_area: Option<f64>,
	/// Water area in km2. None when the source publishes total only.
	pub water_area: Option<f64>,
	pub seat: Option<String>,
	pub timezone: Option<String>,
	#[serde(default, deserialize_with = "null_default")]
	pub timezones: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct City {
	pub name: String,
	pub local_name: Option<String>,
	#[serde(rename = "type")]
	pub kind: Option<String>,
	/// What this city is the capital of: country, state, or none.
	pub capital_of: Option<String>,
	pub state: Option<String>,
	pub state_name: Option<String>,
	pub district: Option<String>,
	pub district_name: Option<String>,
	pub country: String,
	pub country_name: Option<String>,
	pub latitude: Option<f64>,
	pub longitude: Option<f64>,
	pub elevation: Option<f64>,
	pub elevation_ft: Option<f64>,
	pub population: Option<i64>,
	/// Total area in km2 (land + water, or the official total).
	pub area: Option<f64>,
	/// Land area in km2. None when the source publishes total only.
	pub land_area: Option<f64>,
	/// Water area in km2. None when the source publishes total only.
	pub water_area: Option<f64>,
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
pub struct CityNearby {
	pub city: String,
	pub state: Option<String>,
	pub country: String,
	pub radius: f64,
	pub unit: String,
	#[serde(default, deserialize_with = "null_default")]
	pub nearby: Vec<CityNearest>,
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
	pub country_name: Option<String>,
	pub latitude: Option<f64>,
	pub longitude: Option<f64>,
	pub elevation: Option<f64>,
	pub elevation_ft: Option<f64>,
	pub population: Option<i64>,
	/// Total area in km2. None when the source has no water split.
	pub area: Option<f64>,
	/// Land area in km2, where the source has it.
	pub land_area: Option<f64>,
	/// Water area in km2, where the source has it.
	pub water_area: Option<f64>,
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
	pub didyoumean: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct VatAddress {
	pub street: Option<String>,
	pub city: Option<String>,
	pub postal: Option<String>,
	pub country: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct VatDeep {
	pub registered: Option<bool>,
	pub name: Option<String>,
	pub address: Option<VatAddress>,
	pub consultation: Option<String>,
	/// Registry timestamp of this check, ISO.
	pub consulted_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Vat {
	pub vat: Option<String>,
	pub valid: bool,
	pub country: Option<String>,
	pub from: Option<String>,
	pub deep: Option<VatDeep>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Iban {
	pub iban: Option<String>,
	pub valid: bool,
	pub country: Option<String>,
	/// Print form in groups of four, for display. None when invalid.
	pub formatted: Option<String>,
	pub checksum: Option<String>,
	/// Bank identifier parsed from the number, not a name.
	pub bank: Option<String>,
	/// Institution name from the national bank-code directory. None when unsourced.
	pub bank_name: Option<String>,
	/// BIC from that same directory. None when unsourced or missing.
	pub bic: Option<String>,
	/// Branch identifier when that country has one.
	pub branch: Option<String>,
	pub account: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Npi {
	/// Normalized 10-digit NPI. Invalid input still echoes the fold.
	pub npi: Option<String>,
	pub valid: bool,
	/// Exists in the healthcare provider registry.
	pub registered: Option<bool>,
	pub active: Option<bool>,
	/// Date the NPI was deactivated, YYYY-MM-DD. None when still active.
	pub deactivated_at: Option<String>,
	/// On the OIG exclusion list.
	pub excluded: Option<bool>,
	/// individual or organization.
	#[serde(rename = "type")]
	pub kind: Option<String>,
	pub name: Option<String>,
	pub first: Option<String>,
	pub last: Option<String>,
	pub credential: Option<String>,
	pub specialty: Option<String>,
	/// NUCC taxonomy code.
	pub taxonomy: Option<String>,
	pub address: Option<String>,
	pub city: Option<String>,
	pub state: Option<String>,
	pub state_name: Option<String>,
	pub postal: Option<String>,
	pub country: Option<String>,
	pub phone: Option<String>,
	pub deep: Option<NpiDeep>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct NpiEnrollment {
	/// part_a, part_b, practitioner, dme, order_refer, mdpp. None when unknown.
	#[serde(rename = "type")]
	pub kind: Option<String>,
	pub specialty: Option<String>,
	pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct NpiDeep {
	/// In the published Medicare FFS enrollment extract.
	pub medicare: Option<bool>,
	/// Has a Medicare opt-out affidavit.
	pub opt_out: Option<bool>,
	/// Enrollment rows. Empty when medicare is false.
	pub enrollments: Option<Vec<NpiEnrollment>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct VinRecall {
	/// Government campaign number.
	pub campaign: String,
	/// Report date, ISO YYYY-MM-DD.
	pub date: Option<String>,
	pub component: Option<String>,
	/// The filed summary verbatim.
	pub summary: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct TariffMeasure {
	/// Chapter 99 heading, dotted (9903.01.24).
	pub heading: String,
	/// The measure text verbatim.
	pub description: String,
	/// The rate string verbatim.
	pub rate: Option<String>,
	/// Effective from, ISO YYYY-MM-DD. None when the schedule states none.
	pub from: Option<String>,
	/// Expires, ISO YYYY-MM-DD. None when open-ended.
	pub until: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct TariffDeep {
	/// The origin country the measures were resolved for.
	pub origin: Option<String>,
	/// Composed ad valorem percent. None when the components do not compose cleanly.
	pub effective_rate: Option<f64>,
	/// Every Chapter 99 tariff measure that applies to this code from this origin.
	#[serde(default, deserialize_with = "null_default")]
	pub measures: Vec<TariffMeasure>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Tariff {
	/// Normalized code with dots (8471.30.01.00).
	pub hts: String,
	/// The schedule line verbatim.
	pub description: String,
	/// Parent descriptions from the schedule outline, outermost first.
	#[serde(default, deserialize_with = "null_default")]
	pub lineage: Vec<String>,
	/// Units of quantity (No., kg).
	#[serde(default, deserialize_with = "null_default")]
	pub units: Vec<String>,
	/// Column 1 general rate, verbatim.
	pub general: Option<String>,
	/// Column 1 special rate, verbatim.
	pub special: Option<String>,
	/// Column 2 rate, verbatim.
	pub other: Option<String>,
	/// The official release that answered (2026HTSRev17).
	pub revision: String,
	pub deep: Option<TariffDeep>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct TariffSearchHit {
	pub hts: String,
	pub description: String,
	pub general: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct TariffSearch {
	pub q: String,
	pub revision: String,
	/// Up to 20 tariff lines, best match first.
	#[serde(default, deserialize_with = "null_default")]
	pub lines: Vec<TariffSearchHit>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct VinDeep {
	/// Open recall campaigns for the decoded vehicle. Empty when none,
	/// None when the recall registry did not answer.
	pub recalls: Option<Vec<VinRecall>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Vin {
	/// Normalized VIN, uppercase, no spaces. Invalid input still echoes the fold.
	pub vin: Option<String>,
	pub valid: bool,
	pub year: Option<i64>,
	pub make: Option<String>,
	pub model: Option<String>,
	pub trim: Option<String>,
	pub series: Option<String>,
	/// Body style (sedan, coupe, suv, pickup).
	pub body: Option<String>,
	/// Vehicle type (passenger car, truck, motorcycle, bus, trailer).
	#[serde(rename = "type")]
	pub kind: Option<String>,
	pub doors: Option<i64>,
	pub cylinders: Option<i64>,
	/// Engine displacement in liters.
	pub displacement: Option<f64>,
	pub fuel: Option<String>,
	pub horsepower: Option<f64>,
	/// fwd, rwd, awd, 4wd.
	pub drive: Option<String>,
	/// automatic, manual, cvt.
	pub transmission: Option<String>,
	pub manufacturer: Option<String>,
	pub plant_city: Option<String>,
	pub plant_state: Option<String>,
	pub plant_country: Option<String>,
	/// Gross vehicle weight rating class as filed.
	pub gvwr: Option<String>,
	pub deep: Option<VinDeep>,
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
	/// NPA-derived state code (US/CA).
	pub state: Option<String>,
	pub state_name: Option<String>,
	/// Numbering-plan IANA zone. None when the prefix covers more than one zone.
	pub timezone: Option<String>,
	pub national: Option<String>,
	pub international: Option<String>,
	/// Always empty. The metered proves are their own endpoints: carrier, caller, hlr.
	pub deep: Option<serde_json::Value>,
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
	/// Issuing rate-center city.
	pub city: Option<String>,
	pub state: Option<String>,
	pub state_name: Option<String>,
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
	/// Assigned to a subscriber.
	pub live: Option<bool>,
	/// Handset reachable right now. None means unconfirmed, never no.
	pub connected: Option<bool>,
	/// The six network extras fill on live HLR dips only. None elsewhere (NANP, failover).
	pub roaming: Option<bool>,
	pub roaming_network: Option<String>,
	/// ISO2, uppercase.
	pub roaming_country: Option<String>,
	/// Current serving network name.
	pub network: Option<String>,
	pub original_network: Option<String>,
	pub mcc: Option<String>,
	pub mnc: Option<String>,
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
pub struct Asn {
	pub asn: u32,
	pub name: Option<String>,
	pub country: Option<String>,
	pub country_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Mac {
	pub mac: String,
	pub valid: bool,
	pub vendor: Option<String>,
	pub local: Option<bool>,
	pub multicast: Option<bool>,
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
	pub amount: Option<f64>,
	pub converted: Option<f64>,
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
	pub timezone: Option<String>,
	pub name: Option<String>,
	pub abbreviation: Option<String>,
	pub offset: Option<String>,
	pub offset_minutes: Option<i32>,
	pub dst: Option<bool>,
	pub next_dst: Option<TimezoneNextDst>,
	pub latitude: Option<f64>,
	pub longitude: Option<f64>,
	pub at: Option<String>,
	pub to: Option<TimezoneConversionTarget>,
}

/// Calendar facts for a date. Calendar fields are None when valid is false.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct DateInfo {
	pub date: String,
	pub valid: bool,
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
	pub unix: Option<i64>,
	pub to: Option<String>,
	pub days: Option<i32>,
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
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct WeatherCurrent {
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
pub struct WeatherSource {
	pub id: String,
	pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct Weather {
	pub latitude: f64,
	pub longitude: f64,
	pub current: WeatherCurrent,
	pub station: Option<WeatherStation>,
	pub source: WeatherSource,
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

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct TimezoneConversionTarget {
	pub timezone: String,
	pub name: Option<String>,
	pub abbreviation: Option<String>,
	pub offset: String,
	pub offset_minutes: i32,
	pub dst: bool,
	pub at: String,
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
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CompanyCountry {
	pub name: Option<String>,
	#[serde(default, deserialize_with = "null_default")]
	pub blocs: Vec<String>,
	pub tax: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CompanyDeep {
	pub country: Option<CompanyCountry>,
	pub postal: Option<Postal>,
	pub city: Option<City>,
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
	pub activity: Option<String>,
	pub address: Option<String>,
	pub city: Option<String>,
	pub state: Option<String>,
	pub state_name: Option<String>,
	pub postal: Option<String>,
	pub country_name: Option<String>,
	pub vat: Option<String>,
	pub gst: Option<bool>,
	pub acn: Option<String>,
	pub siren: Option<String>,
	pub siege: Option<bool>,
	pub kind: Option<String>,
	pub invoice: Option<String>,
	pub deep: Option<CompanyDeep>,
}
