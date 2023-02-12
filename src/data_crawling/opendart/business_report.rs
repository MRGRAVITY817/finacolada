use crate::utils::FINACOLADA_USER_AGENT;

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
    use {
        super::*,
        crate::data_crawling::opendart::{REPORT_CODE_FIRST_QUARTER, REPORT_CODE_SEMI_ANNUAL},
        insta::assert_snapshot,
    };

    fn api_key() -> Result<String, std::env::VarError> {
        std::env::var("OPENDART_API_KEY")
    }

    #[tokio::test]
    async fn samsung_business_report_2020_first_quarter() {
        let client = reqwest::Client::new();
        let api_key = api_key().unwrap();
        let corp_code = "00126380";
        let business_year = "2020";
        let report_code = REPORT_CODE_FIRST_QUARTER;

        let result = get_business_report(&client, &api_key, corp_code, business_year, report_code)
            .await
            .unwrap();

        assert_snapshot!(result, @r###"{"status":"000","message":"정상","list":[{"rcept_no":"20200515001451","corp_cls":"Y","corp_code":"00126380","corp_name":"삼성전자","se":"주당액면가액(원)","thstrm":"100","frmtrm":"100","lwfr":"100"},{"rcept_no":"20200515001451","corp_cls":"Y","corp_code":"00126380","corp_name":"삼성전자","se":"(연결)당기순이익(백만원)","thstrm":"4,889,599","frmtrm":"21,505,054","lwfr":"43,890,877"},{"rcept_no":"20200515001451","corp_cls":"Y","corp_code":"00126380","corp_name":"삼성전자","se":"(별도)당기순이익(백만원)","thstrm":"2,305,931","frmtrm":"15,353,323","lwfr":"32,815,127"},{"rcept_no":"20200515001451","corp_cls":"Y","corp_code":"00126380","corp_name":"삼성전자","se":"(연결)주당순이익(원)","thstrm":"720","frmtrm":"3,166","lwfr":"6,461"},{"rcept_no":"20200515001451","corp_cls":"Y","corp_code":"00126380","corp_name":"삼성전자","se":"현금배당금총액(백만원)","thstrm":"2,404,605","frmtrm":"9,619,243","lwfr":"9,619,243"},{"rcept_no":"20200515001451","corp_cls":"Y","corp_code":"00126380","corp_name":"삼성전자","se":"주식배당금총액(백만원)","thstrm":"-","frmtrm":"-","lwfr":"-"},{"rcept_no":"20200515001451","corp_cls":"Y","corp_code":"00126380","corp_name":"삼성전자","se":"(연결)현금배당성향(%)","thstrm":"49.20","frmtrm":"44.70","lwfr":"21.90"},{"rcept_no":"20200515001451","corp_cls":"Y","corp_code":"00126380","corp_name":"삼성전자","se":"현금배당수익률(%)","stock_knd":"보통주","thstrm":"0.80","frmtrm":"2.60","lwfr":"3.70"},{"rcept_no":"20200515001451","corp_cls":"Y","corp_code":"00126380","corp_name":"삼성전자","se":"현금배당수익률(%)","stock_knd":"우선주","thstrm":"0.90","frmtrm":"3.10","lwfr":"4.50"},{"rcept_no":"20200515001451","corp_cls":"Y","corp_code":"00126380","corp_name":"삼성전자","se":"주식배당수익률(%)","stock_knd":"보통주","thstrm":"-","frmtrm":"-","lwfr":"-"},{"rcept_no":"20200515001451","corp_cls":"Y","corp_code":"00126380","corp_name":"삼성전자","se":"주식배당수익률(%)","stock_knd":"우선주","thstrm":"-","frmtrm":"-","lwfr":"-"},{"rcept_no":"20200515001451","corp_cls":"Y","corp_code":"00126380","corp_name":"삼성전자","se":"주당 현금배당금(원)","stock_knd":"보통주","thstrm":"354","frmtrm":"1,416","lwfr":"1,416"},{"rcept_no":"20200515001451","corp_cls":"Y","corp_code":"00126380","corp_name":"삼성전자","se":"주당 현금배당금(원)","stock_knd":"우선주","thstrm":"354","frmtrm":"1,417","lwfr":"1,417"},{"rcept_no":"20200515001451","corp_cls":"Y","corp_code":"00126380","corp_name":"삼성전자","se":"주당 주식배당(주)","stock_knd":"보통주","thstrm":"-","frmtrm":"-","lwfr":"-"},{"rcept_no":"20200515001451","corp_cls":"Y","corp_code":"00126380","corp_name":"삼성전자","se":"주당 주식배당(주)","stock_knd":"우선주","thstrm":"-","frmtrm":"-","lwfr":"-"}]}"###)
    }

    #[tokio::test]
    async fn samsung_business_report_2020_semi_annual() {
        let client = reqwest::Client::new();
        let api_key = api_key().unwrap();
        let corp_code = "00126380";
        let business_year = "2020";
        let report_code = REPORT_CODE_SEMI_ANNUAL;

        let result = get_business_report(&client, &api_key, corp_code, business_year, report_code)
            .await
            .unwrap();

        assert_snapshot!(result, @r###"{"status":"000","message":"정상","list":[{"rcept_no":"20200814001766","corp_cls":"Y","corp_code":"00126380","corp_name":"삼성전자","se":"주당액면가액(원)","thstrm":"100","frmtrm":"100","lwfr":"100"},{"rcept_no":"20200814001766","corp_cls":"Y","corp_code":"00126380","corp_name":"삼성전자","se":"(연결)당기순이익(백만원)","thstrm":"10,378,563","frmtrm":"21,505,054","lwfr":"43,890,877"},{"rcept_no":"20200814001766","corp_cls":"Y","corp_code":"00126380","corp_name":"삼성전자","se":"(별도)당기순이익(백만원)","thstrm":"5,828,172","frmtrm":"15,353,323","lwfr":"32,815,127"},{"rcept_no":"20200814001766","corp_cls":"Y","corp_code":"00126380","corp_name":"삼성전자","se":"(연결)주당순이익(원)","thstrm":"1,528","frmtrm":"3,166","lwfr":"6,461"},{"rcept_no":"20200814001766","corp_cls":"Y","corp_code":"00126380","corp_name":"삼성전자","se":"현금배당금총액(백만원)","thstrm":"4,809,210","frmtrm":"9,619,243","lwfr":"9,619,243"},{"rcept_no":"20200814001766","corp_cls":"Y","corp_code":"00126380","corp_name":"삼성전자","se":"주식배당금총액(백만원)","thstrm":"-","frmtrm":"-","lwfr":"-"},{"rcept_no":"20200814001766","corp_cls":"Y","corp_code":"00126380","corp_name":"삼성전자","se":"(연결)현금배당성향(%)","thstrm":"46.30","frmtrm":"44.70","lwfr":"21.90"},{"rcept_no":"20200814001766","corp_cls":"Y","corp_code":"00126380","corp_name":"삼성전자","se":"현금배당수익률(%)","stock_knd":"보통주","thstrm":"1.50","frmtrm":"2.60","lwfr":"3.70"},{"rcept_no":"20200814001766","corp_cls":"Y","corp_code":"00126380","corp_name":"삼성전자","se":"현금배당수익률(%)","stock_knd":"우선주","thstrm":"1.80","frmtrm":"3.10","lwfr":"4.50"},{"rcept_no":"20200814001766","corp_cls":"Y","corp_code":"00126380","corp_name":"삼성전자","se":"주식배당수익률(%)","stock_knd":"보통주","thstrm":"-","frmtrm":"-","lwfr":"-"},{"rcept_no":"20200814001766","corp_cls":"Y","corp_code":"00126380","corp_name":"삼성전자","se":"주식배당수익률(%)","stock_knd":"우선주","thstrm":"-","frmtrm":"-","lwfr":"-"},{"rcept_no":"20200814001766","corp_cls":"Y","corp_code":"00126380","corp_name":"삼성전자","se":"주당 현금배당금(원)","stock_knd":"보통주","thstrm":"708","frmtrm":"1,416","lwfr":"1,416"},{"rcept_no":"20200814001766","corp_cls":"Y","corp_code":"00126380","corp_name":"삼성전자","se":"주당 현금배당금(원)","stock_knd":"우선주","thstrm":"708","frmtrm":"1,417","lwfr":"1,417"},{"rcept_no":"20200814001766","corp_cls":"Y","corp_code":"00126380","corp_name":"삼성전자","se":"주당 주식배당(주)","stock_knd":"보통주","thstrm":"-","frmtrm":"-","lwfr":"-"},{"rcept_no":"20200814001766","corp_cls":"Y","corp_code":"00126380","corp_name":"삼성전자","se":"주당 주식배당(주)","stock_knd":"우선주","thstrm":"-","frmtrm":"-","lwfr":"-"}]}"###)
    }
}
