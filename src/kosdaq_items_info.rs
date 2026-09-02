use std::time::Duration;

use chrono::NaiveDate;
use ratelimit::TryWaitError;
use serde::Deserialize;

use crate::KrxOpenApiClient;

#[derive(Debug, Deserialize)]
pub struct KosdaqItemInfo {
	/// 표준코드
	#[serde(rename = "ISU_CD")]
	pub isu_cd: String,

	/// 단축코드
	#[serde(rename = "ISU_SRT_CD")]
	pub isu_srt_cd: String,

	/// 한글 종목명
	#[serde(rename = "ISU_NM")]
	pub isu_nm: String,

	/// 한글 종목약명
	#[serde(rename = "ISU_ABBRV")]
	pub isu_abbrv: String,

	/// 영문 종목명
	#[serde(rename = "ISU_ENG_NM")]
	pub isu_eng_nm: String,

	/// 상장일
	#[serde(rename = "LIST_DD")]
	pub list_dd: String,

	/// 시장구분
	#[serde(rename = "MKT_TP_NM")]
	pub mkt_tp_nm: String,

	/// 증권구분
	#[serde(rename = "SECUGRP_NM")]
	pub secugrp_nm: String,

	/// 소속부
	#[serde(rename = "SECT_TP_NM")]
	pub sect_tp_nm: String,

	/// 주식종류
	#[serde(rename = "KIND_STKCERT_TP_NM")]
	pub kind_stkcert_tp_nm: String,

	/// 액면가
	#[serde(rename = "PARVAL")]
	pub parval: String,

	/// 상장주식수
	#[serde(rename = "LIST_SHRS")]
	pub list_shrs: String,
}

#[derive(Deserialize)]
struct ResponsePayload {
	#[serde(rename = "OutBlock_1")]
	list: Vec<KosdaqItemInfo>,
}

const URL: &str = "https://data-dbg.krx.co.kr/svc/apis/sto/ksq_isu_base_info";

impl KrxOpenApiClient {
	pub async fn fetch_kosdaq_items_info(&self, base_date: NaiveDate) -> Result<Vec<KosdaqItemInfo>, Box<dyn std::error::Error>> {
		while let Err(TryWaitError::Insufficient(dur)) = self.ratelimiter.try_wait() {
			tokio::time::sleep(dur).await;
		}

		let params = [
			("basDd", base_date.format("%Y%m%d").to_string()),
		];
		let url = reqwest::Url::parse_with_params(URL, &params)?;
		log::debug!("Fetching KospiItemsInfo from {}", url);

		let resp = self.client.get(url)
			.header("AUTH_KEY", &self.auth_key)
			.timeout(Duration::from_secs(5))
			.send()
			.await?
			.error_for_status()?
			.json::<ResponsePayload>()
			.await?;
		Ok(resp.list)
	}
}
