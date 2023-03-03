use crate::utils::FINACOLADA_USER_AGENT;

async fn get_disclosure_by_date_range(
    query_client: &reqwest::Client,
    api_key: &str,
    begin_date: &str,
    end_date: &str,
    max_records: u32,
) -> anyhow::Result<String> {
    let response = query_client
        .get("https://opendart.fss.or.kr/api/list.json")
        .query(&[
            ("crtfc_key", api_key),
            ("bgn_de", begin_date),
            ("end_de", end_date),
            ("page_no", "1"),
            ("page_count", &max_records.to_string()),
        ])
        .header("User-Agent", FINACOLADA_USER_AGENT)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}

async fn get_disclosure_by_corp_code(
    query_client: &reqwest::Client,
    api_key: &str,
    corp_code: &str,
    begin_date: &str,
    end_date: &str,
    max_records: u32,
) -> anyhow::Result<String> {
    let response = query_client
        .get("https://opendart.fss.or.kr/api/list.json")
        .query(&[
            ("crtfc_key", api_key),
            ("corp_code", corp_code),
            ("bgn_de", begin_date),
            ("end_de", end_date),
            ("page_no", "1"),
            ("page_count", &max_records.to_string()),
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

        assert_snapshot!(result, @r###"{"status":"000","message":"정상","page_no":1,"page_count":3,"total_count":15391,"total_page":5131,"list":[{"corp_code":"00324548","corp_name":"한국투자신탁운용","stock_code":"","corp_cls":"E","report_nm":"증권신고서(집합투자증권-신탁형)(한국투자만기가있는채권형ETF증권투자신탁(채권-재간접형))","rcept_no":"20230208000563","flr_nm":"한국투자신탁운용","rcept_dt":"20230208","rm":""},{"corp_code":"01669226","corp_name":"한화위탁관리부동산투자회사","stock_code":"","corp_cls":"E","report_nm":"증권신고서(집합투자증권-회사형)(한화위탁관리부동산투자회사(주))","rcept_no":"20230206000423","flr_nm":"한화위탁관리부동산투자회사","rcept_dt":"20230206","rm":"정"},{"corp_code":"01716481","corp_name":"한국캐피탈제오차유동화전문유한회사","stock_code":"","corp_cls":"E","report_nm":"자산유동화계획의등록신청서(유동화전문회사)","rcept_no":"20230208000195","flr_nm":"한국캐피탈제오차유동화전문유한회사","rcept_dt":"20230208","rm":"정"}]}"###)
    }

    #[tokio::test]
    async fn get_samsung_disclosure_data() {
        let client = reqwest::Client::new();
        let api_key = std::env::var("OPENDART_API_KEY").unwrap();
        let begin_date = "20230101";
        let end_date = "20230208";
        let max_records = 3;
        let corp_code = "005930";

        let result = get_disclosure_by_corp_code(
            &client,
            &api_key,
            corp_code,
            begin_date,
            end_date,
            max_records,
        )
        .await
        .unwrap();

        assert_snapshot!(result, @r###"{"status":"000","message":"정상","page_no":1,"page_count":3,"total_count":15,"total_page":5,"list":[{"corp_code":"00126380","corp_name":"삼성전자","stock_code":"005930","corp_cls":"Y","report_nm":"최대주주등소유주식변동신고서","rcept_no":"20230207800704","flr_nm":"삼성전자","rcept_dt":"20230207","rm":"유"},{"corp_code":"00126380","corp_name":"삼성전자","stock_code":"005930","corp_cls":"Y","report_nm":"주식등의대량보유상황보고서(일반)","rcept_no":"20230203000495","flr_nm":"삼성물산","rcept_dt":"20230203","rm":""},{"corp_code":"00126380","corp_name":"삼성전자","stock_code":"005930","corp_cls":"Y","report_nm":"임원ㆍ주요주주특정증권등소유상황보고서","rcept_no":"20230202000366","flr_nm":"송재혁","rcept_dt":"20230202","rm":""}]}"###)
    }
}
