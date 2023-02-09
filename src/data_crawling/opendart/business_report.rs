use crate::utils::FINACOLADA_USER_AGENT;

pub const REPORT_CODE_FIRST_QUARTER: &'static str = "11013";
pub const REPORT_CODE_SEMI_ANNUAL: &'static str = "11012";
pub const REPORT_CODE_THRID_QUARTER: &'static str = "11014";
pub const REPORT_CODE_ANNUAL: &'static str = "11011";

async fn get_business_report(
    query_client: &reqwest::Client,
    api_key: &str,
    corp_code: &str,
    business_year: &str,
    report_code: &str,
) -> anyhow::Result<String> {
    let response = query_client
        .get("https://opendart.fss.or.kr/api/alotMatter.json")
        .query(&[
            ("crtfc_key", api_key),
            ("corp_code", corp_code),
            ("bsns_year", business_year),
            ("reprt_code", report_code),
        ])
        .header("User-Agent", FINACOLADA_USER_AGENT)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}

#[cfg(test)]
mod test {
    use {super::*, insta::assert_snapshot};

    fn api_key() -> Result<String, std::env::VarError> {
        std::env::var("OPENDART_API_KEY")
    }

    #[tokio::test]
    async fn samsung_business_report_2021_first_quarter() {
        let client = reqwest::Client::new();
        let api_key = api_key().unwrap();
        let corp_code = "005930";
        let business_year = "2020";
        let report_code = REPORT_CODE_FIRST_QUARTER;

        let result = get_business_report(&client, &api_key, corp_code, business_year, report_code)
            .await
            .unwrap();

        assert_snapshot!(result, @r###"{"status":"013","message":"조회된 데이타가 없습니다."}"###)
    }
}
