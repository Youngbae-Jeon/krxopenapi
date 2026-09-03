use ratelimit::Ratelimiter;

use crate::KrxOpenApiError;

pub struct Builder {
	_auth_key: String,
	_rate: u64,
	_rate_initial_available: u64,
	_rate_period: std::time::Duration,
}
impl Builder {
	pub fn auth_key(mut self, auth_key: String) -> Self {
		self._auth_key = auth_key;
		self
	}
	pub fn rate(mut self, rate: u64) -> Self {
		self._rate = rate;
		self
	}
	pub fn rate_initial_available(mut self, rate_initial_available: u64) -> Self {
		self._rate_initial_available = rate_initial_available;
		self
	}
	pub fn rate_period(mut self, ratelimit_period: std::time::Duration) -> Self {
		self._rate_period = ratelimit_period;
		self
	}
	pub fn build(self) -> Result<KrxOpenApiClient, KrxOpenApiError> {
		let ratelimiter = Ratelimiter::builder(1)
			.period(self._rate_period)
			.initial_available(1)
			.build()
			.map_err(|e| KrxOpenApiError { message: e.to_string() })?;
		let reqwest_client = reqwest::Client::new();
		let client = KrxOpenApiClient { auth_key: self._auth_key, ratelimiter, client: reqwest_client };
		Ok(client)
	}
}
impl Default for Builder {
	fn default() -> Self {
		Self {
			_auth_key: String::new(),
			_rate: 1,
			_rate_initial_available: 1,
			_rate_period: std::time::Duration::from_millis(500),
		}
	}
}


pub struct KrxOpenApiClient {
	pub(crate) auth_key: String,
	pub(crate) ratelimiter: Ratelimiter,
	pub(crate) client: reqwest::Client,
}

impl KrxOpenApiClient {
	pub fn builder() -> Builder {
		Builder::default()
	}
}
