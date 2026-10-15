use parseapi::*;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
#[derive(Debug, Clone)]
struct Recorded {
	method: String,
	body: String,
	target: String,
	headers: HashMap<String, String>,
}

struct TestServer {
	base_url: String,
	requests: Arc<Mutex<Vec<Recorded>>>,
}

impl TestServer {
	/// Serves the canned (status, body) responses in order, one connection each.
	fn start(responses: Vec<(u16, &'static str)>) -> TestServer {
		Self::start_with_headers(
			responses
				.into_iter()
				.map(|(status, body)| (status, body, String::new()))
				.collect(),
		)
	}

	fn start_with_headers(responses: Vec<(u16, &'static str, String)>) -> TestServer {
		let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
		let base_url = format!("http://{}", listener.local_addr().expect("addr"));
		let requests: Arc<Mutex<Vec<Recorded>>> = Arc::new(Mutex::new(Vec::new()));
		let recorded = Arc::clone(&requests);

		std::thread::spawn(move || {
			for (status, body, headers) in responses {
				let (mut stream, _) = match listener.accept() {
					Ok(conn) => conn,
					Err(_) => return,
				};
				let mut raw = Vec::new();
				let mut buf = [0_u8; 1024];
				while !raw.windows(4).any(|w| w == b"\r\n\r\n") {
					match stream.read(&mut buf) {
						Ok(0) => break,
						Ok(n) => raw.extend_from_slice(&buf[..n]),
						Err(_) => break,
					}
				}
				let text = String::from_utf8_lossy(&raw);
				let head_end = raw.windows(4).position(|w| w == b"\r\n\r\n").unwrap() + 4;
				let mut lines = text[..head_end].split("\r\n");
				let request_line = lines.next().unwrap_or_default();
				let method = request_line.split(' ').next().unwrap_or_default().to_owned();
				let target = request_line
					.split(' ')
					.nth(1)
					.unwrap_or_default()
					.to_string();
				let mut request_headers = HashMap::new();
				for line in lines {
					if let Some((name, value)) = line.split_once(':') {
						request_headers
							.insert(name.trim().to_lowercase(), value.trim().to_string());
					}
				}
				let length = request_headers.get("content-length").and_then(|v| v.parse::<usize>().ok()).unwrap_or(0);
				while raw.len() < head_end + length {
					let n = stream.read(&mut buf).unwrap();
					if n == 0 { break; }
					raw.extend_from_slice(&buf[..n]);
				}
				let request_body = String::from_utf8(raw[head_end..].to_vec()).unwrap();
				recorded.lock().unwrap().push(Recorded {
					method,
					body: request_body,
					target,
					headers: request_headers,
				});

				let response = format!(
					"HTTP/1.1 {status} X\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n{headers}\r\n{body}",
					body.len(),
				);
				let _ = stream.write_all(response.as_bytes());
			}
		});

		TestServer { base_url, requests }
	}

	fn client(&self) -> Client {
		Client::builder()
			.api_key("test_key_123")
			.base_url(&self.base_url)
			.retries(0)
			.build()
			.expect("client")
	}

	fn requests(&self) -> Vec<Recorded> {
		self.requests.lock().unwrap().clone()
	}
}

#[tokio::test]
async fn city_quality_options_preserve_default_methods() {
    let server = TestServer::start(vec![(200, "{}"),(200, "{}"),(200, "{}"),(200, "{}"),(200, "{}"),(200, "{}"),(200, "{}"),(200, "{}"),(200, "{}"),(200, "{}")]);
    let client = server.client();
    client.city("Wien", None).await.unwrap();
    client.city_id("city_pin").await.unwrap();
    client.city_search("Wie", None).await.unwrap();
    client.city_nearest(48.0, 16.0).await.unwrap();
    client.city_nearby("Wien", None).await.unwrap();
    client.city("Wien", CityOptions::default().country("AT").state("9").r#type("city").grain("municipality").hierarchy(true).aliases(true).ambiguity("error")).await.unwrap();
    client.city_id_with_options("city_pin", CityIdOptions::default().hierarchy(true)).await.unwrap();
    client.city_search("Wie", CitySearchOptions::default().r#type("city").grain("municipality").hierarchy(true).aliases(true)).await.unwrap();
    client.city_nearest_with_options(48.0, 16.0, CityNearestOptions::default().country("AT").state("9").r#type("city").grain("municipality").hierarchy(true)).await.unwrap();
    client.city_nearby("Wien", CityNearbyOptions::default().r#type("city").grain("municipality").hierarchy(true).aliases(true).ambiguity("error")).await.unwrap();
    let calls = server.requests();
    assert_eq!(calls[0].target, "/city/Wien");
    assert_eq!(calls[1].target, "/city/id/city_pin");
    assert_eq!(calls[2].target, "/city?q=Wie");
    assert_eq!(calls[3].target, "/city?lat=48&lon=16");
    assert_eq!(calls[4].target, "/city/Wien/nearby");
    for (index, call) in calls.iter().skip(5).enumerate() {
        let query: std::collections::BTreeMap<_,_> = reqwest::Url::parse(&format!("http://test{}",call.target)).unwrap().query_pairs().map(|(k,v)|(k.into_owned(),v.into_owned())).collect();
        assert_eq!(query.get("hierarchy").map(String::as_str),Some("true"));
        if index != 1 {assert_eq!(query.get("type").map(String::as_str),Some("city"));assert_eq!(query.get("grain").map(String::as_str),Some("municipality"));}
        if [0,2,4].contains(&index) {assert_eq!(query.get("aliases").map(String::as_str),Some("true"));}
        if [0,4].contains(&index) {assert_eq!(query.get("ambiguity").map(String::as_str),Some("error"));}
        if index == 3 {assert_eq!(query.get("country").map(String::as_str),Some("AT"));assert_eq!(query.get("state").map(String::as_str),Some("9"));}
    }
}

#[test]
fn city_quality_hierarchy_decodes_old_and_nullable_responses() {
    let child = serde_json::json!({"id":"city_child","name":"Ward","country":"JP","grain":"subdivision","parent":{"id":"city_parent","name":null,"name_local":"堺市","country":"JP","state":"JP-27","type":"city","grain":"municipality","future":true},"future":true});
    let city: City = serde_json::from_value(child.clone()).unwrap();
    assert_eq!(city.grain.as_deref(),Some("subdivision"));
    let parent = city.parent.unwrap();
    assert_eq!(parent.id,"city_parent");assert!(parent.name.is_none());assert_eq!(parent.name_local.as_deref(),Some("堺市"));assert_eq!(parent.kind.as_deref(),Some("city"));
    let search: CitySearch = serde_json::from_value(serde_json::json!({"cities":[child.clone()]})).unwrap();
    assert_eq!(search.cities[0].parent.as_ref().unwrap().id,"city_parent");
    let nearby: CityNearby = serde_json::from_value(serde_json::json!({"nearby":[child]})).unwrap();
    assert_eq!(nearby.nearby[0].city.parent.as_ref().unwrap().id,"city_parent");
    for body in [serde_json::json!({"id":"city_old","name":"Old","country":"AT"}),serde_json::json!({"id":"city_old","name":"Old","country":"AT","grain":null,"parent":null})] { let old: City=serde_json::from_value(body).unwrap();assert!(old.grain.is_none()&&old.parent.is_none()); }
}

#[test]
fn city_quality_population_observation_preserves_zero_and_unknown() {
    let city: City = serde_json::from_value(serde_json::json!({"id":"city_pin","deep":{"population":0,"population_observed_on":"2025-12-31","population_basis":"registered_population"}})).unwrap();
    let deep=city.deep.unwrap();assert_eq!(deep.population,Some(0));assert_eq!(deep.population_observed_on.as_deref(),Some("2025-12-31"));assert_eq!(deep.population_basis.as_deref(),Some("registered_population"));
    for body in [serde_json::json!({"deep":{}}),serde_json::json!({"deep":{"population_observed_on":null,"population_basis":null}})] {let old:City=serde_json::from_value(body).unwrap();let deep=old.deep.unwrap();assert!(deep.population_observed_on.is_none()&&deep.population_basis.is_none());}
}

#[tokio::test]
async fn city_queries_encode_zero_bounds_dateline_and_opaque_cursor() {
    let server = TestServer::start(vec![(200,"{}"),(200,"{}"),(200,"{}"),(200,"{}"),(200,"{}"),(200,"{}"),(200,"{}")]);
    let client = server.client();
    let cursor = "opaque+/=cursor?&";
    client.city_list(CityListOptions::default().country("JP").limit(5).hierarchy(true).min_population(0).max_population(5000000).timezone("Asia/Tokyo").bbox("170,-20,-170,20").sort("population").order("desc").cursor(cursor)).await.unwrap();
    client.city_nearby_coords(0.0,0.0,CityNearbyCoordsOptions::default().radius(20.0).unit("mi").country("JP").state("27").min_population(0).max_population(5000000).timezone("Asia/Tokyo").bbox("170,-20,-170,20").sort("population").order("desc").cursor(cursor)).await.unwrap();
    client.city_search("Sak",CitySearchOptions::default().page(true).min_population(0).max_population(5000000).timezone("Asia/Tokyo").bbox("170,-20,-170,20").sort("population").order("desc").cursor(cursor)).await.unwrap();
    client.city_nearby("Sakai",CityNearbyOptions::default().page(true).min_population(0).max_population(5000000).timezone("Asia/Tokyo").bbox("170,-20,-170,20").sort("population").order("desc").cursor(cursor)).await.unwrap();
    client.city_list(None).await.unwrap();
    client.city_nearby_coords(0.0,0.0,None).await.unwrap();
    client.city_search("Sak",CitySearchOptions::default().cursor(cursor)).await.unwrap();
    let calls = server.requests();
    for (index, call) in calls.iter().take(4).enumerate() {
        let url=reqwest::Url::parse(&format!("http://test{}",call.target)).unwrap();
        assert_eq!(url.path(),if index==3 {"/city/Sakai/nearby"} else {"/city"});
        let query:std::collections::BTreeMap<_,_>=url.query_pairs().map(|(k,v)|(k.into_owned(),v.into_owned())).collect();
        for (key,value) in [("min_population","0"),("max_population","5000000"),("timezone","Asia/Tokyo"),("bbox","170,-20,-170,20"),("sort","population"),("order","desc"),("cursor",cursor)] { assert_eq!(query.get(key).map(String::as_str),Some(value)); }
        if index<2 {assert!(!query.contains_key("page"));} else {assert_eq!(query.get("page").map(String::as_str),Some("true"));}
        if index==0 {assert_eq!(query.get("browse").map(String::as_str),Some("true"));}
        if index==1 {for key in ["lat","lon"] {assert_eq!(query.get(key).map(String::as_str),Some("0"));}assert_eq!(query.get("nearby").map(String::as_str),Some("true"));}
    }
    assert_eq!(calls[4].target,"/city?browse=true&format=list");
    assert_eq!(calls[5].target,"/city?nearby=true&lat=0&lon=0&format=list");
    let query:std::collections::BTreeMap<_,_>=reqwest::Url::parse(&format!("http://test{}",calls[6].target)).unwrap().query_pairs().map(|(k,v)|(k.into_owned(),v.into_owned())).collect();
    assert_eq!(query.len(),2);assert_eq!(query.get("q").map(String::as_str),Some("Sak"));assert_eq!(query.get("cursor").map(String::as_str),Some(cursor));
}

#[test]
fn city_queries_decode_pagination_without_inventing_cursor() {
    let list:CityList=serde_json::from_value(serde_json::json!({"object":"list","has_more":false,"url":"/city","data":[{"object":"city","id":"city_one","name":"One","country":"JP"}],"country":"JP","next_cursor":"opaque","future":true})).unwrap();
    assert_eq!(list.data.len(),1);assert_eq!(list.next_cursor.as_deref(),Some("opaque"));assert_eq!(list.country.as_deref(),Some("JP"));
    let coords:CityNearbyCoords=serde_json::from_value(serde_json::json!({"object":"list","has_more":false,"url":"/city","latitude":0,"longitude":0,"radius":20,"unit":"mi","data":[{"object":"city","id":"city_one","name":"One","country":"JP","distance":4,"distance_mi":2.485}],"next_cursor":null})).unwrap();
    assert_eq!((coords.latitude,coords.longitude,coords.radius),(0.0,0.0,20.0));assert_eq!(coords.data.len(),1);assert_eq!(coords.data[0].distance,Some(4.0));assert!(coords.next_cursor.is_none());
    for body in [serde_json::json!({"cities":[]}),serde_json::json!({"cities":[],"next_cursor":null})] {let old:CitySearch=serde_json::from_value(body).unwrap();assert!(old.next_cursor.is_none());}
    for body in [serde_json::json!({"nearby":[]}),serde_json::json!({"nearby":[],"next_cursor":null})] {let old:CityNearby=serde_json::from_value(body).unwrap();assert!(old.next_cursor.is_none());}
}

#[tokio::test]
async fn city_canonical_filters_and_classic_defaults() {
 let server=TestServer::start(vec![(200,r#"{"object":"list","data":[{"id":"city_123456789abc","object":"city","name_local":"北九州市","latitude":0,"longitude":null}],"has_more":true,"url":"/city","next_cursor":"opaque+/=","country":"JP"}"#),(200,r#"{"id":"city_123456789abc","object":"city","name_local":"北九州市"}"#),(200,r#"{"object":"list","data":[],"has_more":false,"url":"/city","next_cursor":null}"#),(200,"{}"),(200,"{}"),(200,"{}"),(200,"{}"),(200,"{}")]);
 let c=server.client();let page=c.city_list(CityListOptions::default().countries(["JP","AT"]).exclude_countries(["US"]).timezones(["Asia/Tokyo"]).min_elevation(0.0).max_elevation(100.0).min_population(0).sort("-population,name").cursor("old+/=")).await.unwrap();assert!(page.next_cursor.is_some());assert_eq!(page.data[0].latitude,Some(0.0));assert!(page.data[0].longitude.is_none() && page.data[0].name.is_none());assert_eq!(page.data[0].name_local.as_deref(),Some("北九州市"));
 let row=c.city_resource("北九州", CityOptions::default().country("JP")).await.unwrap();assert!(row.name.is_none());assert_eq!(row.object,"city");c.city_search_list("Kit", CitySearchOptions::default().countries(["JP"])).await.unwrap();c.city("Wien",None).await.unwrap();c.city_id("city_123456789abc").await.unwrap();c.city_search("Wie",None).await.unwrap();c.city_nearest(0.0,0.0).await.unwrap();c.city_nearby("Wien",None).await.unwrap();
 let calls=server.requests();let query:std::collections::BTreeMap<_,_>=reqwest::Url::parse(&format!("http://test{}",calls[0].target)).unwrap().query_pairs().map(|(k,v)|(k.into_owned(),v.into_owned())).collect();for(key,value)in[("format","list"),("countries","JP,AT"),("exclude_countries","US"),("timezones","Asia/Tokyo"),("min_elevation","0"),("min_population","0"),("max_elevation","100"),("sort","-population,name"),("cursor","old+/=")]{assert_eq!(query[key],value);}
 assert!(calls[1].target.contains("format=resource"));assert!(calls[2].target.contains("format=list")&&calls[2].target.contains("countries=JP"));for (index,path) in [(3,"/city/Wien"),(4,"/city/id/city_123456789abc"),(5,"/city?q=Wie"),(6,"/city?lat=0&lon=0"),(7,"/city/Wien/nearby")]{assert_eq!(calls[index].target,path);}for call in calls {assert_eq!(call.headers["parse-version"],"2.0.0");assert_eq!(call.headers["x-api-key"],"test_key_123");}
}

#[tokio::test]
async fn city_canonical_graph_and_distance_zero_unknown() {
 let server=TestServer::start(vec![(200,r#"{"object":"list","data":[{"id":"city_123456789abc","object":"city","name":null}],"has_more":false,"url":"/city/id/city_123456789abc/children","next_cursor":null,"city_id":"city_123456789abc","relationship":"children","coverage":"partial"}"#),(200,r#"{"object":"list","data":[],"has_more":false,"url":"/city/id/city_123456789abc/ancestors","next_cursor":null,"city_id":"city_123456789abc","relationship":"ancestors","coverage":"partial"}"#),(200,r#"{"object":"list","data":[],"has_more":false,"url":"/city/id/city_123456789abc/siblings","next_cursor":null,"city_id":"city_123456789abc","relationship":"siblings","coverage":"partial","parent_id":null}"#),(200,r#"{"object":"city_distance","from":"city_123456789abc","to":"city_123456789abc","distance":0,"distance_mi":0,"method":"great_circle"}"#),(200,r#"{"object":"city_distance","from":"city_123456789abc","to":"city_123456789abd","distance":null,"distance_mi":null,"method":"great_circle"}"#)]);let c=server.client();assert_eq!(c.city_children("city_123456789abc",CityRelationsOptions::default().limit(1)).await.unwrap().coverage,"partial");c.city_ancestors("city_123456789abc",None).await.unwrap();assert!(c.city_siblings("city_123456789abc",None).await.unwrap().parent_id.is_none());assert_eq!(c.city_distance("city_123456789abc","city_123456789abc").await.unwrap().distance,Some(0.0));assert!(c.city_distance("city_123456789abc","city_123456789abd").await.unwrap().distance.is_none());let calls=server.requests();assert_eq!(calls[0].target,"/city/id/city_123456789abc/children?limit=1");assert_eq!(calls[1].target,"/city/id/city_123456789abc/ancestors");assert_eq!(calls[2].target,"/city/id/city_123456789abc/siblings");assert_eq!(calls[3].target,"/city/id/city_123456789abc/distance/city_123456789abc");assert_eq!(calls[4].target,"/city/id/city_123456789abc/distance/city_123456789abd");
}

#[tokio::test]
async fn city_canonical_409_candidates_are_bounded_and_other_errors_unchanged() {
 let body=r#"{"code":"ambiguous_city","message":"Choose a city","request_id":"req_city","docs":"https://parseapi.com/docs/city#choose-a-city","candidates":[{"id":"city_123456789abc","object":"city","name":null,"name_local":"北九州市","deep":{"private":"no"},"future":"ignored"},{"id":"city_123456789abd","object":"city","country":"JP"},{"id":"city_123456789abe","object":"city","name":"excluded"}]}"#;let server=TestServer::start_with_headers(vec![(409,body,"Retry-After: 2\r\n".into()),(400,body,String::new()),(429,r#"{"code":"rate_limited","message":"Wait","candidates":[{"id":"city_123456789abc","object":"city"}]}"#,"Retry-After: 3\r\n".into())]);let c=server.client();match c.city("Same",CityOptions::default().ambiguity("error")).await.unwrap_err(){Error::Api{status,code,candidates,request_id,retry_after,..}=>{assert_eq!(status,409);assert_eq!(code,"ambiguous_city");assert_eq!(request_id.as_deref(),Some("req_city"));assert_eq!(retry_after.as_deref(),Some("2"));let rows=candidates.unwrap();assert_eq!(rows.len(),2);assert_eq!(rows[0].name_local.as_deref(),Some("北九州市"));assert!(rows[0].name.is_none());},other=>panic!("wrong error {other:?}")};for expected in [400,429]{match c.city("Same",None).await.unwrap_err(){Error::Api{status,candidates,retry_after,..}=>{assert_eq!(status,expected);assert!(candidates.is_none());if status==429{assert_eq!(retry_after.as_deref(),Some("3"));}},other=>panic!("wrong error {other:?}")}}assert_eq!(server.requests().len(),3);
}

#[tokio::test]
async fn city_explicit_matching_retains_suggestion_evidence() {
 let server=TestServer::start(vec![(200,r#"{"object":"list","data":[{"id":"city_7aj899nvwyku","object":"city","match":{"type":"fuzzy","source":"name","value":"Vienna","language":null,"edit_distance":1}}],"has_more":false,"url":"/city","next_cursor":null}"#)]);let result=server.client().city_search_list("Viena", CitySearchOptions::default().match_mode("fuzzy").max_distance(1)).await.unwrap();assert_eq!(result.data[0].r#match.as_ref().unwrap().edit_distance,Some(1));let q:std::collections::BTreeMap<_,_>=reqwest::Url::parse(&format!("http://test{}",server.requests()[0].target)).unwrap().query_pairs().map(|(k,v)|(k.into_owned(),v.into_owned())).collect();assert!(server.requests()[0].target.starts_with("/city/search?"));assert_eq!(q["match"],"fuzzy");assert_eq!(q["max_distance"],"1");assert_eq!(q["format"],"list");
}


#[tokio::test]
async fn city_reviewed_romanization_optin_and_null(){
 let server=TestServer::start(vec![(200,r#"{"id":"city_7aj899nvwyku","object":"city","name_romanized":"Kitakyūshū","romanization_language":"ja-Latn"}"#),(200,r#"{"id":"city_7aj899nvwyku","object":"city","name_romanized":null,"romanization_language":null}"#)]);let c=server.client();let row=c.city_id_resource("city_7aj899nvwyku", CityIdOptions::default().romanized(true)).await.unwrap();assert_eq!(row.name_romanized.as_deref(),Some("Kitakyūshū"));assert_eq!(row.romanization_language.as_deref(),Some("ja-Latn"));assert!(c.city_id_resource("city_7aj899nvwyku", CityIdOptions::default().romanized(true)).await.unwrap().name_romanized.is_none());assert!(server.requests()[0].target.contains("romanized=true"));
}

#[tokio::test]
async fn city_classic_explicit_match_adapts_canonical_envelope(){
 let server=TestServer::start(vec![(200,r#"{"object":"list","q":"Viena","data":[{"id":"city_7aj899nvwyku","name":"Vienna","country":"AT"}],"has_more":false,"next_cursor":null}"#)]);
 let page=server.client().city_search("Viena",CitySearchOptions::default().country("AT").match_mode("fuzzy").max_distance(1)).await.unwrap();assert_eq!(page.cities.len(),1);assert_eq!(page.cities[0].id,"city_7aj899nvwyku");assert_eq!(page.q,"Viena");assert!(server.requests()[0].target.starts_with("/city/search?"));
}
