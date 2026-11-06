use parseapi::{Country,Language,Currency,Postal,PostalDeep,Point,Industry,Emoji,CompanyProfile,NaicsSearch};
use serde_json::{Value,from_value,json};
#[test]
fn core_references_decode_without_changing_deep_records(){
 let f:Value=serde_json::from_str(include_str!("fixtures/core-connectors.json")).unwrap();
 let c:Country=from_value(f["country"].clone()).unwrap();assert_eq!(c.iso3.as_deref(),Some("USA"));assert_eq!(c.capital.as_ref().unwrap().latitude,Some(0.0));assert_eq!(c.borders.unwrap(),vec!["CA","MX"]);
 let l:Language=from_value(f["language"].clone()).unwrap();assert_eq!(l.iso3.as_deref(),Some("eng"));assert_eq!(l.countries.as_ref().unwrap().len(),2);
 let u:Currency=from_value(f["currency"].clone()).unwrap();assert_eq!(u.countries.unwrap(),vec!["US"]);
 let p:Postal=from_value(f["postal"].clone()).unwrap();assert_eq!(p.clone().currency.as_deref(),Some("USD"));assert_eq!(p.postal,"00501");
 let point:Point=from_value(f["point"].clone()).unwrap();assert_eq!(point.clone().city.unwrap().id.as_deref(),Some("city_7aj899nvwyku"));
 assert_eq!(p.country,"US");assert_eq!(p.clone().neighbors.as_ref().unwrap(),&vec!["00544"]);assert!(p.currency.is_some());
 let n:Industry=from_value(f["industry"].clone()).unwrap();assert_eq!(n.clone().children.unwrap()[0].naics,"111");
 let search:NaicsSearch=from_value(f["industry_search"].clone()).unwrap();assert_eq!(search.results[0].naics,"722515");assert_eq!(search.results[0].r#match.as_ref().unwrap().text,"Coffee shops, on premise brewing");
 let e:Emoji=from_value(f["emoji"].clone()).unwrap();assert_eq!(e.clone().base.as_deref(),Some("👍"));assert_eq!(e.clone().variants.unwrap()[0].emoji,"👍🏽");
 let co:CompanyProfile=from_value(f["company"].clone()).unwrap();assert_eq!(co.clone().identifiers.as_ref().unwrap()[0].value,"0000320193");assert_eq!(co.clone().industries.as_ref().unwrap()[0].code,"3571");let phone=&co.phone_numbers.as_ref().unwrap()[0];assert_eq!(phone.country.as_deref(),Some("US"));assert_eq!(phone.e164.as_deref(),Some("+14089961010"));
}
#[test]
fn older_missing_unknown_and_empty_references_stay_decodable(){
 let c:Country=from_value(json!({})).unwrap();assert!(c.iso3.is_none()&&c.borders.is_none());
 let l:Language=from_value(json!({"countries":null})).unwrap();assert!(l.countries.is_none());
 let l:Language=from_value(json!({"countries":[]})).unwrap();assert_eq!(l.countries,Some(vec![]));
 let e:Emoji=from_value(json!({})).unwrap();assert!(e.base.is_none());
 let p:PostalDeep=from_value(json!({})).unwrap();assert!(p.neighbors.is_none());let p:PostalDeep=from_value(json!({"neighbors":null})).unwrap();assert!(p.neighbors.is_none());let p:PostalDeep=from_value(json!({"neighbors":[]})).unwrap();assert_eq!(p.neighbors,Some(vec![]));
 let point:Point=from_value(json!({"city_id":"unpublished-alias"})).unwrap();assert!(point.city.is_none());
 let co:CompanyProfile=from_value(json!({})).unwrap();assert!(co.legal_name.is_none());
}
