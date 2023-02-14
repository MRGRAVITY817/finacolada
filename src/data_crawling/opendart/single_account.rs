use crate::utils::FINACOLADA_USER_AGENT;

async fn get_single_account_info(
    query_client: &reqwest::Client,
    api_key: &str,
    corp_code: &str,
    business_year: &str,
    report_code: &str,
) -> anyhow::Result<String> {
    let response = query_client
        .get("https://opendart.fss.or.kr/api/fnlttSinglAcnt.json")
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
    use {
        super::*, crate::data_crawling::opendart::REPORT_CODE_FIRST_QUARTER, insta::assert_snapshot,
    };

    fn api_key() -> Result<String, std::env::VarError> {
        std::env::var("OPENDART_API_KEY")
    }

    #[tokio::test]
    async fn get_samsung_account_info() {
        let client = reqwest::Client::new();
        let api_key = api_key().unwrap();
        let corp_code = "00126380";
        let business_year = "2020";
        let report_code = REPORT_CODE_FIRST_QUARTER;

        let result =
            get_single_account_info(&client, &api_key, corp_code, business_year, report_code)
                .await
                .unwrap();

        assert_snapshot!(result, @r###"{"status":"000","message":"정상","list":[{"rcept_no":"20200515001451","reprt_code":"11013","bsns_year":"2020","corp_code":"00126380","stock_code":"005930","fs_div":"CFS","fs_nm":"연결재무제표","sj_div":"BS","sj_nm":"재무상태표","account_nm":"유동자산","thstrm_nm":"제 52 기1분기말","thstrm_dt":"2020.03.31 현재","thstrm_amount":"186,739,748,000,000","frmtrm_nm":"제 51 기말","frmtrm_dt":"2019.12.31 현재","frmtrm_amount":"181,385,260,000,000","ord":"1","currency":"KRW"},{"rcept_no":"20200515001451","reprt_code":"11013","bsns_year":"2020","corp_code":"00126380","stock_code":"005930","fs_div":"CFS","fs_nm":"연결재무제표","sj_div":"BS","sj_nm":"재무상태표","account_nm":"비유동자산","thstrm_nm":"제 52 기1분기말","thstrm_dt":"2020.03.31 현재","thstrm_amount":"170,717,787,000,000","frmtrm_nm":"제 51 기말","frmtrm_dt":"2019.12.31 현재","frmtrm_amount":"171,179,237,000,000","ord":"3","currency":"KRW"},{"rcept_no":"20200515001451","reprt_code":"11013","bsns_year":"2020","corp_code":"00126380","stock_code":"005930","fs_div":"CFS","fs_nm":"연결재무제표","sj_div":"BS","sj_nm":"재무상태표","account_nm":"자산총계","thstrm_nm":"제 52 기1분기말","thstrm_dt":"2020.03.31 현재","thstrm_amount":"357,457,535,000,000","frmtrm_nm":"제 51 기말","frmtrm_dt":"2019.12.31 현재","frmtrm_amount":"352,564,497,000,000","ord":"5","currency":"KRW"},{"rcept_no":"20200515001451","reprt_code":"11013","bsns_year":"2020","corp_code":"00126380","stock_code":"005930","fs_div":"CFS","fs_nm":"연결재무제표","sj_div":"BS","sj_nm":"재무상태표","account_nm":"유동부채","thstrm_nm":"제 52 기1분기말","thstrm_dt":"2020.03.31 현재","thstrm_amount":"64,763,290,000,000","frmtrm_nm":"제 51 기말","frmtrm_dt":"2019.12.31 현재","frmtrm_amount":"63,782,764,000,000","ord":"7","currency":"KRW"},{"rcept_no":"20200515001451","reprt_code":"11013","bsns_year":"2020","corp_code":"00126380","stock_code":"005930","fs_div":"CFS","fs_nm":"연결재무제표","sj_div":"BS","sj_nm":"재무상태표","account_nm":"비유동부채","thstrm_nm":"제 52 기1분기말","thstrm_dt":"2020.03.31 현재","thstrm_amount":"26,306,522,000,000","frmtrm_nm":"제 51 기말","frmtrm_dt":"2019.12.31 현재","frmtrm_amount":"25,901,312,000,000","ord":"9","currency":"KRW"},{"rcept_no":"20200515001451","reprt_code":"11013","bsns_year":"2020","corp_code":"00126380","stock_code":"005930","fs_div":"CFS","fs_nm":"연결재무제표","sj_div":"BS","sj_nm":"재무상태표","account_nm":"부채총계","thstrm_nm":"제 52 기1분기말","thstrm_dt":"2020.03.31 현재","thstrm_amount":"91,069,812,000,000","frmtrm_nm":"제 51 기말","frmtrm_dt":"2019.12.31 현재","frmtrm_amount":"89,684,076,000,000","ord":"11","currency":"KRW"},{"rcept_no":"20200515001451","reprt_code":"11013","bsns_year":"2020","corp_code":"00126380","stock_code":"005930","fs_div":"CFS","fs_nm":"연결재무제표","sj_div":"BS","sj_nm":"재무상태표","account_nm":"자본금","thstrm_nm":"제 52 기1분기말","thstrm_dt":"2020.03.31 현재","thstrm_amount":"897,514,000,000","frmtrm_nm":"제 51 기말","frmtrm_dt":"2019.12.31 현재","frmtrm_amount":"897,514,000,000","ord":"13","currency":"KRW"},{"rcept_no":"20200515001451","reprt_code":"11013","bsns_year":"2020","corp_code":"00126380","stock_code":"005930","fs_div":"CFS","fs_nm":"연결재무제표","sj_div":"BS","sj_nm":"재무상태표","account_nm":"이익잉여금","thstrm_nm":"제 52 기1분기말","thstrm_dt":"2020.03.31 현재","thstrm_amount":"257,078,919,000,000","frmtrm_nm":"제 51 기말","frmtrm_dt":"2019.12.31 현재","frmtrm_amount":"254,582,894,000,000","ord":"17","currency":"KRW"},{"rcept_no":"20200515001451","reprt_code":"11013","bsns_year":"2020","corp_code":"00126380","stock_code":"005930","fs_div":"CFS","fs_nm":"연결재무제표","sj_div":"BS","sj_nm":"재무상태표","account_nm":"자본총계","thstrm_nm":"제 52 기1분기말","thstrm_dt":"2020.03.31 현재","thstrm_amount":"266,387,723,000,000","frmtrm_nm":"제 51 기말","frmtrm_dt":"2019.12.31 현재","frmtrm_amount":"262,880,421,000,000","ord":"21","currency":"KRW"},{"rcept_no":"20200515001451","reprt_code":"11013","bsns_year":"2020","corp_code":"00126380","stock_code":"005930","fs_div":"CFS","fs_nm":"연결재무제표","sj_div":"IS","sj_nm":"손익계산서","account_nm":"매출액","thstrm_nm":"제 52 기1분기","thstrm_dt":"2020.01.01 ~ 2020.03.31","thstrm_amount":"55,325,178,000,000","thstrm_add_amount":"55,325,178,000,000","frmtrm_nm":"제 51 기1분기","frmtrm_dt":"2019.01.01 ~ 2019.03.31","frmtrm_add_amount":"52,385,546,000,000","frmtrm_amount":"52,385,546,000,000","ord":"23","currency":"KRW"},{"rcept_no":"20200515001451","reprt_code":"11013","bsns_year":"2020","corp_code":"00126380","stock_code":"005930","fs_div":"CFS","fs_nm":"연결재무제표","sj_div":"IS","sj_nm":"손익계산서","account_nm":"영업이익","thstrm_nm":"제 52 기1분기","thstrm_dt":"2020.01.01 ~ 2020.03.31","thstrm_amount":"6,447,345,000,000","thstrm_add_amount":"6,447,345,000,000","frmtrm_nm":"제 51 기1분기","frmtrm_dt":"2019.01.01 ~ 2019.03.31","frmtrm_add_amount":"6,233,282,000,000","frmtrm_amount":"6,233,282,000,000","ord":"25","currency":"KRW"},{"rcept_no":"20200515001451","reprt_code":"11013","bsns_year":"2020","corp_code":"00126380","stock_code":"005930","fs_div":"CFS","fs_nm":"연결재무제표","sj_div":"IS","sj_nm":"손익계산서","account_nm":"법인세차감전 순이익","thstrm_nm":"제 52 기1분기","thstrm_dt":"2020.01.01 ~ 2020.03.31","thstrm_amount":"6,756,852,000,000","thstrm_add_amount":"6,756,852,000,000","frmtrm_nm":"제 51 기1분기","frmtrm_dt":"2019.01.01 ~ 2019.03.31","frmtrm_add_amount":"6,912,978,000,000","frmtrm_amount":"6,912,978,000,000","ord":"27","currency":"KRW"},{"rcept_no":"20200515001451","reprt_code":"11013","bsns_year":"2020","corp_code":"00126380","stock_code":"005930","fs_div":"CFS","fs_nm":"연결재무제표","sj_div":"IS","sj_nm":"손익계산서","account_nm":"당기순이익","thstrm_nm":"제 52 기1분기","thstrm_dt":"2020.01.01 ~ 2020.03.31","thstrm_amount":"4,884,926,000,000","thstrm_add_amount":"4,884,926,000,000","frmtrm_nm":"제 51 기1분기","frmtrm_dt":"2019.01.01 ~ 2019.03.31","frmtrm_add_amount":"5,043,585,000,000","frmtrm_amount":"5,043,585,000,000","ord":"29","currency":"KRW"},{"rcept_no":"20200515001451","reprt_code":"11013","bsns_year":"2020","corp_code":"00126380","stock_code":"005930","fs_div":"OFS","fs_nm":"재무제표","sj_div":"BS","sj_nm":"재무상태표","account_nm":"유동자산","thstrm_nm":"제 52 기1분기말","thstrm_dt":"2020.03.31 현재","thstrm_amount":"76,640,880,000,000","frmtrm_nm":"제 51 기말","frmtrm_dt":"2019.12.31 현재","frmtrm_amount":"72,659,080,000,000","ord":"2","currency":"KRW"},{"rcept_no":"20200515001451","reprt_code":"11013","bsns_year":"2020","corp_code":"00126380","stock_code":"005930","fs_div":"OFS","fs_nm":"재무제표","sj_div":"BS","sj_nm":"재무상태표","account_nm":"비유동자산","thstrm_nm":"제 52 기1분기말","thstrm_dt":"2020.03.31 현재","thstrm_amount":"143,931,511,000,000","frmtrm_nm":"제 51 기말","frmtrm_dt":"2019.12.31 현재","frmtrm_amount":"143,521,840,000,000","ord":"4","currency":"KRW"},{"rcept_no":"20200515001451","reprt_code":"11013","bsns_year":"2020","corp_code":"00126380","stock_code":"005930","fs_div":"OFS","fs_nm":"재무제표","sj_div":"BS","sj_nm":"재무상태표","account_nm":"자산총계","thstrm_nm":"제 52 기1분기말","thstrm_dt":"2020.03.31 현재","thstrm_amount":"220,572,391,000,000","frmtrm_nm":"제 51 기말","frmtrm_dt":"2019.12.31 현재","frmtrm_amount":"216,180,920,000,000","ord":"6","currency":"KRW"},{"rcept_no":"20200515001451","reprt_code":"11013","bsns_year":"2020","corp_code":"00126380","stock_code":"005930","fs_div":"OFS","fs_nm":"재무제표","sj_div":"BS","sj_nm":"재무상태표","account_nm":"유동부채","thstrm_nm":"제 52 기1분기말","thstrm_dt":"2020.03.31 현재","thstrm_amount":"41,130,269,000,000","frmtrm_nm":"제 51 기말","frmtrm_dt":"2019.12.31 현재","frmtrm_amount":"36,237,164,000,000","ord":"8","currency":"KRW"},{"rcept_no":"20200515001451","reprt_code":"11013","bsns_year":"2020","corp_code":"00126380","stock_code":"005930","fs_div":"OFS","fs_nm":"재무제표","sj_div":"BS","sj_nm":"재무상태표","account_nm":"비유동부채","thstrm_nm":"제 52 기1분기말","thstrm_dt":"2020.03.31 현재","thstrm_amount":"2,013,369,000,000","frmtrm_nm":"제 51 기말","frmtrm_dt":"2019.12.31 현재","frmtrm_amount":"2,073,509,000,000","ord":"10","currency":"KRW"},{"rcept_no":"20200515001451","reprt_code":"11013","bsns_year":"2020","corp_code":"00126380","stock_code":"005930","fs_div":"OFS","fs_nm":"재무제표","sj_div":"BS","sj_nm":"재무상태표","account_nm":"부채총계","thstrm_nm":"제 52 기1분기말","thstrm_dt":"2020.03.31 현재","thstrm_amount":"43,143,638,000,000","frmtrm_nm":"제 51 기말","frmtrm_dt":"2019.12.31 현재","frmtrm_amount":"38,310,673,000,000","ord":"12","currency":"KRW"},{"rcept_no":"20200515001451","reprt_code":"11013","bsns_year":"2020","corp_code":"00126380","stock_code":"005930","fs_div":"OFS","fs_nm":"재무제표","sj_div":"BS","sj_nm":"재무상태표","account_nm":"자본금","thstrm_nm":"제 52 기1분기말","thstrm_dt":"2020.03.31 현재","thstrm_amount":"897,514,000,000","frmtrm_nm":"제 51 기말","frmtrm_dt":"2019.12.31 현재","frmtrm_amount":"897,514,000,000","ord":"15","currency":"KRW"},{"rcept_no":"20200515001451","reprt_code":"11013","bsns_year":"2020","corp_code":"00126380","stock_code":"005930","fs_div":"OFS","fs_nm":"재무제표","sj_div":"BS","sj_nm":"재무상태표","account_nm":"이익잉여금","thstrm_nm":"제 52 기1분기말","thstrm_dt":"2020.03.31 현재","thstrm_amount":"172,188,829,000,000","frmtrm_nm":"제 51 기말","frmtrm_dt":"2019.12.31 현재","frmtrm_amount":"172,288,326,000,000","ord":"19","currency":"KRW"},{"rcept_no":"20200515001451","reprt_code":"11013","bsns_year":"2020","corp_code":"00126380","stock_code":"005930","fs_div":"OFS","fs_nm":"재무제표","sj_div":"BS","sj_nm":"재무상태표","account_nm":"자본총계","thstrm_nm":"제 52 기1분기말","thstrm_dt":"2020.03.31 현재","thstrm_amount":"177,428,753,000,000","frmtrm_nm":"제 51 기말","frmtrm_dt":"2019.12.31 현재","frmtrm_amount":"177,870,247,000,000","ord":"22","currency":"KRW"},{"rcept_no":"20200515001451","reprt_code":"11013","bsns_year":"2020","corp_code":"00126380","stock_code":"005930","fs_div":"OFS","fs_nm":"재무제표","sj_div":"IS","sj_nm":"손익계산서","account_nm":"매출액","thstrm_nm":"제 52 기1분기","thstrm_dt":"2020.01.01 ~ 2020.03.31","thstrm_amount":"40,087,939,000,000","thstrm_add_amount":"40,087,939,000,000","frmtrm_nm":"제 51 기1분기","frmtrm_dt":"2019.01.01 ~ 2019.03.31","frmtrm_add_amount":"37,038,395,000,000","frmtrm_amount":"37,038,395,000,000","ord":"24","currency":"KRW"},{"rcept_no":"20200515001451","reprt_code":"11013","bsns_year":"2020","corp_code":"00126380","stock_code":"005930","fs_div":"OFS","fs_nm":"재무제표","sj_div":"IS","sj_nm":"손익계산서","account_nm":"영업이익","thstrm_nm":"제 52 기1분기","thstrm_dt":"2020.01.01 ~ 2020.03.31","thstrm_amount":"3,032,343,000,000","thstrm_add_amount":"3,032,343,000,000","frmtrm_nm":"제 51 기1분기","frmtrm_dt":"2019.01.01 ~ 2019.03.31","frmtrm_add_amount":"3,896,923,000,000","frmtrm_amount":"3,896,923,000,000","ord":"26","currency":"KRW"},{"rcept_no":"20200515001451","reprt_code":"11013","bsns_year":"2020","corp_code":"00126380","stock_code":"005930","fs_div":"OFS","fs_nm":"재무제표","sj_div":"IS","sj_nm":"손익계산서","account_nm":"법인세차감전 순이익","thstrm_nm":"제 52 기1분기","thstrm_dt":"2020.01.01 ~ 2020.03.31","thstrm_amount":"3,087,628,000,000","thstrm_add_amount":"3,087,628,000,000","frmtrm_nm":"제 51 기1분기","frmtrm_dt":"2019.01.01 ~ 2019.03.31","frmtrm_add_amount":"4,154,401,000,000","frmtrm_amount":"4,154,401,000,000","ord":"28","currency":"KRW"},{"rcept_no":"20200515001451","reprt_code":"11013","bsns_year":"2020","corp_code":"00126380","stock_code":"005930","fs_div":"OFS","fs_nm":"재무제표","sj_div":"IS","sj_nm":"손익계산서","account_nm":"당기순이익","thstrm_nm":"제 52 기1분기","thstrm_dt":"2020.01.01 ~ 2020.03.31","thstrm_amount":"2,305,931,000,000","thstrm_add_amount":"2,305,931,000,000","frmtrm_nm":"제 51 기1분기","frmtrm_dt":"2019.01.01 ~ 2019.03.31","frmtrm_add_amount":"3,088,628,000,000","frmtrm_amount":"3,088,628,000,000","ord":"30","currency":"KRW"}]}"###);
    }
}