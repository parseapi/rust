# Changelog

## 2.6.1 — 2026-11-16

- Existing Email verification and Emoji parse/convert methods now use GET query parameters. Method signatures, response models, API 2.2 selection and no-retry defaults are unchanged.
- Existing API POST requests remain supported for compatibility. Long or URL-incompatible inputs select POST before dispatch; requests are never retried just to switch methods.

## 2.6.0 - 2026-11-16

- State uses `iso_code` with its existing value or null. County and State county items add nullable `iso_code` for the exact place. Other ISO representations and package versions are unchanged.
- Select HTTP API `2.2.0` through the existing client factory. Existing installed clients retain their current contract. This release preserves the reviewed breaking field/read migration and requires matching application updates.
- Add County lookup and State counties methods, canonical County models/options, and `county` / `county_name` / `county_name_local` / `counties` wire names. Historical method/type spellings remain source aliases.
- This is a breaking response/model migration: update District member reads to County names and `capital_of == "district"` checks to `"county"`. Existing published clients and saved account versions are unchanged. Historical payload keys remain readable where supported; the native aliases do not add duplicate JSON fields.

## 2.5.0 — 2026-11-02

- Select API `2.1.0` through the existing client factory. Requested profile detail uses natural field homes instead of a response `.deep` bag.
- **Breaking field-read migration:** update existing `.deep` reads, User-Agent identity objects and Postal/Weather detail reads with your application code. This minor version changes existing field reads; upgrade the SDK and application code together.
- Use explicit Email verification; Carrier/HLR return their complete selected result. Preserve recorded Form/Bulk contracts and earlier saved results.
- Time success returns a compact matched `location`; ambiguity choices remain explicit. Omit unused `resolution`, City match `source`/`edit_distance`, and Holiday JSON calendar fingerprints. Update readers that relied on those removed fields.
- Keep optional fields absent when unrequested/unavailable and preserve `null`, `false` and `0`. See the API 2.1 upgrade section in the README.

## 2.4.3 — 2026-11-01

- Read existing Point electoral detail without losing leading-zero district codes or nullable metadata.
- Read existing metric Point elevation, including known zero and negative values.
- Preserve every released method/model member, Card behavior, Postal and Weather compatibility readers, and API contract selection.
- This compatible public addition uses Billy's finite third-digit release exception; feature classification is retained.

## 2.4.2 — 2026-10-31

- Read compact Postal property-tax estimates without requiring District metadata; retain optional earlier currency and period values.
- Use Weather minute precipitation_rate / precipitation_rate_in names for rates, leaving accumulated History precipitation unchanged.
- Accept earlier Weather minute wire names only when the canonical key is absent; canonical null remains unknown.
- Preserve unavailable Weather collections as None and known empty collections as Some(empty).
- The Postal property-tax type and Weather minute member changes can require source changes in existing typed consumers. Weather collection members now use Option<Vec<_>>.

## 2.3.5 — 2026-10-27

- Add optional Country Deep fields: gdp, gdp_per_capita, population_growth, urban_population and internet_users. GDP uses US dollars; growth and population shares use percent. Missing values remain null.

## 2.3.0 — 2026-10-15

- Add compatible City resource/list, spelling-search, geographic-filter, pagination, hierarchy, relationship and distance support.
- Keep existing City calls and response defaults compatible; expose ambiguity candidates when requested.
- Remove population observation periods from public examples; retain published typed members only for source compatibility.

## 2.2.1 — 2026-10-12

- Add Emoji search/browse, categories, catalog, metadata, parsing and conversion; keep existing lookup calls and response decoding compatible.
- Support optional SVG/PNG artwork and translated names. Preserve API 2.0.0 selection, plans and existing non-Emoji behavior.

## 2.2.0 — 2026-09-29

- Add Company stable-ID lookup, candidate search and coverage for the current public-company cohort.
- Preserve existing Company calls and all published compatibility methods.

## 2.1.0 — 2026-09-25

- Add Bank diagnostics, POST requests, requirements and explicit US ACH helpers; Card Core and optional Deep; Provider; Industry; and Vehicle.
- Preserve published IBAN, BIN, NPI, NAICS and VIN methods and response types alongside the new names.
- Respect long Retry-After responses without retrying early and expose the raw header as optional error metadata.
- Retain released Time, Tariff, Postal and Elevation behavior. Expanded Company directory changes are deferred.

## 2.0.0 - 2026-09-24

- Add exact Tariff edition/date selection, answering metadata, contextual search lineage and explanatory null reasons. Explicit selections reject unsupported or mismatched server responses.
- Breaking: `TariffDeep.measures` is now `Option<Vec<TariffMeasure>>`; handle unresolved `None` separately from a resolved empty list.

## 1.6.0 - 2026-09-24

Adds Time location inputs and explicit ambiguity candidates, filtered timezone discovery, multiple conversion targets, wall-time disambiguation, and standard/seasonal offset detail. Existing Timezone methods and API 2.0.0 selection remain unchanged.

## 1.4.0 - 2026-09-24

Adds Australian Postal suburb choices while preserving null, empty, and ambiguous results. Existing calls, compact nearby/distance responses, and API contract `2.0.0` remain unchanged.

## 1.3.0 - Unreleased

Adds Stack site inventory lookup with eight technology category arrays, nullable versions, and explicit page coverage. Stack uses a 35-second default attempt timeout while explicit caller settings remain honored. Existing calls and API contract `2.0.0` remain unchanged.

## 1.1.0 - 2026-09-20

Email deep results now include nullable suggested first name, no-reply flag, plus-address tag, mail provider, verification status and reason. Existing lookup calls, retry defaults and API contract `2.0.0` remain unchanged. Missing details remain unknown, and suggested names do not verify identity.

Email responses also expose the existing core `free` and `domain_type` fields.
