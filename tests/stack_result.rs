use parseapi::Stack;

#[test]
fn current_stack_decodes_captured_wire_without_coverage_fields() {
 let body = include_str!("fixtures/current-stack.json");
 let result: Stack = serde_json::from_str(body).unwrap();
 assert_eq!(result.domain,"stripe.com");
 assert!(result.checked_at.is_some());
 assert!(result.cms.as_ref().unwrap().is_empty());
 assert_eq!(result.servers.as_ref().unwrap().len(),1);
 assert!(result.servers.unwrap()[0].version.is_none());
 let mut body:serde_json::Value=serde_json::from_str(body).unwrap();
 body["cms"]=serde_json::Value::Null;body["frameworks"]=serde_json::json!([]);
 let unknown:Stack=serde_json::from_value(body).unwrap();
 assert!(unknown.cms.is_none());assert!(unknown.frameworks.as_ref().unwrap().is_empty());
}

// Compile the short and options calls without changing the existing Stack calls.
#[allow(dead_code)]
async fn current_calls(client: &parseapi::Client) -> parseapi::Result<()> {
 let _:Stack=client.stack("stripe.com").await?;
 let _:Stack=client.stack_with_options("stripe.com",parseapi::StackOptions::default()).await?;
 Ok(())
}
