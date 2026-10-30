use crate::{push_deep, seg, Client, DomainOptions, Query, Result};
use serde::Deserialize;

/// Partial registration detail from the same observation. None means unknown.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct DomainRegistrationResult {
	pub registered: Option<bool>,
	pub created: Option<String>,
	pub updated: Option<String>,
	pub expires: Option<String>,
	pub registrar: Option<String>,
	/// None is unknown; Some(empty) is a known empty list.
	pub status: Option<Vec<String>>,
	pub dnssec: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct DomainResultDeep {
	pub registration: Option<DomainRegistrationResult>,
}

/// Domain availability with an explicit unknown result.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct DomainResult {
	pub domain: String,
	/// Some(true) means unregistered, Some(false) registered, None unknown.
	pub available: Option<bool>,
	pub deep: Option<DomainResultDeep>,
}

impl Client {
	/// Check registration while preserving unknown availability and partial detail.
	/// Uses the same lookup and options as domain.
	pub async fn domain_result(
		&self,
		domain: &str,
		opts: impl Into<Option<DomainOptions>>,
	) -> Result<DomainResult> {
		let opts = opts.into().unwrap_or_default();
		let mut query = Query::new();
		push_deep(&mut query, opts.deep);
		self.get(&format!("/domain/{}", seg(domain)), query, None).await
	}
}
