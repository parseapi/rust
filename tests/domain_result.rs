use parseapi::{Client, Domain, DomainOptions, DomainRegistration, DomainResult};
use serde_json::Value;
use std::io::{Read, Write};
use std::net::TcpListener;

#[tokio::test]
async fn domain_result_preserves_unknown_partial_and_one_request() {
	let fixtures: Value = serde_json::from_str(include_str!("fixtures/domain-result.json")).unwrap();
	for (name, body) in fixtures.as_object().unwrap() {
		let response = serde_json::to_vec(body).unwrap();
		let listener = TcpListener::bind("127.0.0.1:0").unwrap();
		let origin = format!("http://{}", listener.local_addr().unwrap());
		let server = std::thread::spawn(move || {
			let (mut stream, _) = listener.accept().unwrap();
			stream.set_read_timeout(Some(std::time::Duration::from_secs(3))).unwrap();
			let mut request = Vec::new(); let mut buffer = [0u8; 1024];
			while !request.windows(4).any(|x| x == b"\r\n\r\n") {
				let count = stream.read(&mut buffer).unwrap(); assert!(count > 0);
				request.extend_from_slice(&buffer[..count]);
			}
			write!(stream, "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", response.len()).unwrap();
			stream.write_all(&response).unwrap();
			String::from_utf8(request).unwrap()
		});
		let client = Client::builder().api_key("fixture").base_url(origin).retries(0).build().unwrap();
		let result: DomainResult = client.domain_result("bücher.example", DomainOptions::default().deep(true)).await.unwrap();
		let request = server.join().unwrap();
		assert!(request.starts_with("GET /domain/b%C3%BCcher.example?deep=true HTTP/1.1\r\n"));
		let headers = request.to_lowercase();
		assert!(headers.contains("parse-version: 2.0.0\r\n") && headers.contains("x-api-key: fixture\r\n"));
		if name == "unknown" || name == "unknown_deep" { assert!(result.available.is_none()); }
		else { assert_eq!(result.available, body["available"].as_bool()); }
		if name == "null_registration" { assert!(result.deep.as_ref().unwrap().registration.is_none()); }
		if name == "missing_registered" || name == "null_registered" {
			assert!(result.deep.as_ref().unwrap().registration.as_ref().unwrap().registered.is_none());
		}
		if name == "registered_false_empty" {
			let registration = result.deep.unwrap().registration.unwrap();
			assert_eq!(registration.dnssec, Some(false)); assert_eq!(registration.status, Some(vec![]));
		} else if name == "registered_unknown_detail" || name == "unregistered_partial" {
			let registration = result.deep.unwrap().registration.unwrap();
			assert!(registration.dnssec.is_none() && registration.status.is_none());
			if name == "unregistered_partial" { assert_eq!(registration.registered, Some(false)); }
		} else if name == "registered_true_status" {
			let registration = result.deep.unwrap().registration.unwrap();
			assert_eq!(registration.dnssec, Some(true));
			assert_eq!(registration.status, Some(vec!["client transfer prohibited".to_string()]));
		}
	}
}

#[test]
fn existing_domain_call_uses_nullable_boolean_fields() {
	let old: Domain = serde_json::from_str(r#"{"domain":"example.com","available":true}"#).unwrap();
	let available: Option<bool> = old.available;
	let registration: DomainRegistration = serde_json::from_str(r#"{"registered":true,"dnssec":false,"status":[]}"#).unwrap();
	let dnssec: Option<bool> = registration.dnssec;
	let status: Vec<String> = registration.status;
	assert_eq!(available, Some(true));
	assert_eq!(dnssec, Some(false));
	assert!(status.is_empty());
}
