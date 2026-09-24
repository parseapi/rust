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

Version 1.3.0 sends `Parse-Version: 2.0.0` on every request, including retries. Its response types match API `2.0.0`, and the client selects that contract automatically. No extra constructor setting or key change is needed. This behavior requires the matching API request-version release.

The team setting in [Dashboard API version](https://parseapi.com/dashboard/versions) is the default for requests without a version header. This SDK's header takes precedence without changing that saved default. Existing published packages keep their documented behavior.

Test the new SDK dependency in staging, then deploy the same locked dependency with your application code and existing production key. Future major SDK upgrades can deliberately select a newer API contract, so review their migration notes before upgrading. Rolling back the code and dependency restores the contract selected by that SDK release. If the older SDK does not send a version header, its requests use the team default, which must stay unchanged through that rollback window.

Keep the package version locked in your dependency configuration or lockfile. The selected API contract stays fixed across releases within this planned SDK major. See [API versions and migration](https://parseapi.com/docs/versioning).

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

Name paid deep includes flat `short`, `directory`, and `initials` fields beside `gender` and `salutation`. `name_locale` selects CLDR formatting rules and defaults to `en`. It changes formatting only. Country remains gender context, and unavailable formatting is null. Older responses may omit these fields.

## Display language

This source candidate accepts an optional language for supported display fields.
It requires the matching API localization release and data.

```rust
let country = parse
    .country("DE", Some(parseapi::CountryOptions::default().lang("fr")))
    .await?;
println!("{}", country.name); // Allemagne
```

`lang` applies to this request. The next call uses its usual default unless it
also supplies a language. Codes, native names, numeric facts and response
structure stay unchanged. Missing translations keep the API's documented
fallback. Existing `deep` rules still apply; Date `format` and Measure input
`locale` retain their parsing meanings.

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
parse.bank("DE89370400440532013000", None).await?;
parse.card("424242").await?;
parse.npi("1881018208", None).await?;
parse.asn("AS13335").await?;
parse.mac("00:1B:63:84:45:E6").await?;
parse.name("Andrea").await?;
parse.name_with_options("Andrea", NameOptions::default().country("IT")).await?;
parse.name_with_options("Robert James Smith", NameOptions::default().deep(true).name_locale("en")).await?;
parse.vin("1HGCM82633A004352", None).await?;
parse.carrier("+14155552671", None).await?;
parse.caller("+18004633339", None).await?;
parse.hlr("+447712345678", None).await?;
parse.dns("example.com", None).await?;
parse.dns("_dmarc.example.com", DnsOptions::default().r#type("TXT")).await?;
parse.stack("example.com").await?;
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

Australian postal lookups include core `localities` with suburb choices (`city`, `state`, `state_name`) on every plan. Null or an omitted field means unknown, while `[]` means the reviewed reference has no eligible choices. `city` stays null when the source is ambiguous, even if there is only one eligible choice. Let the user select their suburb and keep manual entry available. These are geographic choices, not mailing-address verification. [G-NAF source, adaptations and licence](https://parseapi.com/legal/attribution#postal-au).

Postal and District paid profiles include `deep.property_tax` where supported. It contains `annual_median`, `currency` and `period`: median annual property tax payable on owner-occupied homes in the statistical area. The amount is adjusted to the final year of the reporting period (`YYYY-YYYY`). This is an area statistic, not a rate or an individual property bill. Unsupported, missing and censored estimates are null.

```rust
let place = parse.postal("28202", PostalOptions::default().country("US").deep(true)).await?;
let property_tax = place.deep.as_ref().and_then(|deep| deep.property_tax.as_ref());
```

Read `population_period` alongside `population`: a reporting year (`YYYY`) or period (`YYYY-YYYY`), null when unknown or unverifiable. Keep missing or null values unknown and preserve a known zero. These fields belong to full place profiles. State district lists include each district's population and period. Postal nearby and distance detail remains metropolitan associations only. Continent population stays in core.

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

## NPI provider lookup

```rust
let provider = parse.npi("1881018208", None).await?;
let profile = parse.npi("1881018208", parseapi::NpiOptions::default().deep(true)).await?;
```

Pass the original NPI as a string. `valid` checks its format and checksum; `registered` means a match in the stored NPPES snapshot. `active` reflects recorded NPI deactivation, not licensure. `excluded` is an NPI-only OIG LEIE match; `false` is not a complete exclusion clearance. These directory facts do not verify credentials, current practice contact or payment eligibility.

Invalid input returns `valid: false` with unknown provider fields. A checksum-valid number missing from the snapshot returns `registered: false`; unavailable storage remains an API error. Preserve `null` as unknown.

The default pooled lookup includes provider identity, specialty and practice contact where held. Paid `deep` adds `deactivated_at`, `medicare`, `opt_out` and `enrollments` from stored source files, with no separate check meter or live verification. `enrollments: null` means unavailable; `[]` means no enrollment rows are returned. The API omits unrequested `deep` and returns `{}` when requested on Free.

## Deep

The default call returns the common answer. Request more detail with `parse.country_with_options("US", CountryOptions::default().deep(true)).await?`. Read those fields from the optional deep member; this does not change the core answer.

| Operation | What `deep` requests |
|---|---|
| IP | Richer IP fields included with a paid plan. No separate check meter. |
| Domain | Registration dates, registrar, status and DNSSEC, included with a paid plan. Use `dns` for DNS records and `mx` for mail routing. |
| Email | A metered mailbox check with deliverability, catch-all, status, reason and address hints, using included email checks or enabled on-demand usage. |
| VAT | A metered registry check where supported, using included VAT checks or enabled on-demand usage. |
| Country, State, City, District, Postal | Reference profiles included with a paid plan; place identity and coordinates stay core. |
| NPI | Deactivation date, Medicare enrollment, opt-out and enrollment rows from stored sources on paid plans. Exclusion evidence stays core. |
| VIN, NAICS, Company | Paid technical or registration profiles. NPI exclusion status and NAICS hierarchy stay core. |
| Tariff | Paid schedule columns and units; add origin for applicable measures. |
| Name, Weather | Paid name context or weather detail; parsing and current conditions stay core. |
| Phone, Bank | Numbering-plan or bank structure detail in the same pooled request on every plan. |
| Time, Date, Currency, Language, Emoji, Point | Optional reference detail in the same pooled request on every plan. |
| Carrier, HLR | Available place or network detail from the same metered core unit, including Free included units. |

Email deep includes mailbox status and the reason for the result, plus a suggested first name, no-reply flag, plus-address tag and mail service. The suggested name is not a verified identity. Unavailable details are null.

Reasons include `accepted`, `invalid_format`, `invalid_domain`, `no_mail_server`, `mailbox_not_found`, `mailbox_disabled`, `mailbox_full`, `catchall`, `disposable`, `temporary_failure`, `rejected` and `unconfirmed`.

Carrier, caller, and HLR are separate metered operations. Choose them explicitly when you need their answers. Ordinary lookups retry twice by default. Metered checks use one attempt by default. Setting retries explicitly can repeat paid usage.

Without `deep`, the response omits that key. When requested, it is an empty object if access is locked or the operation has no deep fields. Otherwise it contains the available fields. A missing or null field means unknown.

## Bank validation

Bank results include optional `checks` and `issues` (`BankChecks` and `BankIssue`). Check statuses and issue codes are open strings; handle unknown future values. `not_supported` means the national check did not run, not that it passed. `issues: []` means no applicable check failed; a missing/null value supports older responses. These findings do not establish account existence or ownership. `deep.account` remains a string so leading zeros are preserved.


Bank lookups send raw input in a JSON body (`POST /bank`), preserving leading zeros, separators and forbidden characters for server validation. `bank` keeps its existing call signature and IBAN result. Optional `deep.directory` identifies the directory edition, country and open-string match grain; absent data remains unknown.

```rust
let requirements = parse.bank_requirements("US", Some("us_ach")).await?;
let result = parse.bank_us_ach(parseapi::BankUsAchInput::new("021000021", "000123456789")).await?;
```

US ACH checks the routing checksum and supported account format, not account existence, ownership or ACH eligibility. Account checksum status stays `not_supported`; bank names are nullable partial-directory references. Account text is preserved, including letter case, spaces and hyphens. Requirements describe this validation workflow; they are not every field needed to initiate a payment. Unsupported country/format combinations return `supported: false`. Pass `None` as the format for IBAN requirements. The sample is synthetic, not an account to pay.

## Errors

Every non-2xx response returns `Error::Api` with `status`, `code`, `message`, `docs`, and `request_id`, plus nullable `retry_after` header metadata. Branch on `code`.

```rust
match parse.city("atlantis", None).await {
    Err(err) if err.code() == Some("not_found") => { /* No matching city. */ }
    other => { /* Handle the result. */ }
}
```

Transport or response-decoding failures return `Error::Transport`. Construction failures return `Error::Config`.

## Requests and retries

Create one client and share it across tasks. Clones share the connection pool. Dropping a request future cancels its work, including any retry wait. The timeout defaults to 35 seconds for Stack and 10 seconds for other operations. Use `tokio::time::timeout` to bound the whole call.

Ordinary lookups retry twice on network failures, 429, 500, 502, 503, and 504. Carrier, caller, and HLR calls use one attempt by default. Deep email, VAT, and address calls also use one attempt, reserving that behavior for address verification. Address deep currently returns an empty object. An explicit retry setting applies to every call, including metered operations. Additional attempts can be billed.

```rust
let parse = parseapi::Client::builder()
    .api_key("your-api-key")
    .timeout(std::time::Duration::from_secs(5))
    .retries(0)
    .build()?;
```

`.retries(0)` disables all automatic retries. Numeric and HTTP-date `Retry-After` values up to five seconds are honored. Longer server waits return the API error immediately, with the raw header in `retry_after`, so the application can schedule a later attempt. Missing or invalid headers use ordinary backoff. Redirects return an API error and are never followed.

Requires Rust 1.88 or later and a Tokio runtime with time and I/O enabled. CI tests both the minimum and stable compiler, including a separate application's fresh dependency resolution.

[Full endpoint and field reference](https://parseapi.com/docs)

## Card

Card looks up issuer, network and type from a BIN/IIN. It accepts 6-11 digits as a string, including leading zeros. Spaces and hyphens are accepted. `prefix` is the actual longest match and can be shorter than the input. Unknown reference fields are null. Invalid prefixes and full card numbers are rejected locally before a request is sent. Accepted input is sent unchanged.

Compare `prefix` with the normalized response `bin`. A matched row can still have all metadata unknown. Keep unknown prepaid status separate from true and false. Place it inside the async `main` above, before `Ok(())`.

```rust
let card = parse.card("4242 42-99").await?;
let match_kind = match card.prefix.as_deref() {
    None => "No reference match",
    Some(prefix) if prefix == card.bin => "Exact prefix match",
    Some(_) => "Broader prefix match",
};
let prepaid = match card.prepaid {
    None => "Unknown prepaid status",
    Some(true) => "Prepaid",
    Some(false) => "Not prepaid",
};
println!("{match_kind}, {prepaid}");
```

## Stack

```rust
let result = parse.stack("example.com").await?;
```

Pass a public hostname without a scheme, path, port or IP address. Stack returns the homepage URL and `checked_at` time, then eight technology arrays: `cms`, `servers`, `frameworks`, `ecommerce`, `analytics`, `chat`, `payments` and `hosting`. Each entry contains a `technology` code, name and nullable version. Multiple CMSs or servers remain separate entries. Empty arrays mean no matches in the checked pages. An unsuccessful check returns null arrays and a null `checked_at`.

`scope` identifies `homepage` or `site` coverage. `pages` counts successfully checked HTML pages. `partial` is true for a homepage-only or incomplete bounded site check, false when the known in-scope candidates finished, and null when no check succeeded. False does not guarantee that every page on the website was discovered.

The complete technology result is included in the core response. The generic `deep=true` option adds only an empty object and is unnecessary for Stack. Successful checks may be reused for up to 24 hours. `pretty` optionally formats the wire JSON. Each lookup uses one request and API version 2.0.0 selected by this client.

Stack defaults to 35 seconds per attempt so a first scan has time to finish. Other lookups retain their 10-second default. An explicit client timeout takes precedence.
