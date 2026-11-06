use parseapi::{Country, Postal};

#[test]
fn tax_reference_fields_preserve_decimal_and_zero() {
 let country: Country = serde_json::from_str(r#"{"country":"DE","tax":{"name":"VAT","rate":19}}"#).unwrap();
 let c = country.clone();
 assert_eq!(c.tax.as_ref().and_then(|tax|tax.name.as_deref()), Some("VAT"));
 assert_eq!(c.tax.as_ref().and_then(|tax|tax.rate), Some(19.0));
 let postal: Postal = serde_json::from_str(r#"{"postal":"12345","country":"US","tax":{"name":"Sales tax","rate":7.9,"rate_state":5,"rate_county":0,"rate_city":null,"rate_special":2.9}}"#).unwrap();
 let p = postal.clone();
 assert_eq!(p.tax.as_ref().and_then(|tax|tax.rate), Some(7.9));
 assert_eq!(p.tax.as_ref().and_then(|tax|tax.rate_state), Some(5.0));
 assert_eq!(p.tax.as_ref().and_then(|tax|tax.rate_county), Some(0.0));
 assert_eq!(p.tax.as_ref().and_then(|tax|tax.rate_city), None);
 assert_eq!(p.tax.as_ref().and_then(|tax|tax.rate_special), Some(2.9));
}

#[test]
fn absent_locked_and_unknown_tax_remain_unknown() {
 for body in ["{}", r#"{}"#, r#"{"tax":null,"tax_rate":null}"#] {
  let country: Country = serde_json::from_str(body).unwrap();
  let postal: Postal = serde_json::from_str(body).unwrap();
  { let c = country; assert!(c.tax.as_ref().and_then(|tax|tax.name.as_ref()).is_none() && c.tax.as_ref().and_then(|tax|tax.rate).is_none() && c.tax_id_format.is_none() && c.tax_id_regex.is_none()); }
  { let p=postal; assert!(p.tax.is_none() && p.tax.as_ref().and_then(|tax|tax.rate).is_none() && p.tax.as_ref().and_then(|tax|tax.rate_special).is_none()); }
 }
}
