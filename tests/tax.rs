use parseapi::{Country, Postal};

#[test]
fn tax_reference_fields_preserve_decimal_and_zero() {
 let country: Country = serde_json::from_str(r#"{"country":"DE","deep":{"tax":"VAT","tax_rate":19,"tax_id_format":"DE999999999","tax_id_regex":"^DE[0-9]{9}$"}}"#).unwrap();
 let c = country.deep.unwrap();
 assert_eq!(c.tax.as_deref(), Some("VAT"));
 assert_eq!(c.tax_rate, Some(19.0));
 assert_eq!(c.tax_id_format.as_deref(), Some("DE999999999"));
 assert_eq!(c.tax_id_regex.as_deref(), Some("^DE[0-9]{9}$"));
 let postal: Postal = serde_json::from_str(r#"{"postal":"12345","country":"US","deep":{"tax":"Sales tax","tax_rate":7.9,"tax_rate_state":5,"tax_rate_county":0,"tax_rate_city":null,"tax_rate_special":2.9}}"#).unwrap();
 let p = postal.deep.unwrap();
 assert_eq!(p.tax_rate, Some(7.9));
 assert_eq!(p.tax_rate_state, Some(5.0));
 assert_eq!(p.tax_rate_county, Some(0.0));
 assert_eq!(p.tax_rate_city, None);
 assert_eq!(p.tax_rate_special, Some(2.9));
}

#[test]
fn absent_locked_and_unknown_tax_remain_unknown() {
 for body in ["{}", r#"{"deep":{}}"#, r#"{"deep":{"tax":null,"tax_rate":null}}"#] {
  let country: Country = serde_json::from_str(body).unwrap();
  let postal: Postal = serde_json::from_str(body).unwrap();
  if let Some(c) = country.deep { assert!(c.tax.is_none() && c.tax_rate.is_none() && c.tax_id_format.is_none() && c.tax_id_regex.is_none()); }
  if let Some(p) = postal.deep { assert!(p.tax.is_none() && p.tax_rate.is_none() && p.tax_rate_special.is_none()); }
 }
}
