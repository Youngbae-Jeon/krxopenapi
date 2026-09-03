use std::time::Duration;

use chrono::NaiveDate;
use ratelimit::TryWaitError;
use serde::Deserialize;

use crate::{KrxOpenApiClient, KrxOpenApiError};

#[derive(Debug, Deserialize)]
pub struct KosdaqItemPrice {
	/// 기준일자
	#[serde(rename = "BAS_DD")]
	pub bas_dd: String,

	/// 종목코드
	#[serde(rename = "ISU_CD")]
	pub isu_cd: String,

	/// 종목명
	#[serde(rename = "ISU_NM")]
	pub isu_nm: String,

	/// 시장구분
	#[serde(rename = "MKT_NM")]
	pub mkt_nm: String,

	/// 소속부
	#[serde(rename = "SECT_TP_NM")]
	pub sect_tp_nm: String,

	/// 종가
	#[serde(rename = "TDD_CLSPRC")]
	pub tdd_clsprc: String,

	/// 대비
	#[serde(rename = "CMPPREVDD_PRC")]
	pub cmpprevdd_prc: String,

	/// 등락률
	#[serde(rename = "FLUC_RT")]
	pub fluc_rt: String,

	/// 시가
	#[serde(rename = "TDD_OPNPRC")]
	pub tdd_opnprc: String,

	/// 고가
	#[serde(rename = "TDD_HGPRC")]
	pub tdd_hgprc: String,

	/// 저가
	#[serde(rename = "TDD_LWPRC")]
	pub tdd_lwprc: String,

	/// 거래량
	#[serde(rename = "ACC_TRDVOL")]
	pub acc_trdvol: String,

	/// 거래대금
	#[serde(rename = "ACC_TRDVAL")]
	pub acc_trdval: String,

	/// 시가총액
	#[serde(rename = "MKTCAP")]
	pub mktcap: String,

	/// 상장주식수
	#[serde(rename = "LIST_SHRS")]
	pub list_shrs: String,
}

#[derive(Deserialize)]
struct ResponsePayload {
	#[serde(rename = "OutBlock_1")]
	list: Vec<KosdaqItemPrice>,
}

const URL: &str = "https://data-dbg.krx.co.kr/svc/apis/sto/ksq_bydd_trd";

impl KrxOpenApiClient {
	pub async fn fetch_kosdaq_items_price(&self, base_date: NaiveDate) -> Result<Vec<KosdaqItemPrice>, KrxOpenApiError> {
		while let Err(TryWaitError::Insufficient(dur)) = self.ratelimiter.try_wait() {
			tokio::time::sleep(dur).await;
		}

		let params = [
			("basDd", base_date.format("%Y%m%d").to_string()),
		];
		let url = reqwest::Url::parse_with_params(URL, &params)
			.map_err(|e| KrxOpenApiError { message: e.to_string() })?;
		log::debug!("Fetching KosdaqItemsPrice from {}", url);

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
