use parseapi::*;
use serde_json::Value;
#[test] fn approved_root_payloads_decode_without_projection() {
 let rows:Value=serde_json::from_str(include_str!("fixtures/approved-2.1.json")).unwrap();
 for row in rows.as_array().unwrap(){let body=&row["body"];let product=row["product"].as_str().unwrap();
 let actual:Vec<(&str,Value)>=match product {
"ip"=>{let v:Ip=serde_json::from_value(body.clone()).unwrap();vec![("country",serde_json::to_value(v.country).unwrap()),("asn",serde_json::to_value(v.asn).unwrap()),("datacenter",serde_json::to_value(v.datacenter).unwrap())]},
"country"=>{let v:Country=serde_json::from_value(body.clone()).unwrap();vec![("name_full",serde_json::to_value(v.name_full).unwrap()),("iso_numeric",serde_json::to_value(v.numeric).unwrap()),("population",serde_json::to_value(v.population).unwrap()),("tax.rate",serde_json::to_value(v.tax.as_ref().and_then(|tax|tax.rate)).unwrap())]},
"state"=>{let v:State=serde_json::from_value(body.clone()).unwrap();vec![("iso_subdivision",serde_json::to_value(v.iso).unwrap()),("population",serde_json::to_value(v.population).unwrap()),("capital.latitude",serde_json::to_value(v.capital.as_ref().and_then(|capital|capital.latitude)).unwrap())]},
"postal"=>{let v:Postal=serde_json::from_value(body.clone()).unwrap();vec![("postal",serde_json::to_value(v.postal).unwrap()),("population",serde_json::to_value(v.population).unwrap()),("tax.rate",serde_json::to_value(v.tax.as_ref().and_then(|tax|tax.rate)).unwrap())]},
"email"=>{let v:Email=serde_json::from_value(body.clone()).unwrap();vec![("valid",serde_json::to_value(v.valid).unwrap()),("deliverable",serde_json::to_value(v.deliverable).unwrap()),("catchall",serde_json::to_value(v.catchall).unwrap()),("status",serde_json::to_value(v.status).unwrap())]},
"carrier"=>{let v:Carrier=serde_json::from_value(body.clone()).unwrap();vec![("valid",serde_json::to_value(v.valid).unwrap()),("city",serde_json::to_value(v.city).unwrap()),("burner",serde_json::to_value(v.burner).unwrap())]},
"hlr"=>{let v:Hlr=serde_json::from_value(body.clone()).unwrap();vec![("live",serde_json::to_value(v.live).unwrap()),("connected",serde_json::to_value(v.connected).unwrap()),("roaming",serde_json::to_value(v.roaming).unwrap())]},
"useragent"=>{let v:Useragent=serde_json::from_value(body.clone()).unwrap();vec![("bot",serde_json::to_value(v.bot).unwrap()),("device.type",serde_json::to_value(v.device.as_ref().and_then(|device|device.kind.clone())).unwrap()),("browser.name",serde_json::to_value(v.browser.as_ref().and_then(|browser|browser.name.clone())).unwrap())]},
"weather"=>{let v:Weather=serde_json::from_value(body.clone()).unwrap();vec![("current.temperature",serde_json::to_value(v.current.temperature).unwrap()),("current.pressure",serde_json::to_value(v.current.pressure).unwrap())]},
_=>panic!("Unknown fixture")};
 for (path,got) in actual {let mut wanted=body;for key in path.split('.') {wanted=&wanted[key]};if got.is_number() && wanted.is_number() { assert_eq!(got.as_f64(),wanted.as_f64(),"{}:{}",row["id"],path); } else { assert_eq!(&got,wanted,"{}:{}",row["id"],path); }}
 }
}
