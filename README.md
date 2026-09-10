```bash
cargo add parseapi
cargo add tokio --features macros,rt-multi-thread
```

```rust
#[tokio::main]
async fn main() -> Result<(), parseapi::Error> {
    let parse = parseapi::Client::new("your-api-key")?;
    let country = parse.country("US").await?;
    println!("{}", country.name);
    Ok(())
}
```

Get a key at [parseapi.com](https://parseapi.com). `Client::from_env()` reads `PARSEAPI_KEY`. An empty explicit key also uses that environment variable.

## API versions

Choose your team's API version in [Dashboard → API version](https://parseapi.com/dashboard/versions). One setting applies to every key, including new and replacement keys. Existing teams keep `1.0.0`; new teams start on `2.0.0`. Keep the same keys and lookup URLs. Installing or upgrading the package does not change the team's setting.

Published SDK `0.3.2` matches API `1.0.0`. The examples and response types in this source tree target API `2.0.0`, including changes that are not in `0.3.2`. Use a package release documented for your team's version. These types do not model every historical response; moving to `2.0.0` may require updating code that reads renamed, moved or removed fields.

Test the target contract in a separate development team before changing your production team's version. A change applies to every integration in that team. See [API versions and migration](https://parseapi.com/docs/versioning).

## Weather from a postal code

Start with the postal code, then pass its coordinates to weather. Reuse the client from the example above.

```rust
let place = parse.postal(
    "28202", parseapi::PostalOptions::default().country("US"),
).await?;
if let (Some(lat), Some(lon)) = (place.latitude, place.longitude) {
    let weather = parse.weather(lat, lon, None).await?;
    println!("{weather:?}");
}
```

The coordinates represent the postal area. Weather is for that point. Missing coordinates skip the weather lookup. This composition performs two ordinary lookups when coordinates are available, with the retry policy below.

Place this inside the async `main` above, before `Ok(())`.

## Supply the context you know

Pass `country` when a postal code or national phone number needs disambiguation. A complete international phone number already carries its country context. For a numeric date such as `03/04/2026`, supply the intended `format`. Defaults resolve what the input establishes. Ambiguous input needs your context.

Results are plain data. Pass a returned code or coordinate to another operation when the task needs it. Check nullable values before composing the next call.

## Calls

Choose the operation and pass what you have. Related operations are separate direct calls, and results are plain typed data.

```rust
let country = parse.country("US").await?;
let states = parse.country_states("US").await?;
let postal = parse.postal("28202", parseapi::PostalOptions::default().country("US")).await?;
let phone = parse.phone("+14155552671", None).await?;
```

`None` uses the defaults. Every operation with optional query inputs has its own options type. Start with `default()` and use its setters. Options and response structs are non-exhaustive so fields can be added without changing existing calls. Methods with no optional inputs keep their short signatures. Future options for those methods require an additive operation or builder, preserving the original method and argument count.

```rust
use parseapi::*;

parse.ip("8.8.8.8", IpOptions::default().deep(true)).await?;
parse.email("hello@example.com", EmailOptions::default().deep(true)).await?;
parse.vat("DE136695976", VatOptions::default().deep(true)).await?;
parse.iban("DE89370400440532013000", None).await?;
parse.bin("424242", None).await?;
parse.npi("1881018208", None).await?;
parse.asn("AS13335").await?;
parse.mac("00:1B:63:84:45:E6").await?;
parse.name("Andrea").await?;
parse.name_with_options("Andrea", NameOptions::default().country("IT")).await?;
parse.vin("1HGCM82633A004352", None).await?;
parse.carrier("+14155552671", None).await?;
parse.caller("+18004633339", None).await?;
parse.hlr("+447712345678", None).await?;
parse.dns("example.com", None).await?;
parse.dns("_dmarc.example.com", DnsOptions::default().r#type("TXT")).await?;
parse.naics("541511").await?;
parse.naics_search("coffee shop", NaicsSearchOptions::default().limit(5)).await?;
parse.tariff("8471.30.01.00", TariffOptions::default().origin("DE").deep(true)).await?;
parse.address("123 Main St", AddressOptions::default().country("US")).await?;
parse.address_search("123 Main", AddressSearchOptions::default().country("US").state("NC")).await?;
parse.company("123456789", CompanyOptions::default().country("FR")).await?;
parse.date("03/04/2026", DateOptions::default().format("mdy")).await?;
parse.date_today(DateTodayOptions::default().to("2026-12-25")).await?;
parse.time("", None).await?; // UTC now
parse.time("America/New_York", TimeOptions::default().at("2026-09-05T15:00:00").to("Asia/Tokyo")).await?;
parse.time_at(40.7128, -74.006, None).await?;
parse.weather(40.7128, -74.006, WeatherOptions::default().deep(true).date("2026-09-01")).await?;
```

Paid NAICS `deep` includes full definitions, child categories and classification `exclusions`, each with a description and linked codes. Generic exclusions can have no linked codes. Omitted or null exclusions in older responses remain unknown. Search results keep `country` and `year` on the envelope and optional depth on each result. They also include core `match`: the matched `field` (`name`, `term` or `naics`) and `text`, plus `corrections` with `from` and `to` tokens for typo fallback. Corrections are empty for exact, plural and prefix matches. Direct code lookups omit `match`. Older responses may omit it.

Nullable values use `Option`. Unknown JSON fields are accepted. An omitted `deep` is `None`, and a requested empty `deep` is `Some` with empty fields. Optional deep arrays preserve unknown versus an empty result. Existing core collections retain their documented null-to-empty normalization. API fields named `type` use `r#type` where a separate `kind` field also exists.

DNS uses pooled requests on every plan. Omit `type` to check A, AAAA, CNAME, MX, NS, TXT, SOA, CAA, SRV and PTR. Records contain `name`, `type`, `ttl` in seconds and a DNS presentation `value`. TXT values retain quoting and chunk boundaries. A selected question can include its CNAME chain. Empty records mean no records. Lookup failures remain errors.

## Time

`time` returns local ISO `at` with its UTC offset and integer Unix seconds in `unix`. Request `deep` for the display name, exact `offset_seconds`, whole `offset_minutes` and next clock change. A conversion target has its own optional `deep` without a next-change field. Historical offsets and ISO times can include offset seconds. Omitted `at` means now. With `to`, an offsetless `at` is source wall time. Otherwise it is UTC. Include an offset for repeated local times around a clock change. Current time and conversion use pooled requests on every plan. Coordinate clock fields can be null when the timezone is unknown. Existing `timezone` methods remain supported.

## Measurements

```rust
let result = parse.measure("5 ft 11 in", parseapi::MeasureOptions::default().to("cm")).await?;
let units = parse.measure_units(parseapi::MeasureUnitsOptions::default().unit("m")).await?;
```

`amount` is a decimal string, such as `"180.34"`. Without `to`, the API returns the canonical unit for the measurement type. Pass `locale` for number formatting and `system` (`us` or `imperial`) when a customary unit needs context. Ambiguous input returns `valid: false`, a `reason`, and available `choices`. Invalid or incompatible target units use the normal API error.

Unit discovery accepts optional `query`, `type`, and `unit` filters. `unit` selects compatible targets. Omit the filters for the reviewed catalog. Both operations use pooled requests.

## Place statistics and optional detail

Postal and District paid profiles include `deep.property_tax` where supported. It contains `annual_median`, `currency` and `period`: median annual property tax payable on owner-occupied homes in the statistical area. The amount is adjusted to the final year of the reporting period (`YYYY-YYYY`). This is an area statistic, not a rate or an individual property bill. Unsupported, missing and censored estimates are null.

```rust
let place = parse.postal("28202", PostalOptions::default().country("US").deep(true)).await?;
let property_tax = place.deep.as_ref().and_then(|deep| deep.property_tax.as_ref());
```

Read `population_period` alongside `population`: a reporting year (`YYYY`) or period (`YYYY-YYYY`), null when unknown or unverifiable. Keep missing or null values unknown and preserve a known zero. These fields belong to full place profiles. State district lists include each district's population and period. Postal nearby and distance detail remains metropolitan associations only. Continent population and its period remain in core.

Point returns the timezone ID with the core location. Its optional deep detail adds terrain and compact nearest-city context on every plan. A nearest city is null when none is within 200 km.

Weather returns current conditions by default. Paid deep adds specialist current measurements, forecasts and related detail. A past `date` is a UTC day and requires deep: it adds `deep.history` alongside current conditions. Date alone does not request history.

```rust
parse.weather(40.7128, -74.006, WeatherOptions::default().deep(true).date("2026-08-15")).await?;
```

Tariff starts with the general schedule line. Paid deep adds units and the special and other schedule columns. An optional origin then resolves country-specific measures. The three calls below show those successive choices. Without origin, schedule detail is still returned and origin-dependent fields are null. A null effective rate is not a zero rate.

```rust
parse.tariff("8471.30.01.00", None).await?;
parse.tariff("8471.30.01.00", TariffOptions::default().deep(true)).await?;
parse.tariff("8471.30.01.00", TariffOptions::default().deep(true).origin("CN")).await?;
```

Address search uses context from the form: prefer postal, or city and state. An optional end-user `ip` is a locality hint for server-side calls. An empty result explains itself with `reason`: `more_input`, `missing_context` or `no_matches`. With suggestions, reason is null. Older responses may omit it, and future reasons remain strings. Catalog and lookup failures use the existing API errors.

HLR reports status at the last check. `live` means assigned and `connected` means reachable at that check. Cached results may be returned. Null means unconfirmed. Deep diagnostics stay within the same metered lookup.

## Deep

The default call returns the common answer. Request more detail with `parse.country_with_options("US", CountryOptions::default().deep(true)).await?`. Read those fields from the optional deep member; this does not change the core answer.

| Operation | What `deep` requests |
|---|---|
| IP | Richer IP fields included with a paid plan. No separate check meter. |
| Domain | Registration dates, registrar, status and DNSSEC, included with a paid plan. Use `dns` for DNS records and `mx` for mail routing. |
| Email | A metered deliverability check, using included email checks or enabled on-demand usage. |
| VAT | A metered registry check where supported, using included VAT checks or enabled on-demand usage. |
| Country, State, City, District, Postal | Reference profiles included with a paid plan; place identity and coordinates stay core. |
| VIN, NPI, NAICS, Company | Paid technical or registration profiles. NPI exclusion status and NAICS hierarchy stay core. |
| Tariff | Paid schedule columns and units; add origin for applicable measures. |
| Name, Weather | Paid name context or weather detail; parsing and current conditions stay core. |
| Phone, IBAN | Numbering-plan or bank structure detail in the same pooled request on every plan. |
| Time, Date, Currency, Language, Emoji, Point | Optional reference detail in the same pooled request on every plan. |
| Carrier, HLR | Available place or network detail from the same metered core unit, including Free included units. |

Carrier, caller, and HLR are separate metered operations. Choose them explicitly when you need their answers. Ordinary lookups retry twice by default. Metered checks use one attempt by default. Setting retries explicitly can repeat paid usage.

Without `deep`, the response omits that key. When requested, it is an empty object if access is locked or the operation has no deep fields. Otherwise it contains the available fields. A missing or null field means unknown.

## Errors

Every non-2xx response returns `Error::Api` with `status`, `code`, `message`, `docs`, and `request_id`. Branch on `code`.

```rust
match parse.city("atlantis", None).await {
    Err(err) if err.code() == Some("not_found") => { /* No matching city. */ }
    other => { /* Handle the result. */ }
}
```

Transport or response-decoding failures return `Error::Transport`. Construction failures return `Error::Config`.

## Requests and retries

Create one client and share it across tasks. Clones share the connection pool. Dropping a request future cancels its work, including any retry wait. The timeout defaults to 10 seconds per attempt. Use `tokio::time::timeout` to bound the whole call.

Ordinary lookups retry twice on network failures, 429, 500, 502, 503, and 504. Carrier, caller, and HLR calls use one attempt by default. Deep email, VAT, and address calls also use one attempt, reserving that behavior for address verification. Address deep currently returns an empty object. An explicit retry setting applies to every call, including metered operations. Additional attempts can be billed.

```rust
let parse = parseapi::Client::builder()
    .api_key("your-api-key")
    .timeout(std::time::Duration::from_secs(5))
    .retries(0)
    .build()?;
```

`.retries(0)` disables all automatic retries. Both numeric and HTTP-date `Retry-After` values are honored, capped at five seconds. Redirects return an API error and are never followed.

Requires Rust 1.88 or later and a Tokio runtime with time and I/O enabled. CI tests both the minimum and stable compiler, including a separate application's fresh dependency resolution.

[Full endpoint and field reference](https://parseapi.com/docs)

BIN lookup accepts 6-11 digits as a string, including leading zeros. Spaces and hyphens are accepted. `prefix` is the actual longest match and can be shorter than the input. Unknown reference fields are null. `deep` adds an empty object on every plan.
