use parseapi::*;

#[test]
fn point_deep_exposes_metric_elevation_and_electoral_values() {
    for body in [r#"{}"#, r#"{"deep":{}}"#, r#"{"deep":{"elevation":null,"electoral":null}}"#] {
        let point: Point = serde_json::from_str(body).unwrap();
        assert!(point.deep.as_ref().and_then(|deep| deep.elevation).is_none());
        assert!(point.deep.as_ref().and_then(|deep| deep.electoral.as_ref()).is_none());
    }
    let zero: Point = serde_json::from_str(r#"{"deep":{"elevation":0,"elevation_ft":0,"electoral":{"code":"0801","name":null,"congress":119,"future":true}}}"#).unwrap();
    let deep = zero.deep.unwrap();
    assert_eq!(deep.elevation, Some(0.0));
    assert_eq!(deep.elevation_ft, Some(0.0));
    let electoral = deep.electoral.unwrap();
    assert_eq!(electoral.code, "0801");
    assert_eq!(electoral.name, None);
    assert_eq!(electoral.congress, Some(119));
    let old: Point = serde_json::from_str(r#"{"deep":{"elevation":-1.5,"resolution":30,"electoral":{"code":"0200","name":"At-large","congress":null}}}"#).unwrap();
    let deep = old.deep.unwrap();
    assert_eq!(deep.elevation, Some(-1.5));
    assert_eq!(deep.resolution, Some(30.0));
    assert_eq!(deep.electoral.unwrap().congress, None);
}
