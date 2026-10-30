use parseapi::{Client, Domain, DomainOptions};
use serde_json::{json, Value};
use std::io::{Read, Write};
use std::net::TcpListener;

#[tokio::test]
async fn ordinary_domain_preserves_unknown_and_known_states() {
    for available in [Value::Null, json!(false), json!(true)] {
        for dnssec in [
            None,
            Some(Value::Null),
            Some(json!(false)),
            Some(json!(true)),
        ] {
            let mut registration = json!({"registered": true});
            if let Some(value) = &dnssec {
                registration["dnssec"] = value.clone();
            }
            let deep = if available.is_null() {
                json!({})
            } else if available == true {
                json!({"registration":{"registered":false}})
            } else {
                json!({"registration":registration})
            };
            let body = json!({"domain":"example.com","available":available,"deep":deep});
            let response = serde_json::to_vec(&body).unwrap();
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let origin = format!("http://{}", listener.local_addr().unwrap());
            let server = std::thread::spawn(move || {
                let (mut stream, _) = listener.accept().unwrap();
                stream
                    .set_read_timeout(Some(std::time::Duration::from_secs(3)))
                    .unwrap();
                let mut request = Vec::new();
                let mut buffer = [0u8; 1024];
                while !request.windows(4).any(|x| x == b"\r\n\r\n") {
                    let count = stream.read(&mut buffer).unwrap();
                    assert!(count > 0);
                    request.extend_from_slice(&buffer[..count]);
                }
                write!(stream, "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", response.len()).unwrap();
                stream.write_all(&response).unwrap();
                String::from_utf8(request).unwrap()
            });
            let client = Client::builder()
                .api_key("fixture")
                .base_url(origin)
                .retries(0)
                .build()
                .unwrap();
            let result: Domain = client
                .domain("bücher.example", DomainOptions::default().deep(true))
                .await
                .unwrap();
            let request = server.join().unwrap();
            assert!(request.starts_with("GET /domain/b%C3%BCcher.example?deep=true HTTP/1.1\r\n"));
            let headers = request.to_lowercase();
            assert!(
                headers.contains("parse-version: 2.0.0\r\n")
                    && headers.contains("x-api-key: fixture\r\n")
            );
            assert_eq!(result.available, available.as_bool());
            if available == false {
                assert_eq!(
                    result.deep.unwrap().registration.unwrap().dnssec,
                    dnssec.and_then(|value| value.as_bool())
                );
            }
        }
    }
}

#[test]
fn malformed_boolean_types_remain_native_decode_errors() {
    assert!(
        serde_json::from_str::<Domain>(r#"{"domain":"example.com","available":"false"}"#).is_err()
    );
    assert!(serde_json::from_str::<Domain>(r#"{"domain":"example.com","available":false,"deep":{"registration":{"registered":true,"dnssec":"false"}}}"#).is_err());
}
