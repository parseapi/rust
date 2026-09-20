use parseapi::Email;

#[test]
fn email_enrichment_preserves_false_null_and_open_codes() {
    let value: Email = serde_json::from_str(r#"{"email":"jane.doe+news@example.com","valid":true,"free":true,"domain_type":"academic","deep": {"first_name":"Jane","no_reply":false,"tag":"news","mail_provider":"future-provider","deliverable":true,"catchall":false,"status":"future-status","reason":"future_reason"},"future":true}"#).unwrap();
    let deep = value.deep.as_ref().unwrap();
    assert_eq!(deep.first_name.as_deref(), Some("Jane"));
    assert_eq!(deep.no_reply, Some(false));
    assert_eq!(deep.tag.as_deref(), Some("news"));
    assert_eq!(deep.mail_provider.as_deref(), Some("future-provider"));
    assert!(value.free);
    assert_eq!(value.domain_type.as_deref(), Some("academic"));
    assert_eq!(deep.status.as_deref(), Some("future-status"));
    assert_eq!(deep.reason.as_deref(), Some("future_reason"));
}

#[test]
fn older_and_locked_email_results_stay_decodable() {
    for body in [r#"{"email":"a@example.com"}"#, r#"{"email":"a@example.com","deep":{}}"#, r#"{"email":"a@example.com","deep": {"first_name":null,"no_reply":null,"tag":null,"mail_provider":null,"status":null,"reason":null}}"#, r#"{"email":"a@example.com","deep":{"deliverable":false,"catchall":true}}"#] {
        let value: Email = serde_json::from_str(body).unwrap();
        if let Some(deep) = value.deep {
            assert!(deep.first_name.is_none() && deep.no_reply.is_none() && deep.tag.is_none() && deep.mail_provider.is_none());
            assert!(deep.status.is_none() && deep.reason.is_none());
        }
    }
}
