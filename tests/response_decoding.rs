use parseapi::*;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::time::Duration;

#[tokio::test]
async fn ordinary_request_and_retry() {
 let listener=TcpListener::bind("127.0.0.1:0").unwrap();let address=listener.local_addr().unwrap();
 let body=include_str!("fixtures/response-fixtures/clean-timezones.json");
 let server=std::thread::spawn(move || {
  let mut requests=Vec::new();
  for attempt in 0..2 {
   let (mut connection,_)=listener.accept().unwrap(); connection.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
   let mut bytes=Vec::new();let mut buffer=[0;1024];
   while !bytes.windows(4).any(|p|p==b"\r\n\r\n") {let read=connection.read(&mut buffer).unwrap();if read==0 {break;}bytes.extend_from_slice(&buffer[..read]);}
   requests.push(String::from_utf8(bytes).unwrap().to_lowercase());
   if attempt==0 {write!(connection,"HTTP/1.1 503 Service Unavailable\r\nRetry-After: 0\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{{}}").unwrap();}
   else {write!(connection,"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",body.len(),body).unwrap();}
  } requests
 });
 let result=Client::builder().api_key("fixture").base_url(format!("http://{address}")).retries(1).build().unwrap().time_zones("").await.unwrap();
 assert!(!result.timezones.is_empty());
 for request in server.join().unwrap() {assert!(request.contains("parse-version: 2.0.0"));assert!(!request.contains("parse-response:"));}
}

#[test]
fn actual_handler_representations_keep_published_getters() {
 for profile in ["standard","clean"] {
  let read=|name:&str| std::fs::read_to_string(format!("{}/tests/fixtures/response-fixtures/{profile}-{name}.json",env!("CARGO_MANIFEST_DIR"))).unwrap();
  let zones:TimeZones=serde_json::from_str(&read("timezones")).unwrap();
  let time:Time=serde_json::from_str(&read("time")).unwrap();
  let metadata:EmojiMetadata=serde_json::from_str(&read("emoji-metadata")).unwrap();
  let _:EmojiCatalog=serde_json::from_str(&read("emoji-catalog")).unwrap();
  let coverage:CompanyCoverage=serde_json::from_str(&read("company-coverage")).unwrap();
  let company:CompanyProfile=serde_json::from_str(&read("company-profile")).unwrap();
  let employees=company.deep.unwrap().employees.unwrap();
  let getters:Vec<String>=vec![zones.timezone_database_version,time.location.unwrap().source,metadata.source_version,coverage.snapshot_at,employees.method.clone()];
  assert_eq!(getters.len(),5);assert_eq!(employees.method,if profile=="standard" {"reported"} else {""});assert_eq!(employees.count,12);assert!(!employees.approximate);
  assert!(!metadata.artwork_sources.is_empty());
 }
}
