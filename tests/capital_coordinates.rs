use parseapi::{Country, State};
#[test]
fn capital_coordinates_accept_old_and_current_payloads() {
 for capital in [r#"{"name":"Example","lat":0,"lon":null}"#,r#"{"name":"Example","latitude":0,"longitude":null}"#,r#"{"name":"Example","lat":99,"lon":99,"latitude":0,"longitude":null}"#,r#"{"name":"Example","latitude":0,"longitude":null,"lat":99,"lon":99}"#] {
  let body=format!(r#"{{"capital":{capital}}}"#);
  let country:Country=serde_json::from_str(&body).unwrap();let state:State=serde_json::from_str(&body).unwrap();
  let c=country.capital.unwrap();let s=state.capital.unwrap();
  assert_eq!((c.latitude,c.longitude),(Some(0.0),None));assert_eq!((s.latitude,s.longitude),(Some(0.0),None));
 }
}
