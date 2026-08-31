# parseapi

Official parseAPI client for Rust.

```bash
cargo add parseapi tokio
```

```rust
let parse = parseapi::Client::new("your-api-key")?;
let country = parse.country("US").await?;
```

Get a key at [parseapi.com](https://parseapi.com). `Client::from_env()` reads `PARSEAPI_KEY`.

## Calls

One async method per endpoint, named after the route. Every response is a typed struct.

```rust
parse.ip("8.8.8.8", None).await?;
parse.ip_self(None).await?;
parse.email("hello@gmail.com", None).await?;
parse.phone("+14155552671", None).await?;
parse.carrier("+14155552671", None).await?;
parse.caller("+14155552671", None).await?;
parse.hlr("+14155552671", None).await?;
parse.postal("SW1A 1AA", "").await?;
parse.postal("28202", "US").await?;
parse.postal_nearby("28202", "US", None).await?;
parse.postal_distance("28202", "10001", "US").await?;
parse.city("charlotte", CityOptions { country: Some("US".into()), ..Default::default() }).await?;
parse.city_id("city_mb8mbqrkz8zb").await?;
parse.city_search("char", None).await?;
parse.city_nearest(35.2271, -80.8431).await?;
parse.city_nearby("denver", CityNearbyOptions { radius: Some(8.0), unit: Some("mi".into()), ..Default::default() }).await?;
parse.country("US").await?;
parse.country_states("US").await?;
parse.state("colorado", "").await?;
parse.state("NC", "US").await?;
parse.state_districts("NC", "US").await?;
parse.district("37081", None).await?;
parse.continent("NA").await?;
parse.continent_countries("NA").await?;
parse.currency("USD").await?;
parse.currency_rate("USD", "EUR").await?;
parse.language("en").await?;
parse.name("BILLY OSHALL").await?;
parse.timezone("America/New_York", None).await?;
parse.holiday("US", None).await?;
parse.holiday_date("US", "2026-12-25").await?;
parse.elevation(35.2271, -80.8431).await?;
parse.point(36.0726, -79.792, None).await?;
parse.weather(40.7128, -74.006, None).await?;
parse.domain("example.com", None).await?;
parse.mx("example.com").await?;
parse.useragent(ua_string, None).await?;
parse.emoji("rocket").await?;
parse.emoji_search("fire", None).await?;
```

## Deep

Pass `DeepOptions { deep: true }` to include the nested `deep` struct with richer fields.

```rust
let ip = parse.ip("52.94.76.10", DeepOptions { deep: true }).await?;
ip.deep.unwrap().datacenter; // Some(true)
```

## Errors

Every non-2xx response returns `Error::Api` with `status`, `code`, `message`, `docs`, and `request_id`. Branch on `code`.

```rust
match parse.city("atlantis", None).await {
    Err(err) if err.code() == Some("not_found") => { /* no such city */ }
    other => { /* ... */ }
}
```

Transport failures after retries surface as `Error::Transport`.

## Options

```rust
let parse = parseapi::Client::builder()
    .api_key("your-api-key")
    .timeout(Duration::from_secs(10)) // per-attempt timeout
    .retries(2) // automatic retries on network errors, 429, and 5xx
    .build()?;
```

Runs on any tokio runtime. TLS via rustls, no OpenSSL.

## Docs

Full field reference for every endpoint: [parseapi.com/docs](https://parseapi.com/docs)
