async fn get_disclosure_by_date_range(
    query_client: &reqwest::Client,
    api_key: &str,
    begin_date: &str,
    end_date: &str,
    max_records: u32,
) -> anyhow::Result<String> {
    let response = query_client.get("https://opendart.fss.or.kr/api/list.json")
				.query(&[("crtfc_key", api_key),("bgn_de", begin_date), ("end_de", end_date), ("page_no", "1"), ("page_count", &max_records.to_string())])
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3538.77 Safari/537.36")
				.send().await?.text().await?;

    Ok(response)
}

#[cfg(test)]
mod test {
    use {super::*, insta::assert_snapshot};

    #[tokio::test]
    async fn get_latest_at_most_three_disclosure_json_data() {
        let client = reqwest::Client::new();
        let api_key = std::env::var("OPENDART_API_KEY").unwrap();
        let begin_date = "20230101";
        let end_date = "20230208";
        let max_records = 3;

        let result =
            get_disclosure_by_date_range(&client, &api_key, begin_date, end_date, max_records)
                .await
                .unwrap();

        assert_snapshot!(result, @r###"{"status":"000","message":"정상","page_no":1,"page_count":3,"total_count":15202,"total_page":5068,"list":[{"corp_code":"01343665","corp_name":"RF머트리얼즈","stock_code":"327260","corp_cls":"K","report_nm":"현금ㆍ현물배당결정","rcept_no":"20230208901127","flr_nm":"RF머트리얼즈","rcept_dt":"20230208","rm":"코"},{"corp_code":"01343665","corp_name":"RF머트리얼즈","stock_code":"327260","corp_cls":"K","report_nm":"현금ㆍ현물배당을위한주주명부폐쇄(기준일)결정","rcept_no":"20230208901126","flr_nm":"RF머트리얼즈","rcept_dt":"20230208","rm":"코"},{"corp_code":"01440481","corp_name":"IBKS제13호스팩","stock_code":"351340","corp_cls":"K","report_nm":"주권매매거래정지(상장폐지 사유발생)","rcept_no":"20230208901121","flr_nm":"코스닥시장본부","rcept_dt":"20230208","rm":"코"}]}"###)
    }

    #[tokio::test]
    async fn get_samsung_disclosure_data() {
        let client = reqwest::Client::new();
        let api_key = std::env::var("OPENDART_API_KEY").unwrap();
        let begin_date = "20230101";
        let end_date = "20230208";
        let max_records = 3;
        let ticker = "005930";

        let result =
            get_disclosure_by_ticker(&client, &api_key, ticker, begin_date, end_date, max_records)
                .await
                .unwrap();

        assert_snapshot!(result, @r"")
    }
}
