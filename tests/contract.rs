// Frozen consumer examples after the final pre-launch cleanup. These compile
// without making requests. Keep them working when adding future features.
use parseapi::*;

#[allow(dead_code)]
async fn frozen_consumer_calls(client: &Client) -> Result<()> {
	let _: Ip = client
		.ip("example", IpOptions::default().deep(true))
		.await?;
	let _: Ip = client.ip_self(IpSelfOptions::default().deep(true)).await?;
	let _: Continent = client.continent("example").await?;
	let _: ContinentCountries = client.continent_countries("example").await?;
	let _: Bloc = client.bloc("example").await?;
	let _: BlocCountries = client.bloc_countries("example").await?;
	let _: Asn = client.asn("AS13335").await?;
	let _: Mac = client.mac("00:1B:63:84:45:E6").await?;
	let _: Country = client.country("example").await?;
	let _: CountryStates = client.country_states("example").await?;
	let _: State = client
		.state("example", StateOptions::default().country("example"))
		.await?;
	let _: StateDistricts = client
		.state_districts(
			"example",
			StateDistrictsOptions::default().country("example"),
		)
		.await?;
	let _: District = client
		.district(
			"example",
			DistrictOptions::default()
				.country("example")
				.state("example"),
		)
		.await?;
	let _: City = client
		.city(
			"example",
			CityOptions::default().country("example").state("example"),
		)
		.await?;
	let _: City = client.city_id("example").await?;
	let _: CitySearch = client
		.city_search(
			"example",
			CitySearchOptions::default()
				.country("example")
				.state("example")
				.limit(1),
		)
		.await?;
	let _: CityNearest = client.city_nearest(1.0, 1.0).await?;
	let _: CityNearby = client
		.city_nearby(
			"example",
			CityNearbyOptions::default()
				.country("example")
				.state("example")
				.radius(1.0)
				.unit("example")
				.limit(1),
		)
		.await?;
	let _: Postal = client
		.postal("example", PostalOptions::default().country("example"))
		.await?;
	let _: PostalNearby = client
		.postal_nearby(
			"example",
			PostalNearbyOptions::default()
				.country("example")
				.radius(1.0)
				.unit("example"),
		)
		.await?;
	let _: PostalDistance = client
		.postal_distance(
			"example",
			"example",
			PostalDistanceOptions::default().country("example"),
		)
		.await?;
	let _: Email = client
		.email("example", EmailOptions::default().deep(true))
		.await?;
	let _: Vat = client
		.vat(
			"example",
			VatOptions::default()
				.country("example")
				.from("example")
				.deep(true),
		)
		.await?;
	let _: Iban = client
		.iban("example", IbanOptions::default().country("example"))
		.await?;
	let _: Npi = client
		.npi("example", NpiOptions::default().deep(true))
		.await?;
	let _: Phone = client
		.phone(
			"example",
			PhoneOptions::default().country("example").deep(true),
		)
		.await?;
	let _: Carrier = client
		.carrier("example", CarrierOptions::default().country("example"))
		.await?;
	let _: Caller = client
		.caller("example", CallerOptions::default().country("example"))
		.await?;
	let _: Hlr = client
		.hlr("example", HlrOptions::default().country("example"))
		.await?;
	let _: Domain = client
		.domain("example", DomainOptions::default().deep(true))
		.await?;
	let _: Mx = client.mx("example").await?;
	let _: Useragent = client
		.useragent("example", UseragentOptions::default().deep(true))
		.await?;
	let _: Vin = client
		.vin("example", VinOptions::default().deep(true))
		.await?;
	let _: Tariff = client
		.tariff(
			"example",
			TariffOptions::default().deep(true).origin("example"),
		)
		.await?;
	let _: TariffSearch = client.tariff_search("example").await?;
	let _: Currency = client.currency("example").await?;
	let _: Language = client.language("example").await?;
	let _: Name = client.name("example").await?;
	let _: CurrencyRate = client
		.currency_rate(
			"example",
			"example",
			CurrencyRateOptions::default().date("example").amount(0.0),
		)
		.await?;
	let _: Timezone = client
		.timezone(
			"example",
			TimezoneOptions::default().at("example").to("example"),
		)
		.await?;
	let _: Timezone = client
		.timezone_at(1.0, 1.0, TimezoneAtOptions::default().at("example"))
		.await?;
	let _: DateInfo = client
		.date(
			"example",
			DateOptions::default().format("example").to("example"),
		)
		.await?;
	let _: DateInfo = client
		.date_today(DateTodayOptions::default().to("example"))
		.await?;
	let _: HolidayYear = client
		.holiday("example", HolidayOptions::default().year(1))
		.await?;
	let _: HolidayDate = client.holiday_date("example", "example").await?;
	let _: Elevation = client.elevation(1.0, 1.0).await?;
	let _: Point = client
		.point(1.0, 1.0, PointOptions::default().deep(true))
		.await?;
	let _: Weather = client
		.weather(
			1.0,
			1.0,
			WeatherOptions::default().deep(true).date("example"),
		)
		.await?;
	let _: Emoji = client.emoji("example").await?;
	let _: EmojiSearch = client
		.emoji_search("example", EmojiSearchOptions::default().limit(1))
		.await?;
	let _: Address = client
		.address(
			"example",
			AddressOptions::default().country("example").deep(true),
		)
		.await?;
	let _: AddressSearch = client
		.address_search(
			"example",
			AddressSearchOptions::default()
				.country("example")
				.postal("example")
				.city("example")
				.state("example")
				.ip("example"),
		)
		.await?;
	let _: Company = client
		.company(
			"example",
			CompanyOptions::default().country("example").deep(true),
		)
		.await?;
	Ok(())
}

#[test]
fn client_and_errors_remain_send_sync() {
	fn send_sync<T: Send + Sync>() {}
	send_sync::<Client>();
	send_sync::<Error>();
	fn send<T: Send>(_: T) {}
	let client = Client::new("contract_test_key").unwrap();
	send(frozen_consumer_calls(&client));
}
