use parseapi::{Currency,CurrencyRate};
use std::collections::BTreeMap;
#[test]
fn restoration_preserves_unknown_zero_and_server_precision(){
 let rows:BTreeMap<String,serde_json::Value>=serde_json::from_str(include_str!("fixtures/currency-compatibility.json")).unwrap();
 let p:Currency=serde_json::from_value(rows["profile"].clone()).unwrap();assert_eq!(p.digits,Some(0));assert!(p.symbol.is_none());assert_eq!(p.countries,Some(vec!["JP".to_string()]));
 for n in ["locked","nulls"]{let p:Currency=serde_json::from_value(rows[n].clone()).unwrap();assert!(p.deep.is_some());assert!(p.iso_numeric.is_none()&&p.name_plural.is_none()&&p.countries.is_none());}
 let r:CurrencyRate=serde_json::from_value(rows["rate"].clone()).unwrap();assert_eq!(r.rate,158.33550065);assert_eq!(r.amount,Some(1.005));assert_eq!(r.converted,Some(1.0));
 let z:CurrencyRate=serde_json::from_value(rows["zero"].clone()).unwrap();assert_eq!(z.rate,0.0);assert_eq!(z.amount,Some(0.0));assert_eq!(z.converted,Some(0.0));
}
