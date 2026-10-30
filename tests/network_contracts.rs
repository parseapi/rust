use parseapi::{District, Domain, Ip, Stack, State};
use serde_json::{from_value, Value};

#[test]
fn current_and_frozen_network_contracts_decode() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/network-contracts.json")).unwrap();
    for name in ["ip_current", "ip_frozen", "ip_locked"] {
        let result: Ip = from_value(fixtures[name].clone()).unwrap();
        let deep = result.deep.unwrap();
        if name == "ip_current" {
            assert_eq!(deep.district.as_deref(), Some("27053"));
            assert_eq!(deep.district_name.as_deref(), Some("Hennepin County"));
            assert_eq!(deep.census_division.as_deref(), Some("West North Central"));
            assert_eq!(deep.census_region.as_deref(), Some("Midwest"));
        } else { assert!(deep.district.is_none() && deep.census_region.is_none()); }
    }
    for name in ["stack_success", "stack_unavailable", "stack_empty"] {
        let result: Stack = from_value(fixtures[name].clone()).unwrap();
        if name == "stack_unavailable" { assert!(result.cms.is_none() && result.checked_at.is_none()); }
        else { assert!(result.cms.is_some() && result.frameworks.is_some()); }
    }
    let state: State = from_value(fixtures["state_current"].clone()).unwrap();
    assert_eq!(state.timezones, ["America/Los_Angeles"]);
    let district: District = from_value(fixtures["district_current"].clone()).unwrap();
    assert_eq!(district.timezones, ["America/New_York"]);
    for field in [Value::Null, serde_json::json!([])] {
        let mut body = fixtures["state_current"].clone();
        body["timezones"] = field;
        let result: State = from_value(body).unwrap();
        assert!(result.timezones.is_empty());
    }
    let domain: Domain = from_value(fixtures["domain_unregistered"].clone()).unwrap();
    assert!(domain.available && !domain.deep.unwrap().registration.unwrap().registered);
}
