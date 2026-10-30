use serde::Deserialize;

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
