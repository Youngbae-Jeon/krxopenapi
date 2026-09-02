use ratelimit::Ratelimiter;

pub struct KrxOpenApiClient {
	pub(crate) auth_key: String,
	pub(crate) ratelimiter: Ratelimiter,
	pub(crate) client: reqwest::Client,
}

impl KrxOpenApiClient {
	pub fn new(auth_key: String) -> Self {
		let ratelimiter = Ratelimiter::builder(1)
			.period(std::time::Duration::from_millis(500))
			.initial_available(1)
			.build()
			.unwrap();
		let client = reqwest::Client::new();
		Self { auth_key, ratelimiter, client }
	}
}
