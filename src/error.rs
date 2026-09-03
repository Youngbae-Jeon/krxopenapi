use std::{error::Error, fmt};

#[derive(Debug)]
pub struct KrxOpenApiError {
	pub message: String,
}

impl fmt::Display for KrxOpenApiError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.message)
	}
}

impl Error for KrxOpenApiError {}

impl From<reqwest::Error> for KrxOpenApiError {
	fn from(err: reqwest::Error) -> Self {
		Self {
			message: err.to_string(),
		}
	}
}
