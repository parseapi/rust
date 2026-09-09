use parseapi::*;

#[test]
fn name_local_decodes_for_every_named_response() {
 for (body, expected) in [(r#"{"name_local":"München"}"#, Some("München")), (r#"{"name_local":null}"#, None), ("{}", None)] {
  macro_rules! check {
   ($response:ty) => {
    assert_eq!(serde_json::from_str::<$response>(body).unwrap().name_local.as_deref(), expected);
   };
  }
  check!(Country);
  check!(State);
  check!(City);
  check!(Language);
  check!(Holiday);
  check!(PointCity);
  assert_eq!(serde_json::from_str::<CityNearest>(body).unwrap().city.name_local.as_deref(), expected);
 }
}
