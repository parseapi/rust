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
parse.npi("1881018208", None).await?;
parse.asn("AS13335").await?;
parse.mac("00:1B:63:84:45:E6").await?;
parse.vin("1HGCM82633A004352", None).await?;
parse.carrier("+14155552671", None).await?;
parse.caller("+18004633339", None).await?;
parse.hlr("+447712345678", None).await?;
parse.tariff("8471.30.01.00", TariffOptions::default().origin("DE").deep(true)).await?;
parse.address("123 Main St", AddressOptions::default().country("US")).await?;
parse.address_search("123 Main", AddressSearchOptions::default().country("US").state("NC")).await?;
parse.company("123456789", CompanyOptions::default().country("FR")).await?;
parse.date("03/04/2026", DateOptions::default().format("mdy")).await?;
parse.date_today(DateTodayOptions::default().to("2026-12-25")).await?;
parse.timezone("America/New_York", TimezoneOptions::default().at("2026-09-05T15:00:00").to("Asia/Tokyo")).await?;
parse.timezone_at(40.7128, -74.006, None).await?;
parse.weather(40.7128, -74.006, WeatherOptions::default().deep(true).date("2026-09-01")).await?;
```

Nullable values use `Option`. Unknown JSON fields are accepted. An omitted `deep` is `None`, and a requested empty `deep` is `Some` with empty fields. Nullable arrays are normalized to empty vectors. API fields named `type` use `r#type` where a separate `kind` field also exists.

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
