use parseapi::*;
use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
struct Request {
    target: String,
    headers: BTreeMap<String, String>,
}
struct Server {
    client: Client,
    calls: Arc<Mutex<Vec<Request>>>,
}
impl Server {
    fn start(responses: Vec<(u16, &str, String, &str)>, retries: u32) -> Self {
        let responses = responses
            .into_iter()
            .map(|(status, media, body, headers)| {
                (
                    status,
                    media.to_owned(),
                    body.into_bytes(),
                    headers.to_owned(),
                )
            })
            .collect::<Vec<_>>();
        Self::start_bytes(responses, retries)
    }
    fn start_bytes(responses: Vec<(u16, String, Vec<u8>, String)>, retries: u32) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let calls = Arc::new(Mutex::new(Vec::new()));
        let recorded = Arc::clone(&calls);
        let client = Client::builder()
            .api_key("test")
            .base_url(format!("http://{}", listener.local_addr().unwrap()))
            .retries(retries)
            .build()
            .unwrap();
        std::thread::spawn(move || {
            for (status, media, body, headers) in responses {
                let (mut stream, _) = listener.accept().unwrap();
                stream
                    .set_read_timeout(Some(std::time::Duration::from_secs(3)))
                    .unwrap();
                let mut raw = Vec::new();
                let mut buffer = [0; 1024];
                while !raw.windows(4).any(|part| part == b"\r\n\r\n") {
                    let count = stream.read(&mut buffer).unwrap();
                    if count == 0 {
                        break;
                    }
                    raw.extend_from_slice(&buffer[..count]);
                }
                let text = String::from_utf8(raw).unwrap();
                let mut lines = text.split("\r\n");
                let target = lines
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .nth(1)
                    .unwrap()
                    .to_owned();
                let request_headers = lines
                    .filter_map(|line| line.split_once(':'))
                    .map(|(name, value)| (name.to_ascii_lowercase(), value.trim().to_owned()))
                    .collect();
                recorded.lock().unwrap().push(Request {
                    target,
                    headers: request_headers,
                });
                write!(stream, "HTTP/1.1 {status} Fixture\r\nContent-Type: {media}\r\nContent-Length: {}\r\nConnection: close\r\n{headers}\r\n", body.len()).unwrap();
                stream.write_all(&body).unwrap();
            }
        });
        Self { client, calls }
    }
    fn requests(&self) -> Vec<Request> {
        self.calls.lock().unwrap().clone()
    }
}
fn query(target: &str) -> BTreeMap<String, String> {
    reqwest::Url::parse(&format!("http://fixture{target}"))
        .unwrap()
        .query_pairs()
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect()
}

#[tokio::test]
async fn original_rate_and_optional_evidence_keep_server_precision_and_nulls() {
    let body = r#"{"base":"USD","quote":"EUR","rate":1e-10,"date":"2024-01-05","amount":1000000000000000,"converted":1.01,"stale":false,"future":true}"#;
    let server = Server::start(
        vec![
            (200, "application/json", "{}".into(), ""),
            (200, "application/json", body.into(), ""),
        ],
        0,
    );
    server
        .client
        .currency_rate("USD", "EUR", None)
        .await
        .unwrap();
    let result = server
        .client
        .currency_rate(
            "USD",
            "EUR",
            CurrencyRateOptions::default()
                .date("2024-01-05")
                .amount(1e15),
        )
        .await
        .unwrap();
    assert_eq!(result.rate, 1e-10);
    assert_eq!(result.amount, Some(1e15));
    assert_eq!(result.converted, Some(1.01));
    assert_eq!(result.stale, Some(false));
    let calls = server.requests();
    assert_eq!(calls[0].target, "/currency/USD/EUR");
    let params = query(&calls[1].target);
    assert!(!params.contains_key("source"));
    assert!(!params.contains_key("deep"));
    assert_eq!(params["date"], "2024-01-05");
    assert_eq!(calls[1].headers["parse-version"], "2.0.0");
    assert_eq!(calls[1].headers["x-api-key"], "test");
}

#[tokio::test]
async fn typed_discovery_batch_and_both_history_contracts_preserve_missing_data() {
    let batch = r#"{"base":"EUR","amount":1.005,"rates":{"USD":{"rate":1.005,"date":"2024-01-05","converted":1.01}},"missing":[{"quote":"XCG","reason":"no_rate"}]}"#;
    let pair = r#"{"base":"EUR","quote":"USD","from":"2024-01-01","to":"2024-12-31","data":[],"missing":["2024-01-01"]}"#;
    let multi = r#"{"base":"EUR","quotes":["USD","TRL"],"data":[{"requested_date":"2004-12-31","quote":"TRL","converted":null}],"missing":[{"requested_date":"2005-02-01","quote":"TRL","reason":"no_rate"}]}"#;
    let server = Server::start(
        vec![batch, pair, multi]
            .into_iter()
            .map(|body| (200, "application/json", body.into(), ""))
            .collect(),
        0,
    );
    let rates = server
        .client
        .currency_rates(
            "EUR",
            ["USD", "XCG"],
            CurrencyRatesOptions::default()
                .amount(1.005),
        )
        .await
        .unwrap();
    assert_eq!(rates.rates["USD"].converted, Some(1.01));
    assert_eq!(rates.missing[0].quote, "XCG");
    assert_eq!(rates.requested_date, None);
    let history = server
        .client
        .currency_history("EUR", "USD", "2024-01-01", "2024-12-31", None)
        .await
        .unwrap();
    assert_eq!(history.missing, ["2024-01-01"]);
    let history = server
        .client
        .currency_rates_history(
            "EUR",
            ["USD", "TRL"],
            "2004-12-31",
            "2005-02-01",
            CurrencyRatesHistoryOptions::default(),
        )
        .await
        .unwrap();
    assert_eq!(history.quotes, ["USD", "TRL"]);
    assert_eq!(history.data[0].converted, None);
    assert_eq!(history.data[0].requested_date.as_str(), "2004-12-31");
    assert_eq!(
        history.missing[0].requested_date.as_deref(),
        Some("2005-02-01")
    );
    let calls = server.requests();
    assert_eq!(calls.len(), 3);
    assert_eq!(query(&calls[0].target)["quotes"], "USD,XCG");
    assert!(calls[2].target.starts_with("/currency/EUR/rates/history?"));
    let nullable: CurrencyRatesHistory =
        serde_json::from_str(r#"{"quotes":null,"data":null,"missing":null,"future":true}"#)
            .unwrap();
    assert!(nullable.quotes.is_empty() && nullable.data.is_empty() && nullable.missing.is_empty());
}

#[tokio::test]
async fn all_new_bounds_fail_before_transport_and_exact_5000_pair_days_work() {
    let server = Server::start(vec![(200, "application/json", "{}".into(), "")], 0);
    let client = &server.client;
    let quotes = (0..50)
        .map(|index| {
            format!(
                "Q{}{}",
                char::from(b'A' + index / 26),
                char::from(b'A' + index % 26)
            )
        })
        .collect::<Vec<_>>();
    for values in [Vec::<&str>::new(), vec!["USD", "usd"], vec!["USD", "US/"]] {
        assert!(matches!(
            client.currency_rates("EUR", values, None).await,
            Err(Error::Config(_))
        ));
    }
    let mut excess = quotes.clone();
    excess.push("ZZZ".into());
    assert!(matches!(
        client.currency_rates("EUR", excess, None).await,
        Err(Error::Config(_))
    ));
    for opts in [
        CurrencyRatesOptions::default().amount(f64::INFINITY),
        CurrencyRatesOptions::default().amount(f64::NAN),
        CurrencyRatesOptions::default().amount(1e16),
        CurrencyRatesOptions::default().date("2999-01-01"),
    ] {
        assert!(matches!(
            client.currency_rates("EUR", ["USD"], opts).await,
            Err(Error::Config(_))
        ));
    }
    for (from, to) in [
        ("1500-02-29", "1500-03-01"),
        ("0000-01-01", "0000-01-02"),
        ("2024-01-02", "2024-01-01"),
        ("2024-01-01", "2025-01-01"),
        ("2024-02-30", "2024-03-01"),
    ] {
        assert!(matches!(
            client.currency_history("EUR", "USD", from, to, None).await,
            Err(Error::Config(_))
        ));
    }
    assert!(matches!(
        client
            .currency_rates_history("EUR", &quotes, "2024-01-01", "2024-04-10", None)
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.requests().is_empty());
    client
        .currency_rates_history("EUR", quotes, "2024-01-01", "2024-04-09", None)
        .await
        .unwrap();
    assert_eq!(server.requests().len(), 1);
}




#[tokio::test]
async fn actual_candidate_all_quotes_catalog__decode() {
    let fixture: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/currency-reference.json")).unwrap();
    let server = Server::start(
        ["all_rates", "currencies"]
            .iter()
            .map(|key| (200, "application/json", fixture[key].to_string(), ""))
            .collect(),
        0,
    );
    let all = server
        .client
        .currency_rates_all(
            "USD",
            CurrencyRatesOptions::default(),
        )
        .await
        .unwrap();
    assert_eq!(all.rates.len(), 46);
    let gold = &all.rates["XAU"];
    assert_eq!(
        gold.units.as_ref().unwrap().quote.as_deref().unwrap(),
        "troy_ounce"
    );
    let catalog = server
        .client
        .currency_list(
            CurrencyListOptions::default()
                .q("gold")
                .deep(true)
                .lang("fr"),
        )
        .await
        .unwrap();
    assert_eq!(catalog.data[0].digits, None);
    assert_eq!(catalog.data[0].unit.as_deref(), Some("troy_ounce"));
    let calls = server.calls.lock().unwrap();
    assert!(!calls[0].target.contains("quotes="));
    assert!(!calls[0].target.contains("source="));
    assert!(calls[1].target.starts_with("/currency?"));
    assert!(calls[1].target.contains("lang=fr"));
}

#[tokio::test]
async fn indexed_unit_keep_raw_delivery() {
    let fixture: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/currency-reference.json")).unwrap();
    let server = Server::start(
        vec![(
            200,
            "application/json",
            fixture["indexed_pair"].to_string(),
            "",
        )],
        0,
    );
    let pair = server
        .client
        .currency_rate(
            "USD",
            "MXV",
            CurrencyRateOptions::default().amount(1.0),
        )
        .await
        .unwrap();
    assert_eq!(pair.units.unwrap().quote.unwrap().as_str(), "indexed_unit");
    assert_eq!(
        pair.converted,
        fixture["indexed_pair"]["converted"].as_f64()
    );
    assert_eq!(server.requests().len(), 1);
}

#[tokio::test]
async fn sampled_history_and_compact_reports_decode_complete_api_fixtures() {
    let sampled = include_str!("fixtures/currency/sampled-history.json");
    let summary = include_str!("fixtures/currency/period-summary.json");
    let server = Server::start(
        vec![
            (200, "application/json", sampled.into(), ""),
            (200, "application/json", summary.into(), ""),
        ],
        0,
    );
    let history = server
        .client
        .currency_history(
            "USD",
            "EUR",
            "1971-01-04",
            "2026-10-02",
            CurrencyHistoryOptions::default()
                .interval("month"),
        )
        .await
        .unwrap();
    assert_eq!(history.interval.as_deref(), Some("month"));
    assert_eq!(history.data[1].date, "2024-01-07");
    assert_eq!(history.data[1].stale, Some(true));
    assert_eq!(history.missing, vec!["2024-02-15"]);
    let report = server
        .client
        .currency_rates_summary(
            "U/S",
            ["EUR", "JPY", "XCG"],
            "2024-01-01",
            "2024-02-15",
            CurrencySummaryOptions::default()
                .interval("month"),
        )
        .await
        .unwrap();
    assert_eq!(report.interval, "month");
    assert_eq!(report.data[0].average, 0.825);
    assert_eq!(report.data[0].observation_count, 2);
    assert_eq!(report.data[0].first_date, "2024-01-02");
    assert_eq!(report.data[0].last_date, "2024-01-07");
    assert_eq!(report.data[0].missing_day_count, 1);
    assert_eq!(report.missing.len(), 4);
    assert_eq!(report.missing[1].reason, "no_observations");
    let calls = server.calls.lock().unwrap();
    assert!(calls[0].target.ends_with("&interval=month"));
    assert_eq!(calls[1].target,"/currency/U%2FS/rates/summary?from=2024-01-01&to=2024-02-15&quotes=EUR%2CJPY%2CXCG&interval=month");
    assert_eq!(
        calls[1].headers.get("parse-version").map(String::as_str),
        Some("2.0.0")
    );
}
