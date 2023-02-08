#[cfg(test)]
mod test {
    use {super::*, insta::assert_snapshot};

    #[tokio::test]
    async fn get_latest_at_most_hundred_disclosure_json_data() {
        let client = reqwest::Client::new();
        let begin_date = "20230101";
        let end_date = "20230208";
        let max_records = 100;

        let result = get_disclosure_by_date_range(&client, begin_date, end_date, max_records)
            .await
            .unwrap();

        assert_snapshot!(result, @"")
    }
}
