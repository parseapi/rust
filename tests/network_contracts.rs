use parseapi::{Domain, Ip, Stack};
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
        let scope: String = result.scope;
        let pages: i64 = result.pages;
        assert!(scope.is_empty() && pages == 0 && result.partial.is_none());
        if name == "stack_unavailable" { assert!(result.cms.is_none() && result.checked_at.is_none()); }
        else { assert!(result.cms.is_some() && result.frameworks.is_some()); }
    }
    let domain: Domain = from_value(fixtures["domain_unregistered"].clone()).unwrap();
    assert_eq!(domain.available, Some(true));
    assert!(!domain.deep.unwrap().registration.unwrap().registered);
}
