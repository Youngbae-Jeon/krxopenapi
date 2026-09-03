# KRX Open API fetch client

```rs
	let client = KrxOpenApiClient::builder()
		// give your auth key for KRX Open API (you can get it from https://openapi.krx.co.kr)
		.auth_key(auth_key)
		// set rate limiting to 1 request per 500ms (this is the default)
		.rate(1)
		.rate_initial_available(1)
		.rate_period(500)
		// now create the client
		.build()
		.unwrap();

	let ksp_items_info: Vec<KospiItemInfo> = client.fetch_kospi_items_info(base_date).await.unwrap();
	let ksd_items_info: Vec<KosdaqItemInfo> = client.fetch_kosdaq_items_info(base_date).await.unwrap();
	let ksp_items_price: Vec<KospiItemPrice> = client.fetch_kospi_items_price(base_date).await.unwrap();
	let ksd_items_price: Vec<KosdaqItemPrice> = client.fetch_kosdaq_items_price(base_date).await.unwrap();
	let etf_items_price: Vec<EtfItemPrice> = client.fetch_etf_items_price(base_date).await.unwrap();
```
