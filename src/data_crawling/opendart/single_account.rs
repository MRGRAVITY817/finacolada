#[cfg(test)]
mod test {
    use crate::data_crawling::opendart::business_report::REPORT_CODE_FIRST_QUARTER;
    use insta::assert_snapshot;

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

        assert_snapshot!(result, @"");
    }
}
