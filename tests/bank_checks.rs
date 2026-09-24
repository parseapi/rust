use parseapi::Bank;

#[test]
fn old_and_null_bank_results_keep_decoding() {
    for body in [
        r#"{"iban":"DE89370400440532013000","valid":true,"deep":{"account":"0532013000"}}"#,
        r#"{"iban":"DE89370400440532013000","valid":true,"checks":null,"issues":null}"#,
    ] {
        let value: Bank = serde_json::from_str(body).unwrap();
        assert!(value.checks.is_none());
        assert!(value.issues.is_none());
        assert_eq!(value.iban.as_deref(), Some("DE89370400440532013000"));
    }
}

#[test]
fn passed_iso_checks_keep_unsupported_national_distinct_and_empty_issues() {
    let value: Bank = serde_json::from_str(r#"{"iban":"DE89370400440532013000","valid":true,"checks":{"input":"passed","country":"passed","length":"passed","structure":"passed","checksum":"passed","national":"not_supported"},"issues":[],"deep":{"account":"0532013000"}}"#).unwrap();
    let checks = value.checks.as_ref().unwrap();
    for status in [&checks.input, &checks.country, &checks.length, &checks.structure, &checks.checksum] {
        assert_eq!(status.as_deref(), Some("passed"));
    }
    assert_eq!(checks.national.as_deref(), Some("not_supported"));
    assert!(value.issues.as_ref().unwrap().is_empty());
    assert!(value.valid);
    assert_eq!(value.deep.unwrap().account.as_deref(), Some("0532013000"));
}

#[test]
fn national_failure_and_future_codes_are_findings_not_decode_errors() {
    for (status, code) in [("failed", "invalid_national_checksum"), ("future_status", "future_issue")] {
        let body = serde_json::json!({"valid":false,"checks":{"checksum":"passed","national":status,"future":true},
            "issues":[{"field":"iban","code":code,"message":"Review these bank details.","future":true}],"future":true});
        let value: Bank = serde_json::from_value(body).unwrap();
        assert!(!value.valid);
        let checks = value.checks.unwrap();
        assert_eq!(checks.national.as_deref(), Some(status));
        assert!(checks.input.is_none());
        let issue = &value.issues.unwrap()[0];
        assert_eq!(issue.field.as_deref(), Some("iban"));
        assert_eq!(issue.code.as_deref(), Some(code));
        assert_eq!(issue.message.as_deref(), Some("Review these bank details."));
    }
    let empty: Bank = serde_json::from_str(r#"{"valid":false,"checks":{},"issues":[{}]}"#).unwrap();
    assert!(empty.checks.unwrap().national.is_none());
    assert!(empty.issues.unwrap()[0].code.is_none());
}

#[test]
fn requirements_and_domestic_shapes_preserve_unknown_statuses_and_optional_fields() {
    let requirements: parseapi::BankRequirements = serde_json::from_str(r#"{"country":"US","format":"us_ach","supported":true,"fields":[{"key":"account","label":"Account number","required":true,"type":"string","max_length":17,"max_input_length":128,"length_unit":"non_space_characters"}],"checks":{"account_checksum":"not_supported","future":"future_scope"},"limitations":[]}"#).unwrap();
    assert!(requirements.supported);
    assert_eq!(requirements.fields[0].max_length, Some(17));
    assert_eq!(requirements.fields[0].max_input_length, Some(128));
    assert!(requirements.fields[0].min_length.is_none());
    assert_eq!(requirements.checks.get("future").map(String::as_str), Some("future_scope"));
    let result: parseapi::BankUsAch = serde_json::from_str(r#"{"format":"us_ach","country":"US","routing":"021000021","account":"000a-12 3","valid":true,"checks":{"routing_format":"passed","routing_checksum":"passed","account_format":"passed","account_checksum":"not_supported"},"issues":[]}"#).unwrap();
    assert_eq!(result.account.as_deref(), Some("000a-12 3"));
    assert_eq!(result.checks.unwrap().account_checksum.as_deref(), Some("not_supported"));
    assert!(result.valid && result.issues.unwrap().is_empty());
}
