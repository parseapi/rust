use parseapi::{County, District, StateCounties, Postal, Ip, City};
#[test]
fn county_models_read_current_and_historical_wire_keys() {
 let county: County = serde_json::from_str(r#"{"county":"08031","name":"Denver County","country":"US","type":"county","population":0}"#).unwrap();
 assert_eq!(county.county,"08031"); assert_eq!(county.population,Some(0));
 let old: District=serde_json::from_str(r#"{"district":"75","name":"Paris","country":"FR","type":"department"}"#).unwrap();
 assert_eq!(old.county,"75");assert_eq!(old.kind.as_deref(),Some("department"));
 let list: StateCounties=serde_json::from_str(r#"{"state":"CO","country":"US","districts":[{"district":"08031","name":"Denver County","population":0}]}"#).unwrap();
 assert_eq!(list.counties[0].county,"08031");assert_eq!(list.counties[0].population,Some(0));
 let postal:Postal=serde_json::from_str(r#"{"postal":"08001","country":"US","district":"08031","district_name":"Denver","district_name_local":null}"#).unwrap();
 assert_eq!(postal.county.as_deref(),Some("08031"));assert!(postal.county_name_local.is_none());
 let ip:Ip=serde_json::from_str(r#"{"ip":"8.8.8.8","county":null,"county_name":null}"#).unwrap();assert!(ip.county.is_none());
 let city:City=serde_json::from_str(r#"{"name":"Paris","country":"FR","county":"75","type":"district","capital_of":"county"}"#).unwrap();
 assert_eq!(city.kind.as_deref(),Some("district"));assert_eq!(city.capital_of.as_deref(),Some("county"));
}
