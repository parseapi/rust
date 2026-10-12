//! Currency reference discovery, bounded history and period reports.
use crate::{push, seg, Client, Error, Query, Result};
use serde::Deserialize;
use std::collections::{BTreeMap, HashSet};

fn null_default<'de, D, T>(deserializer: D) -> std::result::Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CurrencyRateDates {
    pub base: Option<Vec<String>>,
    pub quote: Option<Vec<String>>,
}
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CurrencyRateUnits {
    pub base: Option<String>,
    pub quote: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CurrencyRateMissing {
    pub quote: String,
    pub reason: String,
    pub requested_date: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CurrencyQuote {
    pub rate: f64,
    pub date: String,
    pub converted: Option<f64>,
    pub stale: Option<bool>,
    pub dates: Option<CurrencyRateDates>,
    pub units: Option<CurrencyRateUnits>,
}
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CurrencyObservation {
    pub requested_date: String,
    pub rate: f64,
    pub date: String,
    pub converted: Option<f64>,
    pub stale: Option<bool>,
    pub dates: Option<CurrencyRateDates>,
    pub units: Option<CurrencyRateUnits>,
}
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CurrencyQuoteObservation {
    pub quote: String,
    pub requested_date: String,
    pub rate: f64,
    pub date: String,
    pub converted: Option<f64>,
    pub stale: Option<bool>,
    pub dates: Option<CurrencyRateDates>,
    pub units: Option<CurrencyRateUnits>,
}
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CurrencyRates {
    pub base: String,
    pub amount: Option<f64>,
    pub requested_date: Option<String>,
    #[serde(deserialize_with = "null_default")]
    pub rates: BTreeMap<String, CurrencyQuote>,
    #[serde(deserialize_with = "null_default")]
    pub missing: Vec<CurrencyRateMissing>,
}
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CurrencyHistory {
    pub base: String,
    pub quote: String,
    pub from: String,
    pub to: String,
    pub amount: Option<f64>,
    #[serde(deserialize_with = "null_default")]
    pub data: Vec<CurrencyObservation>,
    #[serde(deserialize_with = "null_default")]
    pub missing: Vec<String>,
    pub interval: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CurrencyRatesHistory {
    pub base: String,
    pub from: String,
    pub to: String,
    pub amount: Option<f64>,
    #[serde(deserialize_with = "null_default")]
    pub quotes: Vec<String>,
    #[serde(deserialize_with = "null_default")]
    pub data: Vec<CurrencyQuoteObservation>,
    #[serde(deserialize_with = "null_default")]
    pub missing: Vec<CurrencyRateMissing>,
    pub interval: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CurrencySummaryRow {
    pub quote: String,
    pub from: String,
    pub to: String,
    pub average: f64,
    pub min: f64,
    pub max: f64,
    pub observation_count: u64,
    pub first_date: String,
    pub last_date: String,
    pub missing_day_count: u64,
    pub stale: Option<bool>,
    pub units: Option<CurrencyRateUnits>,
}
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CurrencySummaryMissing {
    pub quote: String,
    pub from: String,
    pub to: String,
    pub reason: String,
    pub missing_day_count: u64,
}
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CurrencyRatesSummary {
    pub base: String,
    pub from: String,
    pub to: String,
    pub interval: String,
    #[serde(deserialize_with = "null_default")]
    pub quotes: Vec<String>,
    #[serde(deserialize_with = "null_default")]
    pub data: Vec<CurrencySummaryRow>,
    #[serde(deserialize_with = "null_default")]
    pub missing: Vec<CurrencySummaryMissing>,
}

/// Period reports count original publication days once. Amount and deep are not report controls.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct CurrencySummaryOptions {
    pub interval: Option<String>,
}
impl CurrencySummaryOptions {
    pub fn interval(mut self, value: impl Into<String>) -> Self {
        self.interval = Some(value.into());
        self
    }
}


/// Configures one bounded quote batch. Amount and converted values are JSON numbers.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct CurrencyRatesOptions {
    pub date: Option<String>,
    pub amount: Option<f64>,

}
impl CurrencyRatesOptions {
    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }
    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

}

/// Configures pair history and its CSV representation.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct CurrencyHistoryOptions {
    pub amount: Option<f64>,
    pub interval: Option<String>,
}
impl CurrencyHistoryOptions {
    pub fn interval(mut self, value: impl Into<String>) -> Self {
        self.interval = Some(value.into());
        self
    }
    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

}

/// Configures multi-quote history and its CSV representation.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct CurrencyRatesHistoryOptions {
    pub amount: Option<f64>,
    pub interval: Option<String>,
}
impl CurrencyRatesHistoryOptions {
    pub fn interval(mut self, value: impl Into<String>) -> Self {
        self.interval = Some(value.into());
        self
    }
    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

}

fn controls(amount: Option<f64>) -> Result<Query> {
    if amount.is_some_and(|value| !value.is_finite() || value.abs() > 1e15) {
        return Err(Error::Config(
            "Currency amount must be finite and within +/-1e15.".into(),
        ));
    }
    let mut query = Query::new();
    push(&mut query, "amount", amount.map(|value| value.to_string()));
    Ok(query)
}

fn quotes<I, S>(input: I) -> Result<Vec<String>>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut result = Vec::new();
    let mut seen = HashSet::new();
    for value in input {
        let code = value.as_ref().trim();
        if result.len() >= 50
            || code.len() != 3
            || !code.bytes().all(|byte| byte.is_ascii_alphabetic())
            || !seen.insert(code.to_ascii_uppercase())
        {
            return Err(Error::Config(
                "Currency quotes requires 1 to 50 distinct three-letter codes.".into(),
            ));
        }
        result.push(code.to_owned());
    }
    if result.is_empty() {
        return Err(Error::Config(
            "Currency quotes requires 1 to 50 distinct three-letter codes.".into(),
        ));
    }
    Ok(result)
}

fn ordinal(day: &str) -> Option<u64> {
    let bytes = day.as_bytes();
    if bytes.len() != 10
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || bytes
            .iter()
            .enumerate()
            .any(|(index, byte)| index != 4 && index != 7 && !byte.is_ascii_digit())
    {
        return None;
    }
    let year: u64 = day[..4].parse().ok()?;
    let month: usize = day[5..7].parse().ok()?;
    let date: u64 = day[8..].parse().ok()?;
    if year < 1000 || !(1..=12).contains(&month) {
        return None;
    }
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let lengths = [
        31,
        if leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    if date == 0 || date > lengths[month - 1] {
        return None;
    }
    let previous = year - 1;
    Some(
        previous * 365 + previous / 4 - previous / 100
            + previous / 400
            + lengths[..month - 1].iter().sum::<u64>()
            + date,
    )
}

fn day(day: &str) -> Result<u64> {
    let today = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        / 86400
        + ordinal("1970-01-01").unwrap();
    ordinal(day).filter(|value| *value <= today).ok_or_else(|| {
        Error::Config("Currency dates must be real UTC days from year 1000 through today.".into())
    })
}

fn range(from: &str, to: &str, count: usize) -> Result<()> {
    range_with_max(from, to, count, 366)
}

fn range_with_max(from: &str, to: &str, count: usize, max_days: u64) -> Result<()> {
    let first = day(from)?;
    let last = day(to)?;
    if last < first || last - first + 1 > max_days || (last - first + 1) * count as u64 > 5000 {
        return Err(Error::Config(format!(
            "Currency history requires 1 to {max_days} days and at most 5000 quote-days."
        )));
    }
    Ok(())
}

// Daily defaults preserve existing local limits. The API owns sampled response/read budgets.
fn history_range(from: &str, to: &str, count: usize, interval: Option<&str>) -> Result<()> {
    match interval.unwrap_or("day") {
        "day" => range(from, to, count),
        "week" | "month" => report_range(from, to),
        _ => Err(Error::Config(
            "Currency history interval must be day, week or month.".into(),
        )),
    }
}
fn report_range(from: &str, to: &str) -> Result<()> {
    let first = day(from)?;
    let last = day(to)?;
    if last < first || last - first + 1 > 36_600 {
        return Err(Error::Config(
            "Currency sampled history and reports require 1 to 36600 calendar days.".into(),
        ));
    }
    Ok(())
}
fn summary_interval(value: Option<&str>) -> Result<()> {
    if !["month", "quarter", "year"].contains(&value.unwrap_or("month")) {
        return Err(Error::Config(
            "Currency summary interval must be month, quarter or year.".into(),
        ));
    }
    Ok(())
}

/// Maintained metadata catalog, distinct from rate coverage.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CurrencyResource {
    pub currency: String,
    pub name: String,
    pub symbol: Option<String>,
    pub symbol_native: Option<String>,
    pub digits: Option<i32>,
    pub deep: Option<crate::CurrencyDeep>,
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CurrencyList {
    #[serde(deserialize_with = "null_default")]
    pub data: Vec<CurrencyResource>,

}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct CurrencyListOptions {
    pub q: Option<String>,
    pub country: Option<String>,
    pub deep: bool,
    /// Display language for this request.
    pub lang: Option<String>,
}
impl CurrencyListOptions {
    pub fn lang(mut self, value: impl Into<String>) -> Self {
        self.lang = Some(value.into());
        self
    }
    pub fn q(mut self, value: impl Into<String>) -> Self {
        self.q = Some(value.into());
        self
    }
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }
    pub fn deep(mut self, value: bool) -> Self {
        self.deep = value;
        self
    }
}

impl Client {

    pub async fn currency_rates<I, S>(
        &self,
        base: &str,
        input: I,
        opts: impl Into<Option<CurrencyRatesOptions>>,
    ) -> Result<CurrencyRates>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let quotes = quotes(input)?;
        let opts = opts.into().unwrap_or_default();
        if let Some(value) = &opts.date {
            day(value)?;
        }
        let mut query = vec![("quotes", quotes.join(","))];
        push(&mut query, "date", opts.date);
        query.extend(controls(opts.amount)?);
        self.get(&format!("/currency/{}/rates", seg(base)), query, None)
            .await
    }

    /// Complete maintained latest rates. Existing subset signature is unchanged.
    pub async fn currency_rates_all(
        &self,
        base: &str,
        opts: impl Into<Option<CurrencyRatesOptions>>,
    ) -> Result<CurrencyRates> {
        let opts = opts.into().unwrap_or_default();
        if let Some(value) = &opts.date {
            day(value)?;
        }
        let mut query = controls(opts.amount)?;
        push(&mut query, "date", opts.date);
        self.get(&format!("/currency/{}/rates", seg(base)), query, None)
            .await
    }

    pub async fn currency_list(
        &self,
        opts: impl Into<Option<CurrencyListOptions>>,
    ) -> Result<CurrencyList> {
        let opts = opts.into().unwrap_or_default();
        let mut query = Query::new();
        push(&mut query, "q", opts.q);
        push(&mut query, "country", opts.country);
        push(&mut query, "deep", opts.deep.then(|| "true".into()));
        push(&mut query, "lang", opts.lang);
        self.get("/currency", query, None).await
    }

    pub async fn currency_history(
        &self,
        base: &str,
        quote: &str,
        from: &str,
        to: &str,
        opts: impl Into<Option<CurrencyHistoryOptions>>,
    ) -> Result<CurrencyHistory> {
        let opts = opts.into().unwrap_or_default();
        history_range(from, to, 1, opts.interval.as_deref())?;
        let mut query = vec![("from", from.into()), ("to", to.into())];
        query.extend(controls(opts.amount)?);
        push(&mut query, "interval", opts.interval);
        self.get(
            &format!("/currency/{}/{}/history", seg(base), seg(quote)),
            query,
            None,
        )
        .await
    }

    pub async fn currency_rates_history<I, S>(
        &self,
        base: &str,
        input: I,
        from: &str,
        to: &str,
        opts: impl Into<Option<CurrencyRatesHistoryOptions>>,
    ) -> Result<CurrencyRatesHistory>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let quotes = quotes(input)?;
        let opts = opts.into().unwrap_or_default();
        history_range(from, to, quotes.len(), opts.interval.as_deref())?;
        let mut query = vec![
            ("from", from.into()),
            ("to", to.into()),
            ("quotes", quotes.join(",")),
        ];
        query.extend(controls(opts.amount)?);
        push(&mut query, "interval", opts.interval);
        self.get(
            &format!("/currency/{}/rates/history", seg(base)),
            query,
            None,
        )
        .await
    }



    /// Original observations summarized by month (default), quarter or year.
    pub async fn currency_rates_summary<I, S>(
        &self,
        base: &str,
        input: I,
        from: &str,
        to: &str,
        opts: impl Into<Option<CurrencySummaryOptions>>,
    ) -> Result<CurrencyRatesSummary>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let quotes = quotes(input)?;
        report_range(from, to)?;
        let opts = opts.into().unwrap_or_default();
        summary_interval(opts.interval.as_deref())?;
        let mut query = vec![
            ("from", from.into()),
            ("to", to.into()),
            ("quotes", quotes.join(",")),
        ];
        query.extend(controls(None)?);
        push(&mut query, "interval", opts.interval);
        self.get(
            &format!("/currency/{}/rates/summary", seg(base)),
            query,
            None,
        )
        .await
    }

}
